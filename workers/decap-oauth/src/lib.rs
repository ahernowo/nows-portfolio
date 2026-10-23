//! Decap CMS GitHub login helper.
//!
//! Decap CMS runs in the browser and cannot keep a client secret, so GitHub
//! sign-in needs a small server. This worker is that server:
//!
//! 1. `GET /auth` sets a random `state` cookie and sends the browser to GitHub.
//! 2. GitHub sends the browser back to `GET /callback` with a one-time code.
//! 3. We check `state`, swap the code for a token, and hand the token to the
//!    Decap window, but only if that window is on `ALLOWED_ORIGIN`.
//!
//! Configuration (see `wrangler.toml`):
//! - vars: `ALLOWED_ORIGIN`, `GITHUB_SCOPE`
//! - secrets: `GITHUB_CLIENT_ID`, `GITHUB_CLIENT_SECRET`

use cms_core::oauth::{self, CookieOptions, DecapResult};
use serde::Deserialize;
use worker::*;

const STATE_COOKIE: &str = "decap_oauth_state";
const STATE_MAX_AGE: u64 = 600;
const USER_AGENT: &str = "nows-portfolio-decap-oauth";

struct Config {
    client_id: String,
    client_secret: String,
    allowed_origin: String,
    scope: String,
}

impl Config {
    fn from_env(env: &Env) -> std::result::Result<Self, String> {
        let get_secret = |name: &str| {
            env.secret(name)
                .map(|s| s.to_string())
                .or_else(|_| env.var(name).map(|v| v.to_string()))
                .map_err(|_| format!("{name} is not set. Run `npx wrangler secret put {name}`."))
        };
        let allowed_origin = env
            .var("ALLOWED_ORIGIN")
            .map(|v| v.to_string())
            .map_err(|_| "ALLOWED_ORIGIN is not set in wrangler.toml.".to_string())?;
        if !oauth::is_exact_origin(&allowed_origin) {
            return Err(format!(
                "ALLOWED_ORIGIN must look like https://example.com with no path or trailing slash, got {allowed_origin:?}."
            ));
        }
        let scope = env
            .var("GITHUB_SCOPE")
            .map(|v| v.to_string())
            .unwrap_or_else(|_| "public_repo".to_string());
        Ok(Self {
            client_id: get_secret("GITHUB_CLIENT_ID")?,
            client_secret: get_secret("GITHUB_CLIENT_SECRET")?,
            allowed_origin,
            scope,
        })
    }
}

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let url = req.url()?;
    if req.method() != Method::Get {
        return plain(405, "Only GET is supported.");
    }
    if url.path() == "/" {
        return plain(
            200,
            "Decap CMS sign-in helper. Open /admin on the website to sign in.",
        );
    }

    let config = match Config::from_env(&env) {
        Ok(c) => c,
        Err(msg) => {
            console_error!("config error: {msg}");
            return plain(
                500,
                "The sign-in helper is not configured. Check the worker settings.",
            );
        }
    };

    match url.path() {
        "/auth" => start(&url, &config),
        "/callback" => callback(&req, &url, &config).await,
        _ => plain(404, "Not found."),
    }
}

