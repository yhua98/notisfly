mod api;
mod cli;
mod db;
mod error;
mod profile;
mod store;

use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
};

use axum::Router;
use sqlx::{Pool, Postgres};
use tokio::signal;

use tower_http::services::ServeDir;

#[derive(Clone, Debug)]
struct AppState {
    db: Pool<Postgres>,
    docs: Arc<RwLock<HashMap<String, yrs::Doc>>>,
    #[allow(unused)]
    profile: profile::Profile,
}

#[tokio::main]
async fn main() {
    // get profile from command line arguments and .env file
    let profile = if let Ok(profile) = profile::Profile::parse() {
        profile
    } else {
        eprintln!("Failed to parse profile");
        std::process::exit(1);
    };

    println!("{}", profile.clone().banner());

    let db_conn = db::connect(profile.db_url.clone()).await.unwrap();

    let app_state = AppState {
        db: db_conn.clone(),
        docs: Arc::new(RwLock::new(HashMap::new())),
        profile: profile.clone(),
    };

    let app = Router::new()
        .nest("/api", api::register())
        .nest_service("/", ServeDir::new("assets"))
        .with_state(app_state);

    let (shutdown_send, mut shutdown_recv) = tokio::sync::mpsc::unbounded_channel::<bool>();
    let (close_send, close_recv) = tokio::sync::mpsc::unbounded_channel::<bool>();

    tokio::spawn(async move {
        tracing_subscriber::fmt::init();

        let listener =
            tokio::net::TcpListener::bind(format!("0.0.0.0:{port}", port = profile.port.clone()))
                .await
                .unwrap();
        let serve = axum::serve(
            listener,
            app.into_make_service_with_connect_info::<SocketAddr>(),
        );

        serve
            .with_graceful_shutdown(async move {
                println!("Serve: waiting for shutdown signal...");
                shutdown_recv.recv().await;
                println!("Serve: close finished");
                close_send.send(true).unwrap();
            })
            .await
            .unwrap();
    });

    shutdown_signal(db_conn, shutdown_send, close_recv).await
}

async fn shutdown_signal(
    db: Pool<Postgres>,
    shutdown_send: tokio::sync::mpsc::UnboundedSender<bool>,
    mut close_recv: tokio::sync::mpsc::UnboundedReceiver<bool>,
) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            println!("Ctrl+C received, send shutdown signal...");
            shutdown_send.send(true).unwrap();
            close_recv.recv().await;
            println!("Close finished, closing db connection...");
            db.close().await;
            println!("DB connection closed, bye!");
        },
        _ = terminate => {},
    }
}
