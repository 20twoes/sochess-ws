use bson::serde_helpers::chrono_datetime_as_bson_datetime;
use chrono::{DateTime, Utc};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use std::default::Default;

use crate::chessops::Position;
use crate::user::User;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GameState {
    Created,
    Accepted,
    FirstMove,
    InProgress,
    DefectMoveKing,
    Ended,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Game {
    // Public ID, to be used in URL
    pub pid: String,

    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub created: DateTime<Utc>,

    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub updated: DateTime<Utc>,

    pub moves: Vec<Move>,

    pub player1: Option<String>,

    pub player2: Option<String>,

    pub state: GameState,
}

impl Game {
    pub fn new() -> Self {
        let id_len = 12;
        let valid_chars: [char; 62] = [
            '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
            'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
            'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O',
            'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        ];

        Self {
            pid: nanoid!(id_len, &valid_chars),
            created: Utc::now(),
            updated: Utc::now(),
            moves: vec![Default::default()],
            player1: None,
            player2: None,
            state: GameState::Created,
        }
    }

    pub fn add_move(&mut self, fen: String, san: String) {
        self.moves.push(Move {
            san: san,
            fen: fen,
            ..Default::default()
        })
    }

    pub fn set_player_joined(&mut self, user: &User) {
        self.state = GameState::Accepted;
        self.player2 = Some(user.name.clone());
    }

    pub fn is_users_turn(&self, active_player: u8, user: &User) -> bool {
        match active_player {
            1 => user.name == self.player1.clone().unwrap(),
            2 => user.name == self.player2.clone().unwrap(),
            _ => {
                println!("Unrecognized active_player");
                false
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Move {
    /// Standard Algebraic Notation - notates the piece moved
    pub san: String,

    /// The resulting position using our custom FEN notation
    /// See chessops::fen module
    pub fen: String,

    /// Timestamp
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub ts: DateTime<Utc>,
}

impl Default for Move {
    fn default() -> Self {
        Self {
            san: String::from(""),
            fen: Position::new_fen(),
            ts: Utc::now(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameWithoutMoves {
    pub pid: String,
    pub fen: String,
    pub state: GameState,
}

impl GameWithoutMoves {
    pub fn from_game(game: Game) -> Self {
        Self {
            pid: game.pid,
            fen: game.moves.last().unwrap().fen.clone(),
            state: game.state,
        }
    }
}
