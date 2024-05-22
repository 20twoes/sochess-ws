use axum::extract::ws::{Message, WebSocket};
use futures::{sink::SinkExt, stream::StreamExt};

use crate::db;
use crate::state::SharedState;
use crate::websocket_message::WebsocketMessage;

pub async fn serve_play_game(socket: WebSocket, id: String, state: SharedState) {
    let span = tracing::info_span!("handle_socket");
    let _enter = span.enter();
    tracing::info!("connection opened");

    let (mut sender, mut receiver) = socket.split();

    let game_option = db::get_game(&state.db, id.as_str()).await;

    let Some(game) = game_option else {
        // TODO: Send an error message and close connection
        return;
    };

    // Send a response as soon as connection is opened
    if sender.send(Message::Text(serde_json::to_string(&game).unwrap())).await.is_err() {
        // client disconnected
        return;
    }

    let cloned_state = state.clone();
    let tx = state.tx.clone();

    // Wait for messages and broadcast them to all subscribers
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(msg))) = receiver.next().await {
            tracing::info!("received msg={}", msg);

            // Determine message type
            // Process message
            // Broadcast update to all clients
            match WebsocketMessage::new(cloned_state.db.clone(), game.clone(), msg) {
                Ok(mut message) => {
                    if let Ok(response) = message.process().await {
                        let _ = tx.send(response);
                    } else {
                        tracing::error!("Error processing message");
                    }
                },
                Err(err) => tracing::error!("{:?}", err),
            }

            // Update game state
            //game.add_move(msg.clone());
            //db::save_game_move(&cloned_state.db, &game).await;
            //tracing::info!("received msg={}", msg);

            //let _ = tx.send(msg);
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
