use std::collections::HashSet;

use crate::chessops::{Board, Color, Fen, Move, Piece, Player};

const INITIAL_FEN: &'static str = "aqabvrvnbrbnbbbqbkbbbnbrynyrsbsq/aranvpvpbpbpbpbpbpbpbpbpypypsnsr/nbnp12opob/nqnp12opoq/crcp12rprr/cncp12rprn/gbgp12pppb/gqgp12pppq/yqyp12vpvq/ybyp12vpvb/onop12npnn/orop12npnr/rqrp12cpcq/rbrp12cpcb/srsnppppwpwpwpwpwpwpwpwpgpgpanar/sqsbprpnwrwnwbwqwkwbwnwrgngrabaq 1 - - - - 0";

#[derive(Debug)]
pub struct PlayError {}

#[derive(Debug, PartialEq)]
pub struct Position {
    board: Board,
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
        let (board, active_player, p1_owned, p1_controlled, p2_owned, p2_controlled, ply) =
            Fen::parse(&fen);

        Position {
            board: board,
            active_player: active_player,
            p1_owned: p1_owned,
            p1_controlled: p1_controlled,
            p2_owned: p2_owned,
            p2_controlled: p2_controlled,
            ply,
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

        let mut p1_controlled = self
            .p1_controlled
            .iter()
            .map(|x| x.to_char().clone())
            .collect::<Vec<char>>();
        let mut p2_controlled = self
            .p2_controlled
            .iter()
            .map(|x| x.to_char().clone())
            .collect::<Vec<char>>();
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
            Fen::from_board(&self.board),
            self.active_player.to_int(),
            p1_owned,
            p1_controlled_computed,
            p2_owned,
            p2_controlled_computed,
            self.ply,
        )
        .to_string()
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
        } else if !self.does_color_belong_to_user(&new_move.color) {
            // Check if user owns or controls this army
            return Err(PlayError {});
        }

        // Collect any colors not controlled by the opponent
        // This effectively is our own side, meaning, we can't capture
        // these color pieces.
        let other_side = match self.active_player {
            Player::P1 => {
                let mut other_side_colors = self.p2_controlled.clone();
                if let Some(c) = &self.p2_owned {
                    other_side_colors.insert(c.clone());
                }
                other_side_colors
            }
            Player::P2 => {
                let mut other_side_colors = self.p1_controlled.clone();
                if let Some(c) = &self.p1_owned {
                    other_side_colors.insert(c.clone());
                }
                other_side_colors
            }
        };

        let own_side = {
            let mut own_colors = HashSet::new();
            let all_colors = HashSet::from(Color::all());
            let tmp = all_colors.difference(&other_side);
            for c in tmp {
                own_colors.insert(c.clone());
            }
            own_colors
        };

        if !self.board.is_legal_move(&new_move, &own_side) {
            return Err(PlayError {});
        }

        // Update Board
        // Remove piece on starting square
        self.board.by_square.remove(&new_move.from);

        // Add piece on ending square
        let piece = Piece {
            color: new_move.color.clone(),
            role: new_move.role.clone(),
        };
        self.board.by_square.insert(new_move.to.clone(), piece);

        self.active_player = self.active_player.next();
        self.ply += 1;
        Ok(self)
    }

    pub fn accept_first_move(&mut self) -> &Self {
        self.active_player = Player::P1;
        self.p1_owned = Some(Color::Black);
        self.p2_owned = Some(Color::White);

        self
    }

    pub fn reject_first_move(&mut self) -> &Self {
        self.active_player = Player::P2;
        self.p1_owned = Some(Color::White);
        self.p2_owned = Some(Color::Black);

        self
    }

    fn does_color_belong_to_user(&self, color: &Color) -> bool {
        match self.active_player {
            Player::P1 => {
                if *color == self.p1_owned.clone().unwrap() {
                    true
                } else if self.p1_controlled.contains(color) {
                    true
                } else {
                    false
                }
            }
            Player::P2 => {
                if *color == self.p2_owned.clone().unwrap() {
                    true
                } else if self.p2_controlled.contains(color) {
                    true
                } else {
                    false
                }
            }
        }
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
                board: Fen::to_board(INITIAL_FEN.split(' ').next().unwrap()),
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
                board: Fen::to_board(INITIAL_FEN.split(' ').next().unwrap()),
                active_player: Player::P1,
                p1_owned: None,
                p1_controlled: HashSet::new(),
                p2_owned: None,
                p2_controlled: HashSet::new(),
                ply: 0,
            }
            .to_fen(),
            Position::new_fen(),
        );

        assert_eq!(
            Position {
                board: Fen::to_board(INITIAL_FEN.split(' ').next().unwrap()),
                active_player: Player::P1,
                p1_owned: Some(Color::White),
                p1_controlled: HashSet::from([Color::Green]),
                p2_owned: Some(Color::Black),
                p2_controlled: HashSet::from([Color::Yellow, Color::Pink]),
                ply: 0,
            }.to_fen(),
            "aqabvrvnbrbnbbbqbkbbbnbrynyrsbsq/aranvpvpbpbpbpbpbpbpbpbpypypsnsr/nbnp12opob/nqnp12opoq/crcp12rprr/cncp12rprn/gbgp12pppb/gqgp12pppq/yqyp12vpvq/ybyp12vpvb/onop12npnn/orop12npnr/rqrp12cpcq/rbrp12cpcb/srsnppppwpwpwpwpwpwpwpwpgpgpanar/sqsbprpnwrwnwbwqwkwbwnwrgngrabaq 1 w g b py 0".to_string(),
        );
    }

    #[test]
    fn play_move_updates_fields() {
        let mut pos = Position::new();
        let new_move = Move::from_san("WNf01g03");
        let new_pos = pos.play_move(&new_move).unwrap();

        assert_eq!(new_pos.active_player, Player::P2);
        assert_eq!(new_pos.ply, 1);
    }
}
