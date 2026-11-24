// Piano Logger 9.2.0: reading the browser's return trip of a sign-in (Google, through the cloud sync service).
// Pure functions only (no Tauri): lib.rs listens on 127.0.0.1 and hands each request here, so these rules can be tested alone.

// Only plain token characters are ever passed on to the page, so nothing from the address can become script.
pub fn clean(s: &str) -> bool {
    !s.is_empty() && s.len() <= 512 && s.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~'))
}

fn param(q: &str, key: &str) -> Option<String> {
    q.split('&').filter_map(|kv| kv.split_once('=')).find(|(k, _)| *k == key).map(|(_, v)| v.to_string())
}

// What to do with one request that reached the listener:
// (status line, text for the browser tab, the answer for the app if this was the real callback)
pub fn handle(req: &str) -> (&'static str, &'static str, Option<serde_json::Value>) {
    let line = req.lines().next().unwrap_or("");
    let mut it = line.split_whitespace();
    let (method, target) = (it.next().unwrap_or(""), it.next().unwrap_or(""));
    let (path, query) = target.split_once('?').unwrap_or((target, ""));
    if method != "GET" || path != "/cb" {
        return ("404 Not Found", "Not found.", None);
    }
    let state = param(query, "pl_state");
    let ok_state = state.as_deref().map(clean).unwrap_or(false);
    if let Some(code) = param(query, "code") {
        if clean(&code) && ok_state {
            return (
                "200 OK",
                "Piano Logger: you are signed in. You can close this tab and go back to the app.",
                Some(serde_json::json!({ "code": code, "state": state.unwrap() })),
            );
        }
        return (
            "400 Bad Request",
            "Piano Logger could not read the sign-in answer. Please close this tab and try again in the app.",
            Some(serde_json::json!({ "error": "The sign-in answer was not in the expected form" })),
        );
    }
    if param(query, "error").is_some() {
        return (
            "200 OK",
            "Sign-in was not completed. You can close this tab and go back to the app.",
            Some(serde_json::json!({ "error": "Sign-in was cancelled or refused" })),
        );
    }
    (
        "400 Bad Request",
        "Piano Logger could not read the sign-in answer. Please close this tab and try again in the app.",
        Some(serde_json::json!({ "error": "The sign-in answer was empty" })),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_callback_is_passed_on() {
        let (st, _, a) = handle("GET /cb?code=abc-123_x.y~z&pl_state=Zm9v-_ HTTP/1.1\r\nHost: 127.0.0.1\r\n\r\n");
        assert_eq!(st, "200 OK");
        let a = a.unwrap();
        assert_eq!(a["code"], "abc-123_x.y~z");
        assert_eq!(a["state"], "Zm9v-_");
    }

    #[test]
    fn other_paths_and_methods_are_ignored() {
        assert!(handle("GET /favicon.ico HTTP/1.1\r\n\r\n").2.is_none());
        assert!(handle("POST /cb?code=a&pl_state=b HTTP/1.1\r\n\r\n").2.is_none());
        assert!(handle("").2.is_none());
        assert!(handle("GET /cb/../secret HTTP/1.1").2.is_none());
    }

    #[test]
    fn script_or_odd_characters_never_reach_the_page() {
        for bad in [
            "GET /cb?code=a%22);alert(1)//&pl_state=b HTTP/1.1",
            "GET /cb?code=a'b&pl_state=b HTTP/1.1",
            "GET /cb?code=a b&pl_state=b HTTP/1.1",
            "GET /cb?code=a<script>&pl_state=b HTTP/1.1",
            "GET /cb?code=abc&pl_state=x%22y HTTP/1.1",
            "GET /cb?code=abc HTTP/1.1",
            "GET /cb?code=a\"b&pl_state=b HTTP/1.1",
            "GET /cb?code=a\\b&pl_state=b HTTP/1.1",
        ] {
            let (st, _, a) = handle(bad);
            let a = a.expect("an error answer");
            assert_eq!(st, "400 Bad Request", "{bad}");
            assert!(a.get("code").is_none(), "{bad}");
            assert!(a.get("state").is_none(), "{bad}");
        }
    }

    #[test]
    fn provider_errors_and_empty_answers() {
        assert_eq!(handle("GET /cb?error=access_denied&pl_state=b HTTP/1.1").2.unwrap()["error"], "Sign-in was cancelled or refused");
        assert!(handle("GET /cb HTTP/1.1").2.unwrap().get("code").is_none());
        let long = format!("GET /cb?code={}&pl_state=b HTTP/1.1", "a".repeat(600));
        assert!(handle(&long).2.unwrap().get("code").is_none());
        let long_state = format!("GET /cb?code=abc&pl_state={} HTTP/1.1", "a".repeat(600));
        assert!(handle(&long_state).2.unwrap().get("code").is_none());
    }

    #[test]
    fn answer_is_valid_json_the_page_can_read() {
        let (_, _, a) = handle("GET /cb?code=A.b-c_d~e&pl_state=s-t_u HTTP/1.1");
        let text = a.unwrap().to_string();
        let back: serde_json::Value = serde_json::from_str(&text).unwrap();
        assert_eq!(back["code"], "A.b-c_d~e");
    }
}
