use axum::{Router, routing::get};
use dotenv::dotenv;
use tokio::{net::TcpListener, signal::{self, unix::SignalKind}};

use crate::{config::Config, modules::{database, destroy_loader, get_loader, init_loader}};

mod util;
mod config;
mod routes;
mod modules;

#[allow(unsafe_op_in_unsafe_fn)]
unsafe fn load_modules() {
    init_loader();
    let loader = get_loader();

    loader.insmod(database::__module_init());
    loader.load();
}

async fn shutdown_hook() {
    let sig = async {
        signal::ctrl_c()
            .await
            .expect("Failed handling Ctrl+C signal");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(SignalKind::terminate())
            .expect("Failed handling SIGTERM")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = sig => { println!("Ctrl+C received") }
        _ = terminate => { println!("SIGTERM received") }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let config = Config::new();

    unsafe {
        load_modules();
    }

    let app = Router::new()
        .route("/", get(routes::index))
        .nest("/api", routes::api());

    let listener = TcpListener::bind(format!("{}:{}", config.host, config.port)).await.unwrap();
    println!("Service binding at: http://{}:{}", config.host, config.port);

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_hook())
        .await
        .unwrap();

    unsafe {
        destroy_loader();
    }
}
