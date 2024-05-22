/*
 * Data schema we use to communicate with over websocket
 */
use mongodb::Database;
use serde_json::{Error, Value, json};

use crate::db;
use crate::game::Game;

pub struct WebsocketMessage {
    db: Database,
    game: Game,
    json: Value,
}

impl WebsocketMessage {
    pub fn new(db_: Database, game: Game, msg: String) -> Result<Self, &'static str> {
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
            }
        )
    }

    pub async fn process(&mut self) -> Result<String, &'static str> {
        match self.json["t"].as_str() {
            Some("join") => Ok(join_game(self).await),
            _ => Err("Invalid message type"),
        }
    }
}

async fn join_game(message: &mut WebsocketMessage) -> String {
    // Add player 2 to game
    let player2 = message.json["player2"].as_str().unwrap().to_string();
    message.game.set_player_joined();
    db::update_player(&message.db, &message.game, &player2).await;

    let resp = json!({
        "t": "game_update",
        "d": {
            "state": message.game.state,
            "player2": player2,
        }
    });
    resp.to_string()
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
