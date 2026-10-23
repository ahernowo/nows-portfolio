//! GitHub OAuth helpers shared by both workers.
//!
//! The workers do the network calls; this module builds URLs, cookies and
//! the small HTML page that hands a token back to Decap CMS. Keeping these
//! here means the security-sensitive parts have unit tests.

use url::Url;

pub const GITHUB_AUTHORIZE: &str = "https://github.com/login/oauth/authorize";
pub const GITHUB_TOKEN: &str = "https://github.com/login/oauth/access_token";
pub const GITHUB_API: &str = "https://api.github.com";

/// Build the GitHub authorize URL the browser is sent to.
pub fn authorize_url(client_id: &str, redirect_uri: &str, scope: &str, state: &str) -> String {
    let mut url = Url::parse(GITHUB_AUTHORIZE).expect("constant URL is valid");
    url.query_pairs_mut()
        .append_pair("client_id", client_id)
        .append_pair("redirect_uri", redirect_uri)
        .append_pair("scope", scope)
        .append_pair("state", state)
        .append_pair("allow_signup", "false");
    url.into()
}

/// Form body for exchanging the one-time `code` for an access token.
pub fn token_request_body(
    client_id: &str,
    client_secret: &str,
    code: &str,
    redirect_uri: &str,
) -> String {
    url::form_urlencoded::Serializer::new(String::new())
        .append_pair("client_id", client_id)
        .append_pair("client_secret", client_secret)
        .append_pair("code", code)
        .append_pair("redirect_uri", redirect_uri)
        .finish()
}

/// Read one cookie from a `Cookie:` header.
pub fn cookie<'a>(header: &'a str, name: &str) -> Option<&'a str> {
    header.split(';').find_map(|pair| {
        let (k, v) = pair.trim().split_once('=')?;
        (k == name).then_some(v)
    })
}

/// Options for [`set_cookie`].
pub struct CookieOptions<'a> {
    pub path: &'a str,
    pub max_age: u64,
    /// `Lax` lets the cookie survive the redirect back from github.com.
    /// `Strict` is right for the admin session itself.
    pub same_site: &'a str,
}

/// A `Set-Cookie` value. Always `HttpOnly` and `Secure`, so page scripts
/// cannot read it and it never travels over plain HTTP.
pub fn set_cookie(name: &str, value: &str, opts: &CookieOptions<'_>) -> String {
    format!(
        "{name}={value}; Path={}; Max-Age={}; HttpOnly; Secure; SameSite={}",
        opts.path, opts.max_age, opts.same_site
    )
}

/// A `Set-Cookie` value that deletes the cookie.
pub fn clear_cookie(name: &str, path: &str) -> String {
    format!("{name}=; Path={path}; Max-Age=0; HttpOnly; Secure; SameSite=Lax")
}

