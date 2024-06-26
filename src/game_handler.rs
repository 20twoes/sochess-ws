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
            //GameState::Accepted => GameHandler::<Accepted + 'static> {
            //    game: game.clone(),
            //    marker: std::marker::PhantomData,
            //},
            _ => todo!(),
        }
    }

    pub fn read(&self, message: &str) -> Result<serde_json::Value, serde_json::Error> {
        // Parse json string
        serde_json::from_str(message)
    }

    pub async fn process(&mut self, json: serde_json::Value) -> Result<(), &'static str> {
        println!("process 0:  {:?}", json);
        match json["t"].as_str() {
            Some("join") => {
                println!("process 1");
                if let Some(s) = self.state.take() {
                    println!("process 2");
                    self.state = Some(
                        s.join_game(&self.db, &mut self.game, &self.user)
                            .await
                            .unwrap(),
                    );
                }
                Ok(())
            }
            _ => Err("Invalid message type"),
        }
    }
}

#[async_trait]
trait HandlerState {
    async fn join_game(
        &self,
        db_handle: &Database,
        game: &mut Game,
        user: &User,
    ) -> Result<Box<Accepted>, GameHandlerError> {
        Err(GameHandlerError {
            message: "Forbidden game action".to_string(),
        })
    }
}

struct Created {}
struct Accepted {}

#[async_trait]
impl HandlerState for Created {
    async fn join_game(
        &self,
        db_handle: &Database,
        game: &mut Game,
        user: &User,
    ) -> Result<Box<Accepted>, GameHandlerError> {
        println!("running join_game");
        game.set_player_joined(user);
        db::update_player(db_handle, game, &user.name).await;
        Ok(Box::new(Accepted {}))
    }
}

impl HandlerState for Accepted {}
