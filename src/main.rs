mod db;
mod game;
mod handler;
mod state;
mod user;
mod websocket;
mod websocket_message;

use axum::{
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        Method,
    },
    routing::{get, post},
    Router,
};
use mongodb::{options::ClientOptions, Client};
use std::sync::Arc;
use tokio::sync::broadcast;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::state::AppState;

const SOCKET_ADDRESS: &'static str = "0.0.0.0:3000";

const CORS_ORIGINS: [&'static str; 1] = ["https://sovereign-chess-demo.web.app"];

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "sochess_be=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let env = std::env::var("ENV").expect("Need to set `ENV` environment variable");

    let cors_base = CorsLayer::new()
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
        .allow_methods([Method::GET, Method::POST]);

    let cors = if env == "dev" {
        cors_base.allow_origin(Any)
    } else {
        cors_base.allow_origin(CORS_ORIGINS.map(|i| i.parse().unwrap()))
    };

    // Set up database connection
    let db_connection_str =
        std::env::var("MONGO_URI").expect("Need to set `MONGO_URI` environment variable");
    let db_name = std::env::var("MONGO_DB").expect("Need to set `MONGO_DB` environment variable");
    let client_options = ClientOptions::parse(db_connection_str).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database(&db_name);

    let (tx, _) = broadcast::channel(100);
    let app_state = Arc::new(AppState { db, tx });

    let api_routes = Router::new()
        .route("/games", get(handler::get_games))
        .route("/games", post(handler::create_game))
        .route("/games/:id", get(handler::get_game))
        .route("/users", post(handler::create_user));

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/ws/v0/play/:id", get(handler::handle_websocket_play_game))
        .nest("/api", api_routes)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors),
        )
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(SOCKET_ADDRESS).await.unwrap();
    tracing::debug!("> Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
