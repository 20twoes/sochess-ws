use async_trait::async_trait;
use mongodb::Database;
use serde::Serialize;
use std::error::Error;
use std::fmt;

use crate::db;
use crate::game::{Game, GameState, Move};
use crate::game_rules;
use crate::user::User;

#[derive(Debug, Serialize)]
pub struct GameHandlerError {
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

    pub async fn process(&mut self, json: serde_json::Value) -> Result<(), GameHandlerError> {
        match json["t"].as_str() {
            Some("join") => {
                if let Some(s) = self.state.take() {
                    self.state = Some(s.join_game(self).await.expect("Failed to join game"));
                }
                Ok(())
            }
            Some("move") => {
                if let Some(s) = self.state.take() {
                    let new_move = Move {
                        fen: json["d"]["fen"]
                            .as_str()
                            .expect("Cannot find FEN in data object")
                            .to_string(),
                        san: json["d"]["san"]
                            .as_str()
                            .expect("Cannot find SAN in data object")
                            .to_string(),
                        ..Default::default()
                    };
                    match s.add_move(self, new_move).await {
                        Ok(new_state) => {
                            self.state = Some(new_state);
                        }
                        Err(err) => {
                            return Err(err);
                        }
                    }
                }
                Ok(())
            }
            _ => Err(GameHandlerError {
                message: "Invalid message type".to_string(),
            }),
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
        new_move: Move,
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
        new_move: Move,
    ) -> Result<Box<FirstMove>, GameHandlerError> {
        let game = &mut handler.game;
        let user = &handler.user;
        let current_fen = game.moves.last().unwrap().fen.clone();

        if !game.is_users_turn(user) {
            Err(GameHandlerError {
                message: "Not your turn".to_string(),
            })
        } else if !game_rules::is_white_move(new_move.clone()) {
            Err(GameHandlerError {
                message: "Must move a white piece".to_string(),
            })
        } else if !game_rules::is_legal_move(new_move.clone(), current_fen) {
            Err(GameHandlerError {
                message: "Illegal move".to_string(),
            })
        } else {
            game.add_move(new_move.fen.clone());
            game.state = GameState::FirstMove;
            db::save_game_move(&handler.db, &handler.game).await;

            Ok(Box::new(FirstMove {}))
        }
    }
}

impl HandlerState for FirstMove {}
