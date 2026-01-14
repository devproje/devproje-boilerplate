use axum::{
    body::Body,
    extract::Request,
    middleware::Next,
    response::Response,
};
use chrono::Local;
use std::time::Instant;

pub async fn logger(request: Request<Body>, next: Next) -> Response {
    let start = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();

    let res = next.run(request).await;

    let latency = start.elapsed();
    let status = res.status();

    let status_color = match status.as_u16() {
        200..=299 => "\x1b[97;42m",
        300..=399 => "\x1b[90;47m",
        400..=499 => "\x1b[90;43m",
        _ => "\x1b[97;41m",
    };

    let method_color = match method.as_str() {
        "GET" => "\x1b[97;44m",
        "POST" => "\x1b[97;46m",
        "PUT" => "\x1b[90;43m",
        "DELETE" => "\x1b[97;41m",
        "PATCH" => "\x1b[97;42m",
        _ => "\x1b[97;45m"
    };

    let reset = "\x1b[0m";

    let latency_str = if latency.as_secs() > 0 {
        format!("{:.2}s", latency.as_secs_f64())
    } else if latency.as_millis() > 0 {
        format!("{}ms", latency.as_millis())
    } else {
        format!("{}µs", latency.as_micros())
    };

    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");

    println!(
        "{timestamp}  {status_color} {status} {reset} {latency_str:>10} | {method_color} {method:<7} {reset} {path}",
        timestamp = timestamp,
        status = status.as_u16(),
        latency_str = latency_str,
        method = method,
        path = uri.path(),
    );

    return res;
}
