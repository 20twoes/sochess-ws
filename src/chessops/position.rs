use std::collections::HashSet;

use crate::chessops::{Color, Move, Player};

const INITIAL_FEN: &'static str = "aqabvrvnbrbnbbbqbkbbbnbrynyrsbsq/aranvpvpbpbpbpbpbpbpbpbpypypsnsr/nbnp12opob/nqnp12opoq/crcp12rprr/cncp12rprn/gbgp12pppb/gqgp12pppq/yqyp12vpvq/ybyp12vpvb/onop12npnn/orop12npnr/rqrp12cpcq/rbrp12cpcb/srsnppppwpwpwpwpwpwpwpwpgpgpanar/sqsbprpnwrwnwbwqwkwbwnwrgngrabaq 1 - - - - 0";

#[derive(Debug)]
pub struct PlayError {}

#[derive(Debug, PartialEq)]
pub struct Position {
    board: String,
    active_player: Player,
    p1_owned: Option<Color>,
    p1_controlled: HashSet<Color>,
    p2_owned: Option<Color>,
    p2_controlled: HashSet<Color>,
    ply: u32,
}

impl Position {
    pub fn new_fen() -> String {
        String::from(INITIAL_FEN)
    }

    pub fn new() -> Self {
        Self::from_fen(Self::new_fen())
    }

    pub fn from_fen(fen: String) -> Self {
        let parts: Vec<&str> = fen.split(' ').collect();
        assert_eq!(parts.len(), 7);
        assert_eq!(parts[1].len(), 1);
        assert_eq!(parts[2].len(), 1);
        assert_eq!(parts[4].len(), 1);

        let active_player = Player::from_char(parts[1].chars().nth(0).unwrap());
        assert!(active_player.is_some());

        let mut p1_controlled = HashSet::new();
        for ch in parts[3].chars() {
            if let Some(c) = Color::from_char(ch) {
                p1_controlled.insert(c);
            }
        }

        let mut p2_controlled = HashSet::new();
        for ch in parts[5].chars() {
            if let Some(c) = Color::from_char(ch) {
                p2_controlled.insert(c);
            }
        }

        Self {
            board: parts[0].to_string(),
            active_player: active_player.unwrap(),
            p1_owned: Color::from_char(parts[2].chars().nth(0).unwrap()),
            p1_controlled: p1_controlled,
            p2_owned: Color::from_char(parts[4].chars().nth(0).unwrap()),
            p2_controlled: p2_controlled,
            ply: parts[6].parse::<u32>().expect("`ply` is invalid"),
        }
    }

    pub fn to_fen(&self) -> String {
        let p1_owned = if let Some(v) = &self.p1_owned {
            v.to_char().clone()
        } else {
            '-'
        };

        let p2_owned = if let Some(v) = &self.p2_owned {
            v.to_char().clone()
        } else {
            '-'
        };

        let mut p1_controlled = self.p1_controlled.iter().map(|x| x.to_char().clone()).collect::<Vec<char>>();
        let mut p2_controlled = self.p2_controlled.iter().map(|x| x.to_char().clone()).collect::<Vec<char>>();
        let p1_controlled_computed = if p1_controlled.len() > 0 {
            p1_controlled.sort_unstable();
            p1_controlled.into_iter().collect::<String>()
        } else {
            "-".to_string()
        };
        let p2_controlled_computed = if p2_controlled.len() > 0 {
            p2_controlled.sort_unstable();
            p2_controlled.into_iter().collect::<String>()
        } else {
            "-".to_string()
        };

        format!(
            "{} {} {} {} {} {} {}",
            self.board,
            self.active_player.to_int(),
            p1_owned,
            p1_controlled_computed,
            p2_owned,
            p2_controlled_computed,
            self.ply,
        ).to_string()
    }

    pub fn active_player(&self) -> u8 {
        self.active_player.to_int()
    }

    pub fn play_move(&mut self, new_move: &Move) -> Result<&Self, PlayError> {
        if self.p1_owned.is_none() {
            // Must be the first move of the game,
            // thus it must be a white piece that is moved.
            if new_move.color != Color::White {
                return Err(PlayError {});
            }
        }
        self.active_player = self.active_player.next();
        self.ply += 1;
        Ok(self)
        //match self.active_player {
        //    Player::P1 => {
        //        match self.p1_owned {
        //            Some(color) => {
        //                // Does it match the move color?
        //            }
        //        }
        //    }
        //    _ => todo!(),
        //}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_fen_works() {
        assert_eq!(
            Position::from_fen(Position::new_fen()),
            Position {
                board: INITIAL_FEN.split(' ').next().unwrap().to_string(),
                active_player: Player::P1,
                p1_owned: None,
                p1_controlled: HashSet::new(),
                p2_owned: None,
                p2_controlled: HashSet::new(),
                ply: 0,
            },
        );
    }

    #[test]
    fn to_fen_works() {
        assert_eq!(
            Position {
                board: INITIAL_FEN.split(' ').next().unwrap().to_string(),
                active_player: Player::P1,
                p1_owned: None,
                p1_controlled: HashSet::new(),
                p2_owned: None,
                p2_controlled: HashSet::new(),
                ply: 0,
            }.to_fen(),
            Position::new_fen(),
        );

        assert_eq!(
            Position {
                board: INITIAL_FEN.split(' ').next().unwrap().to_string(),
                active_player: Player::P1,
                p1_owned: Some(Color::White),
                p1_controlled: HashSet::from([Color::Green]),
                p2_owned: Some(Color::Black),
                p2_controlled: HashSet::from([Color::Yellow, Color::Pink]),
                ply: 0,
            }.to_fen(),
            "aqabvrvnbrbnbbbqbkbbbnbrynyrsbsq/aranvpvpbpbpbpbpbpbpbpbpypypsnsr/nbnp12opob/nqnp12opoq/crcp12rprr/cncp12rprn/gbgp12pppb/gqgp12pppq/yqyp12vpvq/ybyp12vpvb/onop12npnn/orop12npnr/rqrp12cpcq/rbrp12cpcb/srsnppppwpwpwpwpwpwpwpwpgpgpanar/sqsbprpnwrwnwbwqwkwbwnwrgngrabaq 1 W G B PY 0".to_string(),
        );
    }

    #[test]
    fn play_move_updates_fields() {
        let mut pos = Position::new();
        let new_move = Move::from_san("WNb1c3");
        let mut new_pos = pos.play_move(&new_move).unwrap();

        assert_eq!(new_pos.active_player, Player::P2);
        assert_eq!(new_pos.ply, 1);
    }
}
