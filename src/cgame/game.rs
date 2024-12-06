use anyhow::{anyhow, bail, Context};

use super::{
    board::Board,
    castling_rights::CastlingRights,
    constants::{A_FILE, EIGHTH_RANK, H_FILE, SECOND_RANK, SEVENTH_RANK},
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
        let lsb = bb & (!bb + 1); // Extract the least significant bit
        let origin = calculate_origin(lsb);
        moves.extend(LongAlgebraicMove::new_promotion(origin, lsb));
        bb &= bb - 1; // Clear the least significant bit
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
        println!(
            "Moves: {}",
            moves
                .iter()
                .map(|m| format!("{}, ", m.to_algebraic().unwrap()))
                .collect::<String>()
        );
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
}
