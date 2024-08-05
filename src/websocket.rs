use axum::extract::ws::{Message, WebSocket};
use futures::{sink::SinkExt, stream::StreamExt};

use crate::db;
use crate::game_handler::GameHandler;
use crate::state::SharedState;
use crate::user::User;

pub async fn serve_play_game(socket: WebSocket, id: String, state: SharedState, user: User) {
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
    if sender
        .send(Message::Text(serde_json::to_string(&game).unwrap()))
        .await
        .is_err()
    {
        // client disconnected
        return;
    }

    let cloned_state = state.clone();
    let cloned_user = user.clone();
    let tx = state.tx.clone();
    const ERR_PREFIX: &'static str = "error:";

    // Wait for messages and broadcast them to all subscribers
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(msg))) = receiver.next().await {
            tracing::info!("received msg={}", msg);

            // We need the latest game state
            let game_option = db::get_game(&cloned_state.db, id.as_str()).await;
            let Some(game) = game_option else {
                return;
            };

            // Determine message type
            // Process message
            // Broadcast update to all clients
            let mut handler =
                GameHandler::new(game.clone(), cloned_user.clone(), cloned_state.db.clone());
            let json = handler
                .read(&msg)
                .expect("Failed to parse websocket message");
            //let result = handler.process(json).await;
            match handler.process(json).await {
                Ok(_) => {
                    // Let's try sending the latest game object back each time.
                    // We can optimize later.
                    let game_option = db::get_game(&cloned_state.db, id.as_str()).await;
                    if let Some(game) = game_option {
                        let response = serde_json::to_string(&game).unwrap();
                        let _ = tx.send(response);
                    } else {
                        tracing::error!("Error fetching game after processing message");
                    }
                }
                Err(err) => {
                    // HACK: Add a prefix so we can send errors only to the user who made the move.
                    let msg = ERR_PREFIX.to_string()
                        + &cloned_user.name
                        + &serde_json::to_string(&err).unwrap();
                    let _ = tx.send(msg);
                }
            }
        }
    });

    let mut rx = state.tx.subscribe();

    // Receive broadcast messages from above and forward them to all clients
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            // If the message contains a username, then we only send that message to the
            // appropriate user
            if msg.starts_with(ERR_PREFIX) {
                let for_this_user = ERR_PREFIX.to_string() + &user.name;

                if msg.starts_with(&for_this_user) {
                    let parts: Vec<&str> = msg.as_str().split(&for_this_user).collect();
                    let data = parts[1]; // First part will be an empty string
                    if sender.send(Message::Text(data.to_string())).await.is_err() {
                        break;
                    }
                }
            } else {
                if sender.send(Message::Text(msg)).await.is_err() {
                    break;
                }
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
