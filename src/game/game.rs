use anyhow::{anyhow, bail, Context};

use crate::{
    movegen::MoveGen,
    moves::{
        common::{algebraic_to_u64, u64_to_algebraic},
        simple_move::SimpleMove,
    },
};

use super::{board::Board, castling_rights::CastlingRights};

#[derive(Debug, Eq, Clone, PartialEq, Copy)]
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
        Game::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
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
            .map(SimpleMove::from_algebraic)
            .collect::<Result<Vec<SimpleMove>, anyhow::Error>>()?;

        let mut game = Game::new_default();

        for lmove in moves {
            game = game.apply_move(&lmove);
        }

        Ok(game)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Turn {
    White = 0,
    Black = 1,
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
        self.generate_vision(&self.turn.other())[6] & self.board.king(&self.turn) != 0
    }

    pub fn apply_move(&self, lmove: &SimpleMove) -> Game {
        let mut new_game = *self;

        new_game.update_halfmove_clock(lmove);
        new_game.update_en_passant(lmove);
        new_game.castling_rights.forfeit(lmove.orig);

        // Note: promotions handled by the board apply_move function

        new_game.update_turn();
        new_game.board.apply_move(lmove);

        new_game
    }

    #[inline(always)]
    fn update_halfmove_clock(&mut self, lmove: &SimpleMove) {
        let is_pawn_move = lmove.orig & (self.board.white[0] | self.board.black[0]) != 0;
        let is_enpassant = lmove.dest & self.en_passant != 0 && is_pawn_move;
        let is_capture = lmove.dest & self.board.all() != 0 || is_enpassant;

        if is_pawn_move || is_capture {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }
    }

    #[inline(always)]
    fn update_en_passant(&mut self, lmove: &SimpleMove) {
        self.en_passant = (((lmove.orig & self.board.white[0]) << 8) & (lmove.dest >> 8))
            | (((lmove.orig & self.board.black[0]) >> 8) & (lmove.dest << 8));
    }

    #[inline(always)]
    fn update_turn(&mut self) {
        // Advance only after black's turn
        self.fullmove_number += self.turn as u32;
        self.turn = self.turn.other();
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
            Game::from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1").unwrap();
        let position3 =
            Game::from_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 2")
                .unwrap();

        let lmove1 = SimpleMove::from_algebraic("e2e4").unwrap();
        let lmove2 = SimpleMove::from_algebraic("e7e5").unwrap();

        let mut game = position1;
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
            Game::from_fen("rnbqk2r/ppppnppp/8/2b1p3/4P3/3B1N2/PPPP1PPP/RNBQ1RK1 b kq - 5 4")
                .unwrap();
        let position3 =
            Game::from_fen("rnbqkr2/ppppnppp/8/2b1p3/4P3/3B1N2/PPPP1PPP/RNBQ1RK1 w q - 6 5")
                .unwrap();
        let lmove1 = SimpleMove::from_algebraic("e1g1").unwrap();
        let lmove2 = SimpleMove::from_algebraic("h8f8").unwrap();

        let mut game = position1;
        assert_eq!(game, position1);

        game = game.apply_move(&lmove1);
        assert_eq!(game, position2);

        game = game.apply_move(&lmove2);
        assert_eq!(game, position3);
    }

    #[test]
    fn properly_applies_queenside_castles() {
        let position1 = Game::from_fen("r3k2r/8/3p4/8/8/8/8/R3K2R w KQkq - 0 1").unwrap();
        let position2 = Game::from_fen("r3k2r/8/3p4/8/8/8/8/2KR3R b kq - 1 1").unwrap();
        let position3 = Game::from_fen("2kr3r/8/3p4/8/8/8/8/2KR3R w - - 2 2").unwrap();
        let lmove1 = SimpleMove::from_algebraic("e1c1").unwrap();
        let lmove2 = SimpleMove::from_algebraic("e8c8").unwrap();

        let mut game = position1;
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
            Game::from_fen("r3k2r/1p1nNpp1/p2p3p/4p3/4PP1R/1N2B3/PPPQ4/2KR1B2 b kq - 0 18")
                .unwrap();
        let moves_game = Game::from_uci_startpos("e2e4 c7c5 g1f3 d7d6 d2d4 c5d4 f3d4 g8f6 b1c3 a7a6 c1e3 e7e5 d4b3 c8e6 h2h3 b8d7 g2g4 h7h6 d1d2 f8e7 e1c1 e7f8 f2f4 e6g4 h3g4 d8e7 g4g5 e7d8 g5f6 d8f6 c3d5 f6h4 h1h4 f8e7 d5e7").unwrap();

        println!("moves_game: {}", moves_game.to_fen());

        assert_eq!(fen_game, moves_game);
    }

    #[test]
    fn correct_uci_startpos() {
        let moves_list =
            "g1f3 d7d5 b1c3 g8f6 d2d4 c7c5 c1g5 f6e4 e2e3 d8a5 f1b5 b8d7 d1b1 e4c3 b2c3 a5c3";
        let fen_str = "r1b1kb1r/pp1npppp/8/1Bpp2B1/3P4/2q1PN2/P1P2PPP/RQ2K2R w KQkq - 0 9";

        assert_eq!(
            Game::from_uci_startpos(moves_list).unwrap(),
            Game::from_fen(fen_str).unwrap()
        )
    }

}
