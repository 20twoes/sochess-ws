use axum::extract::ws::{Message, WebSocket};
use futures::{sink::SinkExt, stream::StreamExt};
use mongodb::bson::doc;
use tracing::error;

use crate::game::Game;
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

pub async fn serve_play_game(socket: WebSocket, id: String, state: SharedState) {
    let span = tracing::info_span!("handle_socket");
    let _enter = span.enter();
    tracing::info!("connection opened");

    let (mut sender, mut receiver) = socket.split();

    let games_coll = state.db.collection::<Game>("games");
    let filter = doc! { "pid": id };
    let result = games_coll.find_one(filter, None).await;
    let game_option: Option<Game> = match result {
        Ok(option) => option,
        Err(err) => {
            error!("{:?}", err);
            None
        },
    };

    let mut game: Game;

    match game_option {
        Some(g) => {
            game = g;
        },
        None => {
            // TODO: Send an error message and close connection
            return;
        }
    }


    if let Some(fen) = game.fen() {
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
            game.add_move(msg.clone());
            save_game_move(&game, cloned_state.clone()).await;
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

async fn save_game_move(game: &Game, state: SharedState) {
    // TODO: Make this an atomic operation
    let games_coll = state.db.collection::<Game>("games");
    let filter = doc! { "pid": game.pid.clone() };

    let latest_move = game.moves.last().unwrap();
    let update = doc! {
        "$push": {
            "moves": bson::to_bson(latest_move).unwrap(),
        },
    };
    let _ = games_coll.update_one(
        filter,
        update,
        None
    ).await;
}
