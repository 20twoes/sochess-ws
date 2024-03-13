use axum::{
    http::StatusCode,
    extract::{
        ws::WebSocketUpgrade,
        Path,
        State,
    },
    response::Response,
    Json,
};
use futures::StreamExt;
use mongodb::bson::doc;
use tracing::error;

use crate::game::{Game, GameWithoutMoves};
use crate::websocket;
use crate::state::SharedState;

pub async fn serve_websocket(ws: WebSocketUpgrade, State(state): State<SharedState>) -> Response {
    ws.on_upgrade(|socket| websocket::websocket_service(socket, state))
}

pub async fn handle_websocket_play_game(Path(id): Path<String>, ws: WebSocketUpgrade, State(state): State<SharedState>) -> Response {
    ws.on_upgrade(|socket| websocket::serve_play_game(socket, id, state))
}

pub async fn get_games(State(state): State<SharedState>) -> Json<Vec<GameWithoutMoves>> {
    tracing::info!("get_games");
    let games_coll = state.db.collection::<Game>("games");
    let cursor = games_coll.find(None, None).await;
    match cursor {
        Ok(mut cursor) => {
            let mut games = Vec::new();
            while let Some(result) = cursor.next().await {
                match result {
                    Ok(game) => {
                        let g = GameWithoutMoves::from_game(game);
                        games.push(g);
                    },
                    Err(err) => error!("{:?}", err),
                }
            }
            Json(games)
        },
        Err(err) => {
            error!("{:?}", err);
            Json(Vec::new())
        },
    }
}

pub async fn get_game(Path(id): Path<String>, State(state): State<SharedState>) -> Result<Json<Game>, StatusCode> {
    let games_coll = state.db.collection::<Game>("games");
    let filter = doc! { "pid": id };
    let result = games_coll.find_one(filter, None).await;
    match result {
        Ok(option) => {
            match option {
                Some(game) => Ok(Json(game)),
                None => Err(StatusCode::NOT_FOUND),
            }
        },
        Err(err) => {
            error!("{:?}", err);
            Err(StatusCode::NOT_FOUND)
        },
    }
}

pub async fn create_game(State(state): State<SharedState>) -> Result<Json<Game>, StatusCode> {
    let game = Game::new();
    let games_coll = state.db.collection::<Game>("games");
    let result = games_coll.insert_one(&game, None).await;
    match result {
        Ok(_) => Ok(Json(game)),
        Err(err) => {
            error!("{:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}
