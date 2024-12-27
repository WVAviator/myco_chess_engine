use std::hash::Hash;

use anyhow::{anyhow, bail, Context};

use crate::{
    movegen::MoveGen,
    moves::{
        common::{algebraic_to_u64, u64_to_algebraic},
        simple_move::SimpleMove,
    },
};

use super::{
    board::Board,
    castling_rights::CastlingRights,
    constants::{
        FIFTH_RANK, FOURTH_RANK, KING_START_POSITIONS, ROOK_START_POSITIONS, SECOND_RANK,
        SEVENTH_RANK,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    pub board: Board,
    pub turn: Turn,
    pub castling_rights: CastlingRights,
    pub en_passant: u64,
    pub halfmove_clock: u32,
    pub fullmove_number: u32,
}

impl Game {
    pub fn new_default() -> Self {
        let game =
            Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

        game
    }
    pub fn from_fen(fen_str: &str) -> Result<Self, anyhow::Error> {
        let mut fen_iter = fen_str.split(" ");
        let board = Board::from_fen(
            fen_iter
                .next()
                .ok_or(anyhow!("Invalid FEN string: {}", fen_str))?,
        )?;
        let turn = match fen_iter
            .next()
            .ok_or(anyhow!("Invalid FEN string: {}", fen_str))?
        {
            "w" => Turn::White,
            "b" => Turn::Black,
            _ => bail!(
                "Expected 'w' or 'b' at position 2 in FEN string: {}",
                fen_str
            ),
        };
        let castling_rights = CastlingRights::from_fen(
            fen_iter
                .next()
                .ok_or(anyhow!("Invalid FEN string: {}", fen_str))?,
        )?;
        let en_passant = match fen_iter
            .next()
            .ok_or(anyhow!("Invalid FEN string: {}", fen_str))?
        {
            "-" => 0,
            an => algebraic_to_u64(an)?,
        };
        let halfmove_clock = fen_iter
            .next()
            .ok_or(anyhow!("Invalid FEN string: {}", fen_str))?
            .parse()
            .context(anyhow!(
                "Expected numeric value for halfmove clock at position 5: {}",
                fen_str
            ))?;
        let fullmove_number = fen_iter
            .next()
            .ok_or(anyhow!("Invalid FEN string: {}", fen_str))?
            .parse()
            .context(anyhow!(
                "Expected numeric value for fullmove number at position 5: {}",
                fen_str
            ))?;

        Ok(Game {
            board,
            turn,
            castling_rights,
            en_passant,
            halfmove_clock,
            fullmove_number,
        })
    }

    pub fn to_fen(&self) -> String {
        let board_str = self.board.to_fen();
        let turn_str = match self.turn {
            Turn::White => "w",
            Turn::Black => "b",
        };
        let castling_rights_str = self.castling_rights.to_fen();
        let en_passant_str = match self.en_passant {
            0 => String::from("-"),
            value => u64_to_algebraic(value).unwrap(),
        };
        let halfmove_clock_str = self.halfmove_clock.to_string();
        let fullmove_number_str = self.fullmove_number.to_string();

        format!(
            "{} {} {} {} {} {}",
            board_str,
            turn_str,
            castling_rights_str,
            en_passant_str,
            halfmove_clock_str,
            fullmove_number_str
        )
    }

    pub fn from_uci_startpos(moves_list: &str) -> Result<Self, anyhow::Error> {
        let moves: Vec<SimpleMove> = moves_list
            .split(' ')
            .map(|m| SimpleMove::from_algebraic(m))
            .into_iter()
            .collect::<Result<Vec<SimpleMove>, anyhow::Error>>()?;

        let mut game = Game::new_default();

        for lmove in moves {
            game = game.apply_move(&lmove);
        }

        Ok(game)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Turn {
    White,
    Black,
}

impl Turn {
    pub fn other(&self) -> Turn {
        match self {
            Turn::White => Turn::Black,
            Turn::Black => Turn::White,
        }
    }
}

impl Game {
    pub fn king_in_check(&self) -> bool {
        self.generate_vision(&self.turn.other()) & self.board.king(&self.turn) != 0
    }

    pub fn is_checkmate(&self) -> bool {
        self.king_in_check() && self.generate_legal_moves().is_empty()
    }

    pub fn is_stalemate(&self) -> bool {
        !self.king_in_check() && self.generate_legal_moves().is_empty()
    }

    pub fn apply_move(&self, lmove: &SimpleMove) -> Game {
        let mut new_game = self.clone();

        // Handling enpassant and halfmove clock
        let is_pawn_move = lmove.orig
            & match new_game.turn {
                Turn::White => new_game.board.white[0],
                Turn::Black => new_game.board.black[0],
            }
            != 0;
        let is_enpassant = lmove.dest & new_game.en_passant != 0 && is_pawn_move;
        let is_capture = lmove.dest
            & match new_game.turn {
                Turn::White => new_game.board.black_pieces(),
                Turn::Black => new_game.board.white_pieces(),
            }
            != 0
            || is_enpassant;

        if is_pawn_move || is_capture {
            new_game.halfmove_clock = 0;
        } else {
            new_game.halfmove_clock += 1;
        }

        let is_pawn_double_advance = is_pawn_move
            && lmove.orig & (SECOND_RANK | SEVENTH_RANK) != 0
            && lmove.dest & (FOURTH_RANK | FIFTH_RANK) != 0;

        if is_pawn_double_advance {
            match new_game.turn {
                Turn::White => new_game.en_passant = lmove.orig << 8,
                Turn::Black => new_game.en_passant = lmove.orig >> 8,
            }
        } else {
            new_game.en_passant = 0;
        }

        // Handling castling
        let is_rook_move = lmove.orig & ROOK_START_POSITIONS != 0;
        let is_king_move = lmove.orig & KING_START_POSITIONS != 0;

        if is_rook_move {
            if lmove.orig == 0x1 {
                new_game
                    .castling_rights
                    .unset(CastlingRights::WHITE_QUEENSIDE);
            } else if lmove.orig == 0x80 {
                new_game
                    .castling_rights
                    .unset(CastlingRights::WHITE_KINGSIDE);
            } else if lmove.orig == 0x100000000000000 {
                new_game
                    .castling_rights
                    .unset(CastlingRights::BLACK_QUEENSIDE);
            } else if lmove.orig == 0x8000000000000000 {
                new_game
                    .castling_rights
                    .unset(CastlingRights::BLACK_KINGSIDE);
            }
        }

        if is_king_move {
            if lmove.orig == 0x10 {
                new_game
                    .castling_rights
                    .unset(CastlingRights::WHITE_KINGSIDE);
                new_game
                    .castling_rights
                    .unset(CastlingRights::WHITE_QUEENSIDE);
            } else if lmove.orig == 0x1000000000000000 {
                new_game
                    .castling_rights
                    .unset(CastlingRights::BLACK_KINGSIDE);
                new_game
                    .castling_rights
                    .unset(CastlingRights::BLACK_QUEENSIDE);
            }
        }

        // Note: promotions handled by the board apply_move function

        // Advance turn
        new_game.turn = new_game.turn.other();
        new_game.fullmove_number += 1;

        // Complete move
        new_game.board.apply_move(lmove);

        new_game
    }
}

impl Hash for Game {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.board.hash(state);
        self.castling_rights.hash(state);
        self.en_passant.hash(state);
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn round_trip_fen_conversion() {
        let fen_str = String::from("3R1n1k/1B4pp/1p6/5p2/p7/4P1P1/PP3P1P/RN4K1 b - - 0 48");
        let game = Game::from_fen(&fen_str).unwrap();
        let result_fen = game.to_fen();
        assert_eq!(result_fen, fen_str);
    }

    #[test]
    fn properly_applies_move_pawn_double_advance() {
        let position1 =
            Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
        let position2 =
            Game::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 2").unwrap();
        let position3 =
            Game::from_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 3")
                .unwrap();

        let lmove1 = SimpleMove::from_algebraic("e2e4").unwrap();
        let lmove2 = SimpleMove::from_algebraic("e7e5").unwrap();

        let mut game = position1.clone();
        assert_eq!(game, position1);

        game = game.apply_move(&lmove1);
        assert_eq!(game, position2);

        game = game.apply_move(&lmove2);
        assert_eq!(game, position3);
    }

    #[test]
    fn properly_applies_move_castles() {
        let position1 =
            Game::from_fen("rnbqk2r/ppppnppp/8/2b1p3/4P3/3B1N2/PPPP1PPP/RNBQK2R w KQkq - 4 4")
                .unwrap();
        let position2 =
            Game::from_fen("rnbqk2r/ppppnppp/8/2b1p3/4P3/3B1N2/PPPP1PPP/RNBQ1RK1 b kq - 5 5")
                .unwrap();
        let position3 =
            Game::from_fen("rnbqkr2/ppppnppp/8/2b1p3/4P3/3B1N2/PPPP1PPP/RNBQ1RK1 w q - 6 6")
                .unwrap();
        let lmove1 = SimpleMove::from_algebraic("e1g1").unwrap();
        let lmove2 = SimpleMove::from_algebraic("h8f8").unwrap();

        let mut game = position1.clone();
        assert_eq!(game, position1);

        game = game.apply_move(&lmove1);
        assert_eq!(game, position2);

        game = game.apply_move(&lmove2);
        assert_eq!(game, position3);
    }

    #[test]
    fn properly_applies_queenside_castles() {
        let position1 = Game::from_fen("r3k2r/8/3p4/8/8/8/8/R3K2R w KQkq - 0 1").unwrap();
        let position2 = Game::from_fen("r3k2r/8/3p4/8/8/8/8/2KR3R b kq - 1 2").unwrap();
        let position3 = Game::from_fen("2kr3r/8/3p4/8/8/8/8/2KR3R w - - 2 3").unwrap();
        let lmove1 = SimpleMove::from_algebraic("e1c1").unwrap();
        let lmove2 = SimpleMove::from_algebraic("e8c8").unwrap();

        let mut game = position1.clone();
        println!("position1: \n{}", game.board);
        assert_eq!(game, position1);

        game = game.apply_move(&lmove1);
        println!("position2: \n{}", game.board);
        assert_eq!(game, position2);

        game = game.apply_move(&lmove2);
        println!("position3: \n{}", game.board);
        assert_eq!(game, position3);
    }

    #[test]
    fn calculates_from_moves_correctly_advanced_position() {
        let fen_game =
            Game::from_fen("r3k2r/1p1nNpp1/p2p3p/4p3/4PP1R/1N2B3/PPPQ4/2KR1B2 b kq - 0 36")
                .unwrap();
        let moves_game = Game::from_uci_startpos("e2e4 c7c5 g1f3 d7d6 d2d4 c5d4 f3d4 g8f6 b1c3 a7a6 c1e3 e7e5 d4b3 c8e6 h2h3 b8d7 g2g4 h7h6 d1d2 f8e7 e1c1 e7f8 f2f4 e6g4 h3g4 d8e7 g4g5 e7d8 g5f6 d8f6 c3d5 f6h4 h1h4 f8e7 d5e7").unwrap();

        println!("moves_game: {}", moves_game.to_fen());

        assert_eq!(fen_game, moves_game);
    }

    #[test]
    fn correct_uci_startpos() {
        let moves_list =
            "g1f3 d7d5 b1c3 g8f6 d2d4 c7c5 c1g5 f6e4 e2e3 d8a5 f1b5 b8d7 d1b1 e4c3 b2c3 a5c3";
        let fen_str = "r1b1kb1r/pp1npppp/8/1Bpp2B1/3P4/2q1PN2/P1P2PPP/RQ2K2R w KQkq - 0 17";

        assert_eq!(
            Game::from_uci_startpos(&moves_list).unwrap(),
            Game::from_fen(&fen_str).unwrap()
        )
    }
}