/// Compare two secrets without leaking, through timing, how many leading
/// bytes matched.
pub fn constant_time_eq(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

/// Hex-encode random bytes into a state or nonce string.
pub fn to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Is this GitHub login on the comma-separated allowlist? Case-insensitive,
/// because GitHub logins are.
pub fn is_allowed_login(login: &str, allowlist: &str) -> bool {
    let login = login.trim();
    !login.is_empty()
        && allowlist
            .split(',')
            .map(str::trim)
            .any(|allowed| !allowed.is_empty() && allowed.eq_ignore_ascii_case(login))
}

/// Is `origin` a plain `https://host[:port]` origin? We refuse to post a
/// token to anything vaguer, such as `*`.
pub fn is_exact_origin(origin: &str) -> bool {
    match Url::parse(origin) {
        Ok(u) => {
            (u.scheme() == "https" || (u.scheme() == "http" && u.host_str() == Some("localhost")))
                && u.path() == "/"
                && u.query().is_none()
                && u.fragment().is_none()
                && !origin.ends_with('/')
        }
        Err(_) => false,
    }
}

/// Outcome to report back to Decap CMS.
pub enum DecapResult<'a> {
    Success { token: &'a str },
    Error { message: &'a str },
}

/// The message string Decap CMS listens for.
pub fn decap_message(result: &DecapResult<'_>) -> String {
    match result {
        DecapResult::Success { token } => format!(
            "authorization:github:success:{}",
            serde_json::json!({ "token": token, "provider": "github" })
        ),
        DecapResult::Error { message } => format!(
            "authorization:github:error:{}",
            serde_json::json!({ "message": message, "provider": "github" })
        ),
    }
}

/// Encode a string as a JavaScript string literal that is also safe inside
/// an HTML `<script>` element.
pub fn js_string(s: &str) -> String {
    serde_json::Value::String(s.to_string())
        .to_string()
        .replace('<', "\\u003c")
        .replace('>', "\\u003e")
        .replace('&', "\\u0026")
        .replace('\u{2028}', "\\u2028")
        .replace('\u{2029}', "\\u2029")
}

/// The popup page that completes the Decap CMS login.
///
/// The handshake: the popup tells its opener "authorizing:github"; Decap
/// answers; the popup checks the answer came from `allowed_origin` and only
/// then posts the token to that exact origin. A page on any other origin
/// that opens this popup gets nothing.
pub fn decap_callback_html(message: &str, allowed_origin: &str, nonce: &str) -> String {
    format!(
        r#"<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><meta name="robots" content="noindex"><title>Signing in</title></head>
<body>
<p>Finishing sign-in. This window closes by itself.</p>
<script nonce="{nonce}">
(function () {{
  var allowed = {origin};
  var message = {message};
  if (!window.opener) {{
    document.body.textContent = "Open the CMS and sign in from there.";
    return;
  }}
  function receive(event) {{
    if (event.origin !== allowed) return;
    window.removeEventListener("message", receive);
    window.opener.postMessage(message, allowed);
    window.close();
  }}
  window.addEventListener("message", receive);
  window.opener.postMessage("authorizing:github", allowed);
}})();
</script>
</body>
</html>
"#,
        nonce = nonce,
        origin = js_string(allowed_origin),
        message = js_string(message),
    )
}

/// Security headers for the popup page. The nonce lets exactly our one
/// inline script run and nothing else.
pub fn popup_csp(nonce: &str) -> String {
    format!("default-src 'none'; script-src 'nonce-{nonce}'; base-uri 'none'; frame-ancestors 'none'; form-action 'none'")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authorize_url_is_encoded() {
        let url = authorize_url(
            "abc",
            "https://x.workers.dev/callback",
            "public_repo",
            "s&t",
        );
        assert!(url.starts_with("https://github.com/login/oauth/authorize?client_id=abc&"));
        assert!(url.contains("redirect_uri=https%3A%2F%2Fx.workers.dev%2Fcallback"));
        assert!(url.contains("state=s%26t"));
    }

    #[test]
    fn token_body() {
        let body = token_request_body("id", "se cret", "c0de", "https://a/b");
        assert_eq!(
            body,
            "client_id=id&client_secret=se+cret&code=c0de&redirect_uri=https%3A%2F%2Fa%2Fb"
        );
    }

    #[test]
    fn cookies() {
        let h = "a=1; oauth_state=abc123; b=2";
        assert_eq!(cookie(h, "oauth_state"), Some("abc123"));
        assert_eq!(cookie(h, "missing"), None);
        assert_eq!(cookie("", "a"), None);
        let set = set_cookie(
            "s",
            "v",
            &CookieOptions {
                path: "/",
                max_age: 60,
                same_site: "Strict",
            },
        );
        assert_eq!(
            set,
            "s=v; Path=/; Max-Age=60; HttpOnly; Secure; SameSite=Strict"
        );
    }

    #[test]
    fn equality() {
        assert!(constant_time_eq("abc", "abc"));
        assert!(!constant_time_eq("abc", "abd"));
        assert!(!constant_time_eq("abc", "ab"));
    }

    #[test]
    fn allowlist() {
        assert!(is_allowed_login("Nows", "someone, nows"));
        assert!(!is_allowed_login("nowsx", "nows"));
        assert!(!is_allowed_login("", ","));
        assert!(!is_allowed_login("nows", ""));
    }

    #[test]
    fn exact_origins() {
        assert!(is_exact_origin("https://nows.pages.dev"));
        assert!(is_exact_origin("https://example.com:8443"));
        assert!(is_exact_origin("http://localhost:1111"));
        assert!(!is_exact_origin("*"));
        assert!(!is_exact_origin("https://example.com/"));
        assert!(!is_exact_origin("https://example.com/admin"));
        assert!(!is_exact_origin("http://example.com"));
    }

    #[test]
    fn decap_messages() {
        let ok = decap_message(&DecapResult::Success { token: "t0k" });
        assert_eq!(
            ok,
            r#"authorization:github:success:{"provider":"github","token":"t0k"}"#
        );
        let err = decap_message(&DecapResult::Error { message: "denied" });
        assert!(err.starts_with("authorization:github:error:"));
    }

    #[test]
    fn script_injection_is_escaped() {
        let html = decap_callback_html(
            "</script><script>alert(1)</script>",
            "https://a.dev",
            "n0nce",
        );
        assert!(!html.contains("</script><script>"));
        assert!(html.contains("\\u003c/script\\u003e"));
        assert!(html.contains(r#"<script nonce="n0nce">"#));
        assert!(html.contains(r#"var allowed = "https://a.dev";"#));
    }

    #[test]
    fn csp_has_nonce() {
        assert!(popup_csp("abc").contains("script-src 'nonce-abc'"));
    }
}
