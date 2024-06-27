use async_trait::async_trait;
use mongodb::Database;
use std::error::Error;
use std::fmt;

use crate::db;
use crate::game::{Game, GameState};
use crate::user::User;

#[derive(Debug)]
struct GameHandlerError {
    message: String,
}

impl Error for GameHandlerError {}
impl fmt::Display for GameHandlerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub struct GameHandler {
    state: Option<Box<dyn HandlerState + Send + Sync>>,
    game: Game,
    user: User,
    db: Database,
}

impl GameHandler {
    pub fn new(game: Game, user: User, db_handle: Database) -> Self {
        match game.state {
            GameState::Created => Self {
                state: Some(Box::new(Created {})),
                game: game,
                user: user,
                db: db_handle,
            },
            GameState::Accepted => Self {
                state: Some(Box::new(Accepted {})),
                game: game,
                user: user,
                db: db_handle,
            },
            _ => todo!(),
        }
    }

    pub fn read(&self, message: &str) -> Result<serde_json::Value, serde_json::Error> {
        // Parse json string
        serde_json::from_str(message)
    }

    pub async fn process(&mut self, json: serde_json::Value) -> Result<(), &'static str> {
        match json["t"].as_str() {
            Some("join") => {
                if let Some(s) = self.state.take() {
                    self.state = Some(s.join_game(self).await.unwrap());
                }
                Ok(())
            }
            Some("move") => {
                if let Some(s) = self.state.take() {
                    let new_move = json["d"].as_str().unwrap();
                    self.state = Some(s.add_move(self, new_move.to_string()).await.unwrap());
                }
                Ok(())
            }
            _ => Err("Invalid message type"),
        }
    }
}

#[async_trait]
trait HandlerState {
    #[allow(unused_variables)]
    async fn join_game(
        &self,
        handler: &mut GameHandler,
    ) -> Result<Box<Accepted>, GameHandlerError> {
        Err(GameHandlerError {
            message: "Forbidden game action".to_string(),
        })
    }

    #[allow(unused_variables)]
    async fn add_move(
        &self,
        handler: &mut GameHandler,
        new_move: String,
    ) -> Result<Box<FirstMove>, GameHandlerError> {
        Err(GameHandlerError {
            message: "Forbidden game action".to_string(),
        })
    }
}

struct Created {}
struct Accepted {}
struct FirstMove {}

#[async_trait]
impl HandlerState for Created {
    async fn join_game(
        &self,
        handler: &mut GameHandler,
    ) -> Result<Box<Accepted>, GameHandlerError> {
        handler.game.set_player_joined(&handler.user);
        db::update_player(&handler.db, &handler.game, &handler.user.name).await;
        Ok(Box::new(Accepted {}))
    }
}

#[async_trait]
impl HandlerState for Accepted {
    async fn add_move(
        &self,
        handler: &mut GameHandler,
        new_move: String,
    ) -> Result<Box<FirstMove>, GameHandlerError> {
        let game = &mut handler.game;
        let user = &handler.user;
        if game.is_users_turn(user) {
            game.add_move(new_move.clone());
            db::save_game_move(&handler.db, &handler.game).await;
            Ok(Box::new(FirstMove {}))
        } else {
            Err(GameHandlerError {
                message: "Not player's turn".to_string(),
            })
        }
    }
}

impl HandlerState for FirstMove {}
