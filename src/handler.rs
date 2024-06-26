use axum::{
    extract::{ws::WebSocketUpgrade, Path, Query, State},
    http::{
        header::{HeaderMap, AUTHORIZATION},
        StatusCode,
    },
    response::Response,
    Json,
};
use futures::StreamExt;
use mongodb::{bson::doc, Database};
use std::collections::HashMap;
use tracing::error;

use crate::db;
use crate::game::{Game, GameWithoutMoves};
use crate::state::SharedState;
use crate::user::User;
use crate::websocket;

pub async fn handle_websocket_play_game(
    Path(id): Path<String>,
    Query(params): Query<HashMap<String, String>>,
    ws: WebSocketUpgrade,
    State(state): State<SharedState>,
) -> Result<Response, StatusCode> {
    // TODO: Replace this hack to get the user info for the websocket connection.
    // This is very easy to hack.
    let username = &params["user"];
    let user = match db::get_user(&state.db, username.as_str()).await {
        Some(u) => u,
        None => {
            return Err(StatusCode::UNAUTHORIZED);
        }
    };
    Ok(ws.on_upgrade(|socket| websocket::serve_play_game(socket, id, state, user)))
}

async fn extract_user(headers: HeaderMap, database: &Database) -> Result<User, &'static str> {
    // Check user info was sent in headers
    let username = match get_auth_token(headers) {
        Some(token) => {
            println!("username: {:?}", token);
            token
        }
        None => {
            return Err("No username found");
        }
    };

    // Check if username is valid
    let user = match db::get_user(&database, username.as_str()).await {
        Some(u) => u,
        None => {
            return Err("No username found");
        }
    };

    Ok(user)
}

fn get_auth_token(headers: HeaderMap) -> Option<String> {
    return match headers.get(AUTHORIZATION) {
        Some(token) => {
            //println!("get_auth_token: {:?}", token);
            let value = token.to_str().unwrap();
            let mut parts = value.split(' ');
            let _auth_type = parts.next();
            Some(parts.next().unwrap().to_string())
        }
        None => {
            println!("get_auth_token: none found");
            None
        }
    };
}

pub async fn get_games(
    headers: HeaderMap,
    State(state): State<SharedState>,
) -> Result<Json<Vec<GameWithoutMoves>>, StatusCode> {
    tracing::info!("get_games");

    let user = match extract_user(headers, &state.db).await {
        Ok(user) => user,
        Err(_) => {
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    let games_coll = state.db.collection::<Game>("games");
    let filter = doc! {
        "$or": [{"player1": user.name.clone()}, {"player2": user.name}],
    };
    let cursor = games_coll.find(filter, None).await;
    match cursor {
        Ok(mut cursor) => {
            let mut games = Vec::new();
            while let Some(result) = cursor.next().await {
                match result {
                    Ok(game) => {
                        let g = GameWithoutMoves::from_game(game);
                        games.push(g);
                    }
                    Err(err) => error!("{:?}", err),
                }
            }
            Ok(Json(games))
        }
        Err(err) => {
            error!("{:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_game(
    Path(id): Path<String>,
    State(state): State<SharedState>,
) -> Result<Json<Game>, StatusCode> {
    tracing::info!("get_game");
    let games_coll = state.db.collection::<Game>("games");
    let filter = doc! { "pid": id };
    let result = games_coll.find_one(filter, None).await;
    match result {
        Ok(option) => match option {
            Some(game) => Ok(Json(game)),
            None => Err(StatusCode::NOT_FOUND),
        },
        Err(err) => {
            error!("{:?}", err);
            Err(StatusCode::NOT_FOUND)
        }
    }
}

pub async fn create_game(
    headers: HeaderMap,
    State(state): State<SharedState>,
) -> Result<Json<Game>, StatusCode> {
    tracing::info!("create_game");

    let user = match extract_user(headers, &state.db).await {
        Ok(user) => user,
        Err(_) => {
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    let mut game = Game::new();
    game.player1 = Some(user.name);
    let games_coll = state.db.collection::<Game>("games");
    let result = games_coll.insert_one(&game, None).await;
    match result {
        Ok(_) => Ok(Json(game)),
        Err(err) => {
            error!("{:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create_user(State(state): State<SharedState>) -> Result<Json<User>, StatusCode> {
    let user = User::new();
    let user_coll = state.db.collection::<User>("users");
    let result = user_coll.insert_one(&user, None).await;
    match result {
        Ok(_) => Ok(Json(user)),
        Err(err) => {
            error!("{:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
