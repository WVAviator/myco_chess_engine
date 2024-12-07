use anyhow::{anyhow, bail, Context};

use super::{
    board::Board,
    castling_rights::CastlingRights,
    constants::{A_FILE, EIGHTH_RANK, FIRST_RANK, H_FILE, SECOND_RANK, SEVENTH_RANK},
    moves::{algebraic_to_u64, LongAlgebraicMove},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    board: Board,
    turn: Turn,
    castling_rights: CastlingRights,
    en_passant: u64,
    halfmove_clock: u32,
    fullmove_number: u32,
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

        Ok(Self {
            board,
            turn,
            castling_rights,
            en_passant,
            halfmove_clock,
            fullmove_number,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Turn {
    White,
    Black,
}

impl Game {
    pub fn calculate_white_pawn_moves(&self) -> Vec<LongAlgebraicMove> {
        let mut moves = Vec::new();

        let white_pawns = self.board.white_pawns;
        let occupied = self.board.occupied();
        let black_pieces = self.board.black_pieces();

        // Any pawns not on the seventh rank (promotions) can be advanced forward to an empty square
        let single_advance = ((white_pawns & !SEVENTH_RANK) << 8) & !occupied;
        // Any pawns on the second rank can be advanced twice if both advance squares are empty
        let double_advance = ((((white_pawns & SECOND_RANK) << 8) & !occupied) << 8) & !occupied;
        // Any pawns not on the a file can take any black pieces diagonally to the left
        let take_left =
            ((white_pawns & !A_FILE & !SEVENTH_RANK) << 7) & (black_pieces | self.en_passant);
        // Any pawns not on the h file can take any black pieces diagonally to the right
        let take_right =
            ((white_pawns & !H_FILE & !SEVENTH_RANK) << 9) & (black_pieces | self.en_passant);
        // Pawns on the seventh rank can promote if not blocked
        let promotion_advance = ((white_pawns & SEVENTH_RANK) << 8) & !occupied;
        let promotion_take_left = ((white_pawns & !A_FILE & SEVENTH_RANK) << 7) & black_pieces;
        let promotion_take_right = ((white_pawns & !H_FILE & SEVENTH_RANK) << 9) & black_pieces;

        moves.extend(backtrack_moves(single_advance, |lsb| lsb >> 8));
        moves.extend(backtrack_moves(double_advance, |lsb| lsb >> 16));
        moves.extend(backtrack_moves(take_left, |lsb| lsb >> 7));
        moves.extend(backtrack_moves(take_right, |lsb| lsb >> 9));

        moves.extend(backtrack_moves_promotion(promotion_advance, |lsb| lsb >> 8));
        moves.extend(backtrack_moves_promotion(promotion_take_left, |lsb| {
            lsb >> 7
        }));
        moves.extend(backtrack_moves_promotion(promotion_take_right, |lsb| {
            lsb >> 9
        }));

        moves
    }

    pub fn calculate_black_pawn_moves(&self) -> Vec<LongAlgebraicMove> {
        let mut moves = Vec::new();

        let black_pawns = self.board.black_pawns;
        let occupied = self.board.occupied();
        let white_pieces = self.board.white_pieces();

        // Any pawns not on the seventh rank (promotions) can be advanced forward to an empty square
        let single_advance = ((black_pawns & !SECOND_RANK) >> 8) & !occupied;
        // Any pawns on the second rank can be advanced twice if both advance squares are empty
        let double_advance = ((((black_pawns & SEVENTH_RANK) >> 8) & !occupied) >> 8) & !occupied;
        // Any pawns not on the a file can take any black pieces diagonally to the left
        let take_left =
            ((black_pawns & !A_FILE & !SECOND_RANK) >> 7) & (white_pieces | self.en_passant);
        // Any pawns not on the h file can take any black pieces diagonally to the right
        let take_right =
            ((black_pawns & !H_FILE & !SECOND_RANK) >> 9) & (white_pieces | self.en_passant);
        // Pawns on the seventh rank can promote if not blocked
        let promotion_advance = ((black_pawns & SECOND_RANK) >> 8) & !occupied;
        let promotion_take_left = ((black_pawns & !A_FILE & SECOND_RANK) >> 7) & white_pieces;
        let promotion_take_right = ((black_pawns & !H_FILE & SECOND_RANK) >> 9) & white_pieces;

        moves.extend(backtrack_moves(single_advance, |lsb| lsb << 8));
        moves.extend(backtrack_moves(double_advance, |lsb| lsb << 16));
        moves.extend(backtrack_moves(take_left, |lsb| lsb << 7));
        moves.extend(backtrack_moves(take_right, |lsb| lsb << 9));

        moves.extend(backtrack_moves_promotion(promotion_advance, |lsb| lsb << 8));
        moves.extend(backtrack_moves_promotion(promotion_take_left, |lsb| {
            lsb << 7
        }));
        moves.extend(backtrack_moves_promotion(promotion_take_right, |lsb| {
            lsb << 9
        }));

        moves
    }

    pub fn calculate_pawn_moves(&self) -> Vec<LongAlgebraicMove> {
        match self.turn {
            Turn::White => self.calculate_white_pawn_moves(),
            Turn::Black => self.calculate_black_pawn_moves(),
        }
    }

    pub fn calculate_king_moves(&self) -> Vec<LongAlgebraicMove> {
        let king_position = match self.turn {
            Turn::White => self.board.white_king,
            Turn::Black => self.board.black_king,
        };
        let own_pieces = match self.turn {
            Turn::White => self.board.white_pieces(),
            Turn::Black => self.board.black_pieces(),
        };

        let w = (king_position & !A_FILE) >> 1;
        let nw = (king_position & !A_FILE & !EIGHTH_RANK) << 7;
        let n = (king_position & !EIGHTH_RANK) << 8;
        let ne = (king_position & !H_FILE & !EIGHTH_RANK) << 9;
        let e = (king_position & !H_FILE) << 1;
        let se = (king_position & !H_FILE & !FIRST_RANK) >> 7;
        let s = (king_position & !FIRST_RANK) >> 8;
        let sw = (king_position & !A_FILE & !FIRST_RANK) >> 9;

        let castling_dest = self
            .castling_rights
            .castling_positions(&self.turn, self.board.occupied());

        let dest_squares = (w | nw | n | ne | e | se | s | sw | castling_dest) & !own_pieces;

        create_moves(dest_squares, king_position)
    }
}

pub fn create_moves(dest_squares: u64, origin: u64) -> Vec<LongAlgebraicMove> {
    let mut bb = dest_squares;
    let mut moves = Vec::new();

    while bb != 0 {
        let lsb = bb & (!bb + 1);
        let lmove = LongAlgebraicMove::new(origin, lsb);
        moves.push(lmove);
        bb &= bb - 1;
    }

    moves
}

pub fn backtrack_moves<F>(dest_squares: u64, calculate_origin: F) -> Vec<LongAlgebraicMove>
where
    F: Fn(u64) -> u64,
{
    let mut bb = dest_squares;
    let mut moves = Vec::new();

    while bb != 0 {
        let lsb = bb & (!bb + 1); // Extract the least significant bit
        let origin = calculate_origin(lsb);
        let lmove = LongAlgebraicMove::new(origin, lsb);
        moves.push(lmove);
        bb &= bb - 1; // Clear the least significant bit
    }

    moves
}

pub fn backtrack_moves_promotion<F>(
    dest_squares: u64,
    calculate_origin: F,
) -> Vec<LongAlgebraicMove>
where
    F: Fn(u64) -> u64,
{
    let mut bb = dest_squares;
    let mut moves = Vec::new();

    while bb != 0 {
        let lsb = bb & (!bb + 1);
        let origin = calculate_origin(lsb);
        moves.extend(LongAlgebraicMove::new_promotion(origin, lsb));
        bb &= bb - 1;
    }

    moves
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_fen_starting_position() {
        let game = Game::new_default();
        assert_eq!(
            game,
            Game {
                board: Board::new_default(),
                turn: Turn::White,
                castling_rights: CastlingRights::from_fen("KQkq").unwrap(),
                en_passant: 0,
                halfmove_clock: 0,
                fullmove_number: 1,
            }
        );
    }

    #[test]
    fn calculates_white_pawn_moves() {
        let game = Game::from_fen("1qB2bkr/PPp2p1p/6p1/2r1b1RP/4pPP1/3B4/2PPP3/NQNR2K1 b - - 0 1")
            .unwrap();
        let moves = game.calculate_white_pawn_moves();

        assert_eq!(moves.len(), 15);

        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("a7a8q").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("a7a8r").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("a7a8b").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("a7a8n").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("a7b8q").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("a7b8n").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("a7b8r").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("a7b8b").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("c2c3").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("c2c4").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e2e3").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("f4f5").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("f4e5").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("h5h6").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("h5g6").unwrap()));
    }

    #[test]
    fn calculates_black_pawn_moves() {
        let game = Game::from_fen("8/1ppp4/1P2p3/2B2k2/2K5/8/5p2/6N1 w - - 0 1").unwrap();
        let moves = game.calculate_black_pawn_moves();

        assert_eq!(moves.len(), 13);

        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("f2f1q").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("f2f1r").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("f2f1b").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("f2f1n").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("f2g1q").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("f2g1r").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("f2g1b").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("f2g1n").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e6e5").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("d7d6").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("d7d5").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("c7c6").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("c7b6").unwrap()));
    }

    #[test]
    fn calculates_black_pawn_moves_en_passant() {
        let game = Game::from_fen("8/8/8/5k2/2K1pP2/8/8/8 b - f3 0 1").unwrap();
        let moves = game.calculate_black_pawn_moves();

        assert_eq!(moves.len(), 2);

        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e4f3").unwrap()));
    }

    #[test]
    fn calculate_simple_king_moves() {
        let game = Game::from_fen("8/6k1/8/8/8/1n6/KP6/8 w - - 0 1").unwrap();
        let moves = game.calculate_king_moves();

        assert_eq!(moves.len(), 4);

        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("a2a3").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("a2b3").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("a2b1").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("a2a1").unwrap()));
    }

    #[test]
    fn calculate_king_moves_castles_white() {
        let game =
            Game::from_fen("rn1qk1r1/pbpp1ppp/1p6/2b1p3/4P3/1PNP3N/PBPQBnPP/R3K2R w KQq - 0 1")
                .unwrap();
        let moves = game.calculate_king_moves();

        assert_eq!(moves.len(), 5);

        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e1d1").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e1f1").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e1f2").unwrap()));

        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e1g1").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e1c1").unwrap()));
    }

    #[test]
    fn calculate_king_moves_castles_black_forfeit() {
        let game =
            Game::from_fen("rn1qk1r1/pbpp1ppp/1p6/2b1p3/4P3/1PNP3N/PBPQBnPP/R3K2R b KQq - 0 1")
                .unwrap();
        let moves = game.calculate_king_moves();

        assert_eq!(moves.len(), 2);

        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e8e7").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e8f8").unwrap()));
    }
}
