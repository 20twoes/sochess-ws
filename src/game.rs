use bson::serde_helpers::chrono_datetime_as_bson_datetime;
use chrono::{DateTime, Utc};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use std::default::Default;

use crate::user::User;

const INITIAL_FEN: &'static str = "aqabvrvnbrbnbbbqbkbbbnbrynyrsbsq/aranvpvpbpbpbpbpbpbpbpbpypypsnsr/nbnp12opob/nqnp12opoq/crcp12rprr/cncp12rprn/gbgp12pppb/gqgp12pppq/yqyp12vpvq/ybyp12vpvb/onop12npnn/orop12npnr/rqrp12cpcq/rbrp12cpcb/srsnppppwpwpwpwpwpwpwpwpgpgpanar/sqsbprpnwrwnwbwqwkwbwnwrgngrabaq";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GameState {
    Created,
    Accepted,
    FirstMove,
    P2Decided,
    InProgress,
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
        Self {
            pid: nanoid!(),
            created: Utc::now(),
            updated: Utc::now(),
            moves: vec![Default::default()],
            player1: None,
            player2: None,
            state: GameState::Created,
        }
    }

    //pub fn fen(&self) -> Option<String> {
    //    Some(self.moves.last()?.fen.clone())
    //}

    pub fn add_move(&mut self, fen: String) {
        let last_move = self.moves.last().unwrap();
        let active_player = if last_move.active_player == 1 { 2 } else { 1 };
        let ply = last_move.ply + 1;
        let _move: Move = Move {
            fen: fen,
            active_player: active_player,
            ply: ply,
            ..Default::default()
        };
        self.moves.push(_move)
    }

    pub fn set_player_joined(&mut self, user: &User) {
        self.state = GameState::Accepted;
        self.player2 = Some(user.name.clone());
    }

    pub fn is_users_turn(&self, user: &User) -> bool {
        let last_move = self.moves.last().unwrap();
        match last_move.active_player {
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
    // Standard Algebraic Notation - notates the piece moved
    pub san: String,

    // Forsyth-Edwards Notation - notates the resulting board position
    pub fen: String,

    // The player whose turn to move it is
    // `1` means Player1 is to move; `2` means Player2 is to move
    pub active_player: u8,

    // Half-turn, equal to one move by a player
    pub ply: u32,

    // Timestamp
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub ts: DateTime<Utc>,
}

impl Default for Move {
    fn default() -> Self {
        Self {
            san: String::from(""),
            fen: String::from(INITIAL_FEN),
            active_player: 1,
            ply: 0,
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
