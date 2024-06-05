//! This module defines the data types we use to communicate with over websocket.
//!
//! # Examples
//!
//! ## Authenticate (identify user) [NOT IMPLEMENTED]
//!
//! TODO: Implement real authentication.  Right now we're just using the usernames.
//!
//! Request
//! ```
//! {
//!     "t": "auth",
//!     "user": "anon1234"
//! }
//! ```
//!
//! Response to client
//! ```
//! {
//!     "t": "ack"
//! }
//! ```
//!
//! ## Join a game
//!
//! User must authenticate first.  We will use this user info to know what player to add to the
//! game.
//!
//! Request
//! ```
//! {
//!     "t": "join"
//! }
//! ```
//!
//! Broadcast Response
//!
//! Send updated Game object
use mongodb::Database;
use serde_json::{Error, Value, json};

use crate::db;
use crate::game::Game;
use crate::user::User;

pub struct WebsocketMessage {
    db: Database,
    game: Game,
    json: Value,
    user: Option<User>,
}

impl WebsocketMessage {
    pub fn new(db_: Database, game: Game, user: User, msg: String) -> Result<Self, &'static str> {
        // Parse json string
        let result: Result<Value, Error> = serde_json::from_str(msg.as_str());

        if result.is_err() {
            return Err("Could not parse message");
        }

        // Get message type and return corresponding object
        let json = result.unwrap();

        Ok(
            Self {
                db: db_,
                game: game,
                json: json,
                user: Some(user),
            }
        )
    }

    pub async fn process(&mut self) -> Result<String, &'static str> {
        match self.json["t"].as_str() {
            //Some("auth") => Ok(self.authenticate_user()),
            Some("join") => Ok(self.join_game().await),
            _ => Err("Invalid message type"),
        }
    }

    //fn authenticate_user(&mut self) -> String {
    //    let username = self.json["user"].as_str().unwrap().to_string();
    //    let user = User {
    //        name: username,
    //    };
    //    self.user = Some(user);

    //    let resp = json!({
    //        "t": "ack"
    //    });
    //    resp.to_string()

    //}

    async fn join_game(&mut self) -> String {
        // Add player 2 to game
        //let player2 = self.json["player2"].as_str().unwrap().to_string();
        let player2 = self.user.as_ref().unwrap();
        self.game.set_player_joined(player2);
        db::update_player(&self.db, &self.game, &player2.name).await;

        let resp = json!({
            "t": "game_update",
            "d": {
                "state": self.game.state,
                "player2": player2.name,
            }
        });
        resp.to_string()
    }
}


//struct MakeMove {
//    player: String,
//    fen: String,
//}
//
//#[async_trait]
//impl MessageProcessor for MakeMove {
//    fn from_json_value(json: Value) -> Self {
//        Self {
//            player: json["player"].as_str().unwrap().to_string(),
//            fen: json["fen"].as_str().unwrap().to_string(),
//        }
//    }
//
//    async fn process(&self, db_: &Database, game: &mut Game) {
//        game.add_move(self.fen.clone());
//        db::save_game_move(&db_, &game).await;
//    }
//
//    fn response(&self) -> String {
//        let resp = json!({
//            "t": "game_update",
//            "d": {
//                "move": self.fen,
//            }
//        });
//        resp.to_string()
//    }
//}
