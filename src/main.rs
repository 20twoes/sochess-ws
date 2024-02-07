mod game;
mod handler;
mod state;
mod websocket;

use axum::{
    Router,
    routing::{get, post},
};
use mongodb::{Client, options::ClientOptions};
use std::{
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::state::AppState;

const SOCKET_ADDRESS: &'static str = "0.0.0.0:3000";
const INITIAL_FEN: &'static str = "aqabvrvnbrbnbbbqbkbbbnbrynyrsbsq/aranvpvpbpbpbpbpbpbpbpbpypypsnsr/nbnp12opob/nqnp12opoq/crcp12rprr/cncp12rprn/gbgp12pppb/gqgp12pppq/yqyp12vpvq/ybyp12vpvb/onop12npnn/orop12npnr/rqrp12cpcq/rbrp12cpcb/srsnppppwpwpwpwpwpwpwpwpgpgpanar/sqsbprpnwrwnwbwqwkwbwnwrgngrabaq";

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

    // Set up database connection
    let db_connection_str = std::env::var("MONGO_URI").expect("Need to set `MONGO_URI` environment variable");
    let db_name = std::env::var("MONGO_DB").expect("Need to set `MONGO_DB` environment variable");
    let client_options = ClientOptions::parse(db_connection_str).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let db = client.database(&db_name);

    let (tx, _) = broadcast::channel(100);
    let app_state = Arc::new(AppState {
        db,
        fen: Mutex::new(INITIAL_FEN.to_string()),
        tx,
    });

    let api_routes = Router::new()
        .route("/games", get(handler::get_games))
        .route("/games", post(handler::create_game))
        .route("/games/:id", get(handler::get_game));

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/ws", get(handler::serve_websocket))
        .nest("/api", api_routes)
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(SOCKET_ADDRESS).await.unwrap();
    tracing::debug!("> Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
