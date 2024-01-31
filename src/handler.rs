use axum::{
    extract::{ws::WebSocketUpgrade, State},
    response::Response,
    Json,
};
use futures::StreamExt;
use nanoid::nanoid;
use tracing::error;

use crate::game::Game;
use crate::websocket;
use crate::state::SharedState;

pub async fn serve_websocket(ws: WebSocketUpgrade, State(state): State<SharedState>) -> Response {
    ws.on_upgrade(|socket| websocket::websocket_service(socket, state))
}

pub async fn get_games(State(state): State<SharedState>) -> Json<Vec<Game>> {
    let games_coll = state.db.collection::<Game>("games");
    let cursor = games_coll.find(None, None).await;
    match cursor {
        Ok(mut cursor) => {
            let mut games = Vec::new();
            while let Some(Ok(game)) = cursor.next().await {
                games.push(game);
            }
            Json(games)
        },
        Err(err) => {
            error!("{:?}", err);
            Json(vec![])
        },
    }
}

pub async fn create_game(State(state): State<SharedState>) -> Json<Game> {
    let game = Game { pid: nanoid!() };
    let games_coll = state.db.collection::<Game>("games");
    let result = games_coll.insert_one(&game, None).await;
    match result {
        Ok(_) => (),
        Err(err) => error!("{:?}", err),
    }
    Json(game)
}
