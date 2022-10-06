use httpavonz::web;
use std::env;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[tokio::main]
async fn main() {
    FmtSubscriber::builder()
        .with_env_filter(EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "httpavonz=debug,tower_http=debug".into()),
        ))
        .with_target(true)
        .with_ansi(true)
        .compact()
        .init();

    let router = web::build_app_router();
    let addr = web::build_sock_addr();

    tracing::info!("server started and listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
