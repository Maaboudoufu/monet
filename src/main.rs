//! Mochi's website server.
//!
//! Super-optimized by construction:
//! - std only, zero runtime dependencies
//! - the whole page is a `&'static str` compiled in: no per-request
//!   allocation, formatting, or template processing
//! - one connection per thread, `Connection: close` (correct for static HTML)
//! - ETag + 304 support, proper cache headers
//!
//! Run:  cargo run --release        (listens on 0.0.0.0:8080)

mod html;

use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::thread;

/// Page byte length, precomputed at compile time.
const LEN: usize = html::PAGE.len();
const ETAG: &str = "\"pompom-v1\"";

/// Where the photos live, relative to the running binary's dir (target/release).
fn asset_dir() -> PathBuf {
    // target/release/<exe> -> target/release/../.. = project root -> static/img
    let base = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));
    // If running from the project root directly (cargo run), use ./static/img
    if base.join("static/img").is_dir() {
        base.join("static/img")
    } else {
        // fallback: two levels up (target/release -> .. -> .. -> static/img)
        base.join("../..").join("static/img")
    }
}

fn main() {
    let addr = std::env::var("PORT")
        .map(|p| format!("0.0.0.0:{}", p.trim()))
        .unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    let assets = asset_dir();

    let listener = TcpListener::bind(&addr).expect("bind failed. Is the port busy?");
    println!("Mochi's site live at http://{} (ctrl-c to stop)", addr);
    println!("asset dir: {:?}", assets);

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            let assets = assets.clone();
            thread::spawn(move || {
                let _ = handle(stream, &assets);
            });
        }
    }
}

/// Read one request (bounded), reply.
fn handle(mut stream: TcpStream, assets: &PathBuf) -> std::io::Result<()> {
    // Static page: only the request line is needed.
    // Cap the read so a slow/large client can't tie up the socket.
    let mut buf = [0u8; 4096];
    let n = stream.read(&mut buf)?;
    let req = String::from_utf8_lossy(&buf[..n]);
    let first = req.lines().next().unwrap_or("");

    // Only GET/HEAD matter; anything else gets a short 405.
    let (method, path) = first
        .split_once(' ')
        .and_then(|(m, rest)| rest.split_once(' ').map(|(p, _)| (m, p)))
        .unwrap_or(("GET", "/"));

    // Static asset under /img/
    if (method == "GET" || method == "HEAD") && path.starts_with("/img/") {
        let name = &path[5..];
        // Only allow known, safe extensions to keep it boring.
        if name.contains("..") {
            let resp = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
            return stream.write_all(resp.as_bytes());
        }
        let full = assets.join(name);
        if let Ok(bytes) = fs::read(&full) {
            let mime = match full.extension().and_then(|e| e.to_str()) {
                Some("png") => "image/png",
                Some("webp") => "image/webp",
                Some("jpg") | Some("jpeg") => "image/jpeg",
                Some("svg") => "image/svg+xml",
                _ => "application/octet-stream",
            };
            let head = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\nCache-Control: public, max-age=86400\r\nConnection: close\r\n\r\n",
                mime,
                bytes.len()
            );
            stream.write_all(head.as_bytes())?;
            if method == "GET" {
                stream.write_all(&bytes)?;
            }
            return stream.flush();
        }
        let resp = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
        return stream.write_all(resp.as_bytes());
    }

    if (method == "GET" || method == "HEAD") && (path == "/" || path == "/index.html") {
        // ETag check for 304
        let etag_ok = req
            .lines()
            .find_map(|l| l.strip_prefix("If-None-Match:").or_else(|| l.strip_prefix("if-none-match:")))
            .map(|v| v.trim().to_ascii_lowercase().contains(ETAG))
            .unwrap_or(false);

        if etag_ok {
            let resp = if method == "HEAD" {
                "HTTP/1.1 304 Not Modified\r\nETag: \"pompom-v1\"\r\nCache-Control: public, max-age=3600\r\nConnection: close\r\n\r\n".to_string()
            } else {
                "HTTP/1.1 304 Not Modified\r\nETag: \"pompom-v1\"\r\nCache-Control: public, max-age=3600\r\nConnection: close\r\n\r\n".to_string()
            };
            return stream.write_all(resp.as_bytes());
        }

        let head = format!(
            "HTTP/1.1 200 OK\r\n\
             Content-Type: text/html; charset=utf-8\r\n\
             Content-Length: {}\r\n\
             ETag: \"pompom-v1\"\r\n\
             Cache-Control: public, max-age=3600\r\n\
             Connection: close\r\n\r\n",
            LEN
        );
        stream.write_all(head.as_bytes())?;
        if method == "GET" {
            stream.write_all(html::PAGE.as_bytes())?;
        }
    } else {
        let resp = "HTTP/1.1 405 Method Not Allowed\r\nContent-Type: text/plain\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok";
        stream.write_all(resp.as_bytes())?;
    }
    stream.flush()
}