/// Step 1: remember a random state in a cookie and send the browser to GitHub.
fn start(url: &Url, config: &Config) -> Result<Response> {
    let state = random_hex(16)?;
    let redirect_uri = callback_url(url);
    let location = oauth::authorize_url(&config.client_id, &redirect_uri, &config.scope, &state);

    let headers = Headers::new();
    headers.set("Location", &location)?;
    headers.set("Cache-Control", "no-store")?;
    headers.set(
        "Set-Cookie",
        &oauth::set_cookie(
            STATE_COOKIE,
            &state,
            &CookieOptions {
                path: "/callback",
                max_age: STATE_MAX_AGE,
                same_site: "Lax",
            },
        ),
    )?;
    Ok(Response::empty()?.with_status(302).with_headers(headers))
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

/// Step 2 and 3: check state, exchange the code, hand the token to Decap.
async fn callback(req: &Request, url: &Url, config: &Config) -> Result<Response> {
    let query: std::collections::HashMap<String, String> = url.query_pairs().into_owned().collect();

    if let Some(err) = query.get("error") {
        let message = query
            .get("error_description")
            .cloned()
            .unwrap_or_else(|| err.clone());
        return popup(config, &DecapResult::Error { message: &message });
    }

    let (Some(code), Some(state)) = (query.get("code"), query.get("state")) else {
        return popup(
            config,
            &DecapResult::Error {
                message: "GitHub did not send a sign-in code. Try again.",
            },
        );
    };

    let cookie_header = req.headers().get("Cookie")?.unwrap_or_default();
    let expected = oauth::cookie(&cookie_header, STATE_COOKIE).unwrap_or_default();
    if expected.is_empty() || !oauth::constant_time_eq(expected, state) {
        return popup(
            config,
            &DecapResult::Error {
                message:
                    "Sign-in expired or was started elsewhere. Close this window and sign in again.",
            },
        );
    }

    let token = match exchange_code(config, code, &callback_url(url)).await {
        Ok(t) => t,
        Err(message) => {
            console_error!("token exchange failed: {message}");
            return popup(
                config,
                &DecapResult::Error {
                    message: "GitHub refused the sign-in. Try again.",
                },
            );
        }
    };

    popup(config, &DecapResult::Success { token: &token })
}

async fn exchange_code(
    config: &Config,
    code: &str,
    redirect_uri: &str,
) -> std::result::Result<String, String> {
    let headers = Headers::new();
    let set = |k: &str, v: &str| headers.set(k, v).map_err(|e| e.to_string());
    set("Accept", "application/json")?;
    set("Content-Type", "application/x-www-form-urlencoded")?;
    set("User-Agent", USER_AGENT)?;

    let body =
        oauth::token_request_body(&config.client_id, &config.client_secret, code, redirect_uri);
    let mut init = RequestInit::new();
    init.with_method(Method::Post)
        .with_headers(headers)
        .with_body(Some(wasm_bindgen::JsValue::from_str(&body)));

    let request = Request::new_with_init(oauth::GITHUB_TOKEN, &init).map_err(|e| e.to_string())?;
    let mut response = Fetch::Request(request)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let parsed: TokenResponse = response.json().await.map_err(|e| e.to_string())?;

    match (parsed.access_token, parsed.error) {
        (Some(token), _) if !token.is_empty() => Ok(token),
        (_, Some(err)) => Err(format!(
            "{err}: {}",
            parsed.error_description.unwrap_or_default()
        )),
        _ => Err("no access_token in response".into()),
    }
}

/// The small page that runs in the popup and talks to the Decap window.
fn popup(config: &Config, result: &DecapResult<'_>) -> Result<Response> {
    let nonce = random_hex(16)?;
    let html = oauth::decap_callback_html(
        &oauth::decap_message(result),
        &config.allowed_origin,
        &nonce,
    );

    let headers = Headers::new();
    headers.set("Content-Type", "text/html; charset=utf-8")?;
    headers.set("Content-Security-Policy", &oauth::popup_csp(&nonce))?;
    headers.set("Cache-Control", "no-store")?;
    headers.set("Referrer-Policy", "no-referrer")?;
    headers.set("X-Content-Type-Options", "nosniff")?;
    headers.set(
        "Set-Cookie",
        &oauth::clear_cookie(STATE_COOKIE, "/callback"),
    )?;
    Ok(Response::from_html(html)?.with_headers(headers))
}

fn plain(status: u16, body: &str) -> Result<Response> {
    let headers = Headers::new();
    headers.set("Content-Type", "text/plain; charset=utf-8")?;
    headers.set("X-Content-Type-Options", "nosniff")?;
    headers.set("Cache-Control", "no-store")?;
    Ok(Response::ok(body)?
        .with_status(status)
        .with_headers(headers))
}

fn callback_url(url: &Url) -> String {
    format!("{}/callback", url.origin().ascii_serialization())
}

fn random_hex(len: usize) -> Result<String> {
    let mut buf = vec![0u8; len];
    getrandom::getrandom(&mut buf)
        .map_err(|e| Error::RustError(format!("random source failed: {e}")))?;
    Ok(oauth::to_hex(&buf))
}
