use std::collections::HashSet;

use crate::chessops::{Board, Color, Fen, Move, Piece, Player, Role};

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

    #[cfg(test)]
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

        if !self.board.is_legal_move(&new_move, &own_side, &other_side) {
            return Err(PlayError {});
        }

        self.update_board(new_move);

        self.update_controlled_armies(new_move);

        // Make sure we update the `active_player` after we're done updating the board, since the
        // logic is dependent on this field.
        self.active_player = self.active_player.next();
        self.ply += 1;
        Ok(self)
    }

    pub fn update_board(&mut self, move_: &Move) {
        self.board.by_square.remove(&move_.from);

        let role = if let Some(role) = &move_.promotion {
            if *role == Role::King {
                match self.active_player {
                    Player::P1 => {
                        // Remove existing King since we're promoting to a new King
                        self.board.remove_piece(Piece::new(
                            self.p1_owned.expect("p1_owned should not be None"),
                            Role::King,
                        ));

                        // If King is a different color, update our owned an controlled armies
                        self.p1_owned = Some(move_.color.clone());
                        self.p1_controlled.remove(&move_.color);
                    }
                    Player::P2 => {
                        // Remove existing King since we're promoting to a new King
                        self.board.remove_piece(Piece::new(
                            self.p2_owned.expect("p2_owned should not be None"),
                            Role::King,
                        ));

                        // If King is a different color, update our owned an controlled armies
                        self.p2_owned = Some(move_.color.clone());
                        self.p2_controlled.remove(&move_.color);
                    }
                }
            }

            role
        } else {
            &move_.role
        };

        let piece = Piece {
            color: move_.color.clone(),
            role: role.clone(),
        };

        self.board.by_square.insert(move_.to.clone(), piece);
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

    fn update_controlled_armies(&mut self, move_: &Move) {
        match self.active_player {
            Player::P1 => {
                if let Some(color) = move_.from.color() {
                    self.p1_controlled.remove(&color);
                }
                if let Some(color) = move_.to.color() {
                    if Some(color) != self.p2_owned {
                        self.p1_controlled.insert(color);
                    }
                }
            }
            Player::P2 => {
                if let Some(color) = move_.from.color() {
                    self.p2_controlled.remove(&color);
                }
                if let Some(color) = move_.to.color() {
                    if Some(color) != self.p1_owned {
                        self.p2_controlled.insert(color);
                    }
                }
            }
        }
    }

    pub fn defect_to(&mut self, color: Color) -> Result<(), PlayError> {
        match self.active_player {
            Player::P1 => {
                if !self.p1_controlled.contains(&color) {
                    return Err(PlayError {});
                }

                // Swap King
                let piece = Piece::new(self.p1_owned.unwrap(), Role::King);
                let square = self.board.find(&piece).unwrap();
                self.board.remove_piece(piece.clone());
                self.board
                    .insert_piece(square.clone(), Piece::new(color, Role::King));

                // Update armies
                self.p1_controlled.remove(&color);
                self.p1_owned = Some(color);
            }
            Player::P2 => {
                if !self.p2_controlled.contains(&color) {
                    return Err(PlayError {});
                }

                // Swap King
                let piece = Piece::new(self.p2_owned.unwrap(), Role::King);
                let square = self.board.find(&piece).unwrap();
                self.board.remove_piece(piece.clone());
                self.board
                    .insert_piece(square.clone(), Piece::new(color, Role::King));

                // Update armies
                self.p2_controlled.remove(&color);
                self.p2_owned = Some(color);
            }
        }

        self.active_player = self.active_player.next();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chessops::Square;

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

    #[test]
    fn defect_to_works() {
        let fen =
            String::from("08bk07/16/16/16/16/16/16/16/16/16/16/04wq11/16/16/16/08wk07 1 w n b - 0");
        let mut pos = Position::from_fen(fen);

        let _ = pos.defect_to(Color::Navy);

        assert_eq!(pos.p1_owned, Some(Color::Navy));
        assert!(pos.p1_controlled.is_empty());
        assert_eq!(
            pos.board.get(&Square::I1),
            Some(&Piece::new(Color::Navy, Role::King))
        );
        assert_eq!(pos.active_player, Player::P2);
    }
}
