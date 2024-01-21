use axum::{
    extract::{
        ws::{Message, WebSocketUpgrade, WebSocket},
        State,
    },
    routing::get,
    response::Response,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use std::{
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;

const SOCKET_ADDRESS: &'static str = "127.0.0.1:3000";
const INITIAL_FEN: &'static str = "aqabvrvnbrbnbbbqbkbbbnbrynyrsbsq/aranvpvpbpbpbpbpbpbpbpbpypypsnsr/nbnp12opob/nqnp12opoq/crcp12rprr/cncp12rprn/gbgp12pppb/gqgp12pppq/yqyp12vpvq/ybyp12vpvb/onop12npnn/orop12npnr/rqrp12cpcq/rbrp12cpcb/srsnppppwpwpwpwpwpwpwpwpgpgpanar/sqsbprpnwrwnwbwqwkwbwnwrgngrabaq";

type SharedState = Arc<AppState>;

struct AppState {
    fen: Mutex<String>,
    tx: broadcast::Sender<String>,
}

#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel(100);
    let app_state = Arc::new(AppState { fen: Mutex::new(INITIAL_FEN.to_string()), tx });

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/ws", get(ws_upgrade_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(SOCKET_ADDRESS).await.unwrap();
    println!("> Listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn ws_upgrade_handler(ws: WebSocketUpgrade, State(state): State<SharedState>) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: SharedState) {
    let (mut sender, mut receiver) = socket.split();

    {
        let fen = state.fen.lock().unwrap().clone();

        // Send the current state of the game as soon as a client connects
        if sender.send(Message::Text(fen)).await.is_err() {
            // client disconnected
            return;
        }
    }

    let cloned_state = state.clone();
    let tx = state.tx.clone();

    // Wait for messages and broadcast them to all subscribers
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(msg))) = receiver.next().await {
            // Update game state
            *cloned_state.fen.lock().unwrap() = msg.clone();
            println!("msg={:?}", msg);

            let _ = tx.send(msg);
        }
    });

    let mut rx = state.tx.subscribe();

    // Receive broadcast messages from above and forward them to all clients
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // If any one of the tasks run to completion, we abort the other.
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}
