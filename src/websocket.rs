use axum::extract::ws::{Message, WebSocket};
use futures::{sink::SinkExt, stream::StreamExt};

use crate::state::SharedState;

pub async fn websocket_service(socket: WebSocket, state: SharedState) {
    let span = tracing::info_span!("handle_socket");
    let _enter = span.enter();
    tracing::info!("connection opened");

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
            tracing::info!("received msg={}", msg);

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

    tracing::info!("connection closed");
}
