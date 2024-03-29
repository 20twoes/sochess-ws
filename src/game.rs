use bson::serde_helpers::chrono_datetime_as_bson_datetime;
use chrono::{DateTime, Utc};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use std::default::Default;

const INITIAL_FEN: &'static str = "aqabvrvnbrbnbbbqbkbbbnbrynyrsbsq/aranvpvpbpbpbpbpbpbpbpbpypypsnsr/nbnp12opob/nqnp12opoq/crcp12rprr/cncp12rprn/gbgp12pppb/gqgp12pppq/yqyp12vpvq/ybyp12vpvb/onop12npnn/orop12npnr/rqrp12cpcq/rbrp12cpcb/srsnppppwpwpwpwpwpwpwpwpgpgpanar/sqsbprpnwrwnwbwqwkwbwnwrgngrabaq";

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    // Public ID, to be used in URL
    pub pid: String,

    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub created: DateTime<Utc>,

    #[serde(with = "chrono_datetime_as_bson_datetime")]
    pub updated: DateTime<Utc>,

    pub moves: Vec<Move>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            pid: nanoid!(),
            created: Utc::now(),
            updated: Utc::now(),
            moves: vec![Default::default()],
        }
    }

    pub fn fen(&self) -> Option<String> {
        Some(self.moves.last()?.fen.clone())
    }

    pub fn add_move(&mut self, fen: String) {
        let mut _move: Move = Default::default();
        _move.fen = fen;
        self.moves.push(_move)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Move {
    // Standard Algebraic Notation - notates the piece moved
    pub san: String,

    // Forsyth-Edwards Notation - notates the resulting board position
    pub fen: String,

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
            ply: 0,
            ts: Utc::now(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameWithoutMoves {
    pub pid: String,
    pub fen: String,
}

impl GameWithoutMoves {
    pub fn from_game(game: Game) -> Self {
        Self {
            pid: game.pid,
            fen: game.moves.last().unwrap().fen.clone(),
        }
    }
}
