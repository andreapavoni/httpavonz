use crate::HttpStatusResponse;
use axum::{
    extract::Path,
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{Html, IntoResponse, Redirect, Response},
    routing::get,
    Router,
};
use std::{env, net::SocketAddr};
use tower_http::{
    set_header::SetResponseHeaderLayer,
    trace::{DefaultOnResponse, TraceLayer},
    LatencyUnit,
};

pub fn build_sock_addr() -> SocketAddr {
    // Use "[::]" to listen on both IPv4 (0.0.0.0) and IPv6
    let srv_host = env::var("ROBO_RADIO_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let srv_port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    format!("{}:{}", srv_host, srv_port)
        .parse::<SocketAddr>()
        .unwrap()
}

pub fn build_app_router() -> Router {
    Router::new()
        .route("/", get(index_handler))
        .route("/:code", get(status_code_handler).post(status_code_handler))
        .layer(SetResponseHeaderLayer::if_not_present(
            header::SERVER,
            HeaderValue::from_static("httpavonz"),
        ))
        .layer(
            TraceLayer::new_for_http().on_response(
                DefaultOnResponse::new()
                    .include_headers(true)
                    .latency_unit(LatencyUnit::Micros),
            ),
        )
}

// Include utf-8 file at **compile** time.
async fn index_handler() -> Html<&'static str> {
    Html(std::include_str!("../index.html"))
}

async fn status_code_handler(Path(code): Path<u16>) -> Response {
    let status = HttpStatusResponse::new(code);

    if let Ok(status_code) = StatusCode::from_u16(status.code) {
        let body = if let Some(b) = status.body {
            b.to_string()
        } else {
            format!("{} {}", status.code, status.description)
        };

        let headers = status.headers.unwrap_or_else(|| HeaderMap::new());
        return (status_code, headers, body).into_response();
    }
    Redirect::to("/").into_response()
}