//! A tiny local web server for the dashboard, using only the standard
//! library. It answers GET requests on 127.0.0.1 and nothing else.
//!
//! It checks the `Host` header so a web page on another site can't reach it
//! through DNS tricks, and sends a strict Content Security Policy.

use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::board::{is_valid_id, normalise_id};
use crate::store::{now, Store};

const INDEX_HTML: &str = include_str!("../dashboard/index.html");
const APP_JS: &str = include_str!("../dashboard/app.js");
const APP_CSS: &str = include_str!("../dashboard/app.css");

const CSP: &str = "default-src 'none'; script-src 'self'; style-src 'self'; font-src 'self'; img-src 'self' data:; connect-src 'self'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'";

struct Reply {
    status: &'static str,
    content_type: &'static str,
    body: Vec<u8>,
}

impl Reply {
    fn ok(content_type: &'static str, body: impl Into<Vec<u8>>) -> Self {
        Self {
            status: "200 OK",
            content_type,
            body: body.into(),
        }
    }

    fn json(value: &serde_json::Value) -> Self {
        Self::ok("application/json; charset=utf-8", value.to_string())
    }

    fn not_found() -> Self {
        Self {
            status: "404 Not Found",
            content_type: "text/plain; charset=utf-8",
            body: b"Not found".to_vec(),
        }
    }

    fn error(status: &'static str, message: &str) -> Self {
        Self {
            status,
            content_type: "text/plain; charset=utf-8",
            body: message.as_bytes().to_vec(),
        }
    }
}

pub fn serve(store: Store, port: u16) -> Result<(), String> {
    let listener = TcpListener::bind(("127.0.0.1", port))
        .map_err(|e| format!("Could not open port {port}: {e}. Try another with --port."))?;
    println!("Factory dashboard: http://127.0.0.1:{port}/  (Ctrl+C to stop)");
    let store = Arc::new(store);
    for stream in listener.incoming() {
        let Ok(stream) = stream else { continue };
        let store = Arc::clone(&store);
        thread::spawn(move || {
            let _ = handle(stream, &store, port);
        });
    }
    Ok(())
}

fn handle(mut stream: TcpStream, store: &Store, port: u16) -> std::io::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    let mut reader = BufReader::new(stream.try_clone()?);

    let mut request_line = String::new();
    reader.read_line(&mut request_line)?;
    let mut host = String::new();
    let mut header_bytes = 0;
    loop {
        let mut line = String::new();
        let n = reader.read_line(&mut line)?;
        header_bytes += n;
        if n == 0 || line == "\r\n" || line == "\n" || header_bytes > 16_384 {
            break;
        }
        if let Some((name, value)) = line.split_once(':') {
            if name.eq_ignore_ascii_case("host") {
                host = value.trim().to_string();
            }
        }
    }

    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or_default();
    let target = parts.next().unwrap_or_default();

    let reply = if host != format!("127.0.0.1:{port}") && host != format!("localhost:{port}") {
        Reply::error(
            "421 Misdirected Request",
            "Open the dashboard at http://127.0.0.1 on this port.",
        )
    } else if method != "GET" {
        Reply::error("405 Method Not Allowed", "The dashboard is read-only.")
    } else {
        route(target.split('?').next().unwrap_or_default(), store)
    };

    let head = format!(
        "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nContent-Security-Policy: {CSP}\r\nX-Content-Type-Options: nosniff\r\nReferrer-Policy: no-referrer\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
        reply.status,
        reply.content_type,
        reply.body.len()
    );
    stream.write_all(head.as_bytes())?;
    stream.write_all(&reply.body)?;
    stream.flush()
}

fn route(path: &str, store: &Store) -> Reply {
    match path {
        "/" | "/index.html" => Reply::ok("text/html; charset=utf-8", INDEX_HTML),
        "/app.js" => Reply::ok("text/javascript; charset=utf-8", APP_JS),
        "/app.css" => Reply::ok("text/css; charset=utf-8", APP_CSS),
        "/api/state" => state(store),
        _ if path.starts_with("/fonts/") => font(path, store),
        _ if path.starts_with("/api/orders/") => order_doc(path, store),
        _ => Reply::not_found(),
    }
}

fn state(store: &Store) -> Reply {
    match store.load() {
        Ok(board) => {
            let reports: serde_json::Map<String, serde_json::Value> = board
                .orders
                .iter()
                .map(|o| {
                    let have: Vec<&str> = store.reports(&o.id).iter().map(|s| s.as_str()).collect();
                    (o.id.clone(), serde_json::json!(have))
                })
                .collect();
            Reply::json(&serde_json::json!({
                "now": now(),
                "project": store.root.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default(),
                "board": board,
                "reports": reports,
                "lessons": store.lessons().iter().rev().take(30).collect::<Vec<_>>(),
            }))
        }
        Err(message) => Reply::json(&serde_json::json!({ "error": message })),
    }
}

/// `/api/orders/WO-0001/plan.md` -> the report, rendered safely to HTML.
fn order_doc(path: &str, store: &Store) -> Reply {
    let rest = path.trim_start_matches("/api/orders/");
    let Some((id, file)) = rest.split_once('/') else {
        return Reply::not_found();
    };
    let id = normalise_id(id);
    if !is_valid_id(&id) {
        return Reply::not_found();
    }
    match store.read_doc(&id, file) {
        Some(text) => Reply::json(&serde_json::json!({
            "file": file,
            "html": cms_core::preview::render_safe(&text),
        })),
        None => Reply::not_found(),
    }
}

/// Serve the site's own fonts, so the dashboard matches it.
fn font(path: &str, store: &Store) -> Reply {
    let name = path.trim_start_matches("/fonts/");
    let safe = name.ends_with(".woff2")
        && name
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-' || b == b'.')
        && !name.contains("..");
    if !safe {
        return Reply::not_found();
    }
    match std::fs::read(store.root.join("site/static/fonts").join(name)) {
        Ok(bytes) => Reply::ok("font/woff2", bytes),
        Err(_) => Reply::not_found(),
    }
}
