use std::pin;

use anyhow::{anyhow, bail, Context};

use crate::cgame::moves::u64_to_algebraic;

use super::{
    board::Board,
    castling_rights::CastlingRights,
    constants::{
        ANTIDIAGONAL_MASKS, A_FILE, B_FILE, DIAGONAL_MASKS, EIGHTH_RANK, FIRST_RANK, G_FILE,
        H_FILE, SECOND_RANK, SEVENTH_RANK,
    },
    moves::{algebraic_to_u64, LongAlgebraicMove},
    raycast::{Direction, Raycast},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    board: Board,
    turn: Turn,
    castling_rights: CastlingRights,
    en_passant: u64,
    halfmove_clock: u32,
    fullmove_number: u32,

    pinned_pieces: u64,
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

        let mut game = Game {
            board,
            turn,
            castling_rights,
            en_passant,
            halfmove_clock,
            fullmove_number,
            pinned_pieces: 0,
        };

        game.pinned_pieces = game.calculate_pinned_pieces();

        Ok(game)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Turn {
    White,
    Black,
}

impl Game {
    pub fn calculate_pinned_pieces(&self) -> u64 {
        let mut pinned_pieces = 0;
        let king = match self.turn {
            Turn::White => self.board.white_king,
            Turn::Black => self.board.black_king,
        };
        let opponent_rooks = match self.turn {
            Turn::White => self.board.black_rooks | self.board.black_queens,
            Turn::Black => self.board.white_rooks | self.board.white_queens,
        };
        let opponent_bishops = match self.turn {
            Turn::White => self.board.black_bishops | self.board.black_queens,
            Turn::Black => self.board.white_bishops | self.board.white_queens,
        };
        let player_pieces = match self.turn {
            Turn::White => self.board.white_pieces(),
            Turn::Black => self.board.black_pieces(),
        };
        let opponent_pieces = match self.turn {
            Turn::White => self.board.black_pieces(),
            Turn::Black => self.board.white_pieces(),
        };

        let pinnable_raycast = Raycast::new(king, player_pieces, opponent_pieces);

        // Cast in rook and bishop directions from the king to the first player piece, or stop if hit an opponent piece
        // Then, from those player pieces, continue casting in the same direction detecting rook/bishop pieces - or stop
        // if hitting any other piece. Only rooks and bishops (incl queens) can pin.

        Direction::rook_directions().iter().for_each(|dir| {
            let pinnable_piece = pinnable_raycast.get_first_hit(dir);
            if pinnable_piece != 0 {
                let pin_blockers = self.board.occupied() & !opponent_rooks;
                let pinner_raycast = Raycast::new(pinnable_piece, opponent_rooks, pin_blockers);
                if pinner_raycast.get_first_hit(dir) != 0 {
                    pinned_pieces |= pinnable_piece;
                }
            }
        });

        Direction::bishop_directions().iter().for_each(|dir| {
            let pinnable_piece = pinnable_raycast.get_first_hit(dir);
            if pinnable_piece != 0 {
                let pin_blockers = self.board.occupied() & !opponent_bishops;
                let pinner_raycast = Raycast::new(pinnable_piece, opponent_bishops, pin_blockers);
                if pinner_raycast.get_first_hit(dir) != 0 {
                    pinned_pieces |= pinnable_piece;
                }
            }
        });

        pinned_pieces
    }

    pub fn calculate_white_pawn_moves(&self) -> Vec<LongAlgebraicMove> {
        let mut moves = Vec::new();

        let white_pawns = self.board.white_pawns & !self.pinned_pieces;
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

        let black_pawns = self.board.black_pawns & !self.pinned_pieces;
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

    pub fn calculate_knight_moves(&self) -> Vec<LongAlgebraicMove> {
        let mut moves = Vec::new();
        let knights = match self.turn {
            Turn::White => self.board.white_knights,
            Turn::Black => self.board.black_knights,
        } & !self.pinned_pieces;
        let own_pieces = match self.turn {
            Turn::White => self.board.white_pieces(),
            Turn::Black => self.board.black_pieces(),
        };

        let knight_dest_squares = get_knight_dest_squares(knights) & !own_pieces;

        let mut dest_squares = knight_dest_squares;
        while dest_squares != 0 {
            let lsb = dest_squares & (!dest_squares + 1);
            let mut possible_origins = get_knight_dest_squares(lsb) & knights;
            while possible_origins != 0 {
                let po_lsb = possible_origins & (!possible_origins + 1);
                let lmove = LongAlgebraicMove::new(po_lsb, lsb);
                moves.push(lmove);
                possible_origins &= possible_origins - 1;
            }
            dest_squares &= dest_squares - 1;
        }
        moves
    }

    pub fn calculate_rook_moves(&self) -> Vec<LongAlgebraicMove> {
        let mut moves = Vec::new();
        let rooks = match self.turn {
            Turn::White => self.board.white_rooks | self.board.white_queens,
            Turn::Black => self.board.black_rooks | self.board.black_queens,
        };
        let unpinned_rooks = rooks & !self.pinned_pieces;
        let own_pieces = match self.turn {
            Turn::White => self.board.white_pieces(),
            Turn::Black => self.board.black_pieces(),
        };
        let opponent_pieces = match self.turn {
            Turn::White => self.board.black_pieces(),
            Turn::Black => self.board.white_pieces(),
        };

        let raycast = Raycast::new(unpinned_rooks, opponent_pieces, own_pieces);

        Direction::rook_directions().iter().for_each(|dir| {
            let mut ray = raycast.get_full_ray(dir);
            while ray != 0 {
                let lsb = ray & (!ray + 1);
                let backtrack_raycast = Raycast::new(lsb, unpinned_rooks, 0);
                let origin = backtrack_raycast.get_first_hit(&dir.reversed());
                moves.push(LongAlgebraicMove::new(origin, lsb));
                ray &= ray - 1;
            }
        });

        // Pinned rooks can still move along the same axis as the king
        let pinned_rooks = rooks & self.pinned_pieces;
        let own_king = match self.turn {
            Turn::White => self.board.white_king,
            Turn::Black => self.board.black_king,
        };

        let mut bb = pinned_rooks;
        while bb != 0 {
            let lsb = bb & (!bb + 1);
            if own_king & get_occupied_files(lsb) != 0 {
                let raycast = Raycast::new(lsb, opponent_pieces, own_pieces);
                let north = raycast.get_full_ray(&Direction::N);
                let south = raycast.get_full_ray(&Direction::S);
                moves.extend(create_moves(north | south, lsb));
            }
            if own_king & get_occupied_ranks(lsb) != 0 {
                let raycast = Raycast::new(lsb, opponent_pieces, own_pieces);
                let west = raycast.get_full_ray(&Direction::W);
                let east = raycast.get_full_ray(&Direction::E);
                moves.extend(create_moves(west | east, lsb));
            }

            bb &= bb - 1;
        }

        moves
    }

    pub fn calculate_bishop_moves(&self) -> Vec<LongAlgebraicMove> {
        let mut moves = Vec::new();
        let bishops = match self.turn {
            Turn::White => self.board.white_bishops | self.board.white_queens,
            Turn::Black => self.board.black_bishops | self.board.black_queens,
        };
        let unpinned_bishops = bishops & !self.pinned_pieces;
        let own_pieces = match self.turn {
            Turn::White => self.board.white_pieces(),
            Turn::Black => self.board.black_pieces(),
        };
        let opponent_pieces = match self.turn {
            Turn::White => self.board.black_pieces(),
            Turn::Black => self.board.white_pieces(),
        };

        println!("Unpinned bishops: {}", unpinned_bishops);
        let raycast = Raycast::new(unpinned_bishops, opponent_pieces, own_pieces);

        Direction::bishop_directions().iter().for_each(|dir| {
            let mut ray = raycast.get_full_ray(dir);
            while ray != 0 {
                let lsb = ray & (!ray + 1);
                let backtrack_raycast = Raycast::new(lsb, unpinned_bishops, 0);
                let origin = backtrack_raycast.get_first_hit(&dir.reversed());
                moves.push(LongAlgebraicMove::new(origin, lsb));
                ray &= ray - 1;
            }
        });

        // Pinned bishops can still move along the same axis as the king
        let pinned_bishops = bishops & self.pinned_pieces;
        let own_king = match self.turn {
            Turn::White => self.board.white_king,
            Turn::Black => self.board.black_king,
        };

        println!("Pinned bishops: {}", pinned_bishops);
        let mut bb = pinned_bishops;
        while bb != 0 {
            let lsb = bb & (!bb + 1);
            if own_king & get_occupied_diagonals(lsb) != 0 {
                println!("LSB {} same diagonal as king.", lsb);
                let raycast = Raycast::new(lsb, opponent_pieces, own_pieces);
                let ne = raycast.get_full_ray(&Direction::NE);
                let sw = raycast.get_full_ray(&Direction::SW);
                moves.extend(create_moves(ne | sw, lsb));
            }
            if own_king & get_occupied_antidiagonals(lsb) != 0 {
                println!("LSB {} same antidiagonal as king.", lsb);
                let raycast = Raycast::new(lsb, opponent_pieces, own_pieces);
                let nw = raycast.get_full_ray(&Direction::NW);
                let se = raycast.get_full_ray(&Direction::SE);
                moves.extend(create_moves(nw | se, lsb));
            }

            bb &= bb - 1;
        }

        moves
    }
}

pub fn get_occupied_files(pieces: u64) -> u64 {
    let mut result = 0;
    let mut bb = pieces;

    while bb != 0 {
        let lsb = bb & (!bb + 1);
        let file = (lsb.trailing_zeros() % 8) as u64;
        result |= 0x0101010101010101 << file;
        bb &= bb - 1;
    }

    result
}

pub fn get_occupied_ranks(pieces: u64) -> u64 {
    let mut result = 0;
    let mut bb = pieces;

    while bb != 0 {
        let lsb = bb & (!bb + 1);
        let rank = (lsb.trailing_zeros() / 8) as u64;
        result |= 0xFF << (rank * 8);
        bb &= bb - 1;
    }

    result
}

pub fn get_occupied_diagonals(pieces: u64) -> u64 {
    let mut result = 0;
    let mut bb = pieces;

    while bb != 0 {
        let lsb = bb & (!bb + 1);
        let position = lsb.trailing_zeros() as i32;

        let rank = position / 8;
        let file = position % 8;
        let diag_index = file - rank + 7;

        result |= DIAGONAL_MASKS[diag_index as usize];

        bb &= bb - 1;
    }

    result
}

pub fn get_occupied_antidiagonals(pieces: u64) -> u64 {
    let mut result = 0;
    let mut bb = pieces;

    while bb != 0 {
        let lsb = bb & (!bb + 1);
        let position = lsb.trailing_zeros() as i32;

        let rank = position / 8;
        let file = position % 8;
        let adiag_index = file + rank;

        result |= ANTIDIAGONAL_MASKS[adiag_index as usize];
        bb &= bb - 1;
    }

    result
}

pub fn get_knight_dest_squares(knights: u64) -> u64 {
    let wnw = (knights & !A_FILE & !B_FILE & !EIGHTH_RANK) << 6;
    let nnw = (knights & !A_FILE & !SEVENTH_RANK & !EIGHTH_RANK) << 15;
    let nne = (knights & !H_FILE & !SEVENTH_RANK & !EIGHTH_RANK) << 17;
    let ene = (knights & !H_FILE & !G_FILE & !EIGHTH_RANK) << 10;
    let ese = (knights & !H_FILE & !G_FILE & !FIRST_RANK) >> 6;
    let sse = (knights & !H_FILE & !SECOND_RANK & !FIRST_RANK) >> 15;
    let ssw = (knights & !A_FILE & !SECOND_RANK & !FIRST_RANK) >> 17;
    let wsw = (knights & !A_FILE & !B_FILE & !FIRST_RANK) >> 10;

    wnw | nnw | nne | ene | ese | sse | ssw | wsw
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
                pinned_pieces: 0,
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

    #[test]
    fn calculate_knight_moves() {
        let game = Game::from_fen("6k1/3b4/2P2n2/1P6/3NP3/1b3PN1/2R1P3/1K5R w - - 0 1").unwrap();
        let moves = game.calculate_knight_moves();

        assert_eq!(moves.len(), 6);

        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("d4b3").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("d4e6").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("d4f5").unwrap()));

        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("g3h5").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("g3f5").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("g3f1").unwrap()));
    }

    #[test]
    fn identifies_pinned_pieces() {
        let game = Game::from_fen("k3r3/4p2b/6P1/8/qBB1K3/4N3/8/4r3 w - - 0 1").unwrap();
        let expected_pins = 0x400000100000; // e3 and g6
        let actual_pins = game.calculate_pinned_pieces();
        assert_eq!(actual_pins, expected_pins);
    }

    #[test]
    fn no_moves_for_pinned_pieces() {
        let game = Game::from_fen("k3r3/4p2b/6P1/8/qPP1K3/4N3/8/4r3 w - - 0 1").unwrap();

        let pawn_moves = game.calculate_pawn_moves();
        assert_eq!(pawn_moves.len(), 2);

        let knight_moves = game.calculate_knight_moves();
        assert_eq!(knight_moves.len(), 0);
    }

    #[test]
    fn calculates_basic_rook_moves() {
        let game = Game::from_fen("k7/8/3p4/3r3p/8/B2N4/qb6/7K b - - 0 1").unwrap();

        let moves = game.calculate_rook_moves();

        assert_eq!(moves.len(), 10);

        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("a2a1").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("a2a3").unwrap()));

        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("d5c5").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("d5b5").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("d5a5").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("d5e5").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("d5f5").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("d5g5").unwrap()));

        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("d5d4").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("d5d3").unwrap()));
    }

    #[test]
    fn calculates_basic_rook_moves_pinned() {
        let game = Game::from_fen("8/2p5/2kr2R1/1q5p/B7/3N4/1b6/7K b - - 0 1").unwrap();

        let moves = game.calculate_rook_moves();
        LongAlgebraicMove::print_list(&moves);

        assert_eq!(moves.len(), 3);

        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("d6e6").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("d6f6").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("d6g6").unwrap()));
    }

    #[test]
    fn calculates_basic_bishop_moves() {
        let game = Game::from_fen("1k6/6p1/3r4/1q1NB2p/4BR2/8/1b6/7K w - - 0 1").unwrap();

        let moves = game.calculate_bishop_moves();

        assert_eq!(moves.len(), 14);

        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e5d6").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e5f6").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e5g7").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e5d4").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e5c3").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e5b2").unwrap()));

        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e4f5").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e4g6").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e4h7").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e4f3").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e4g2").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e4d3").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e4c2").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e4b1").unwrap()));
    }

    #[test]
    fn calculates_bishop_moves_pinned() {
        let game = Game::from_fen("8/B3N1p1/3r4/2b4p/3kbR2/8/8/7K b - - 0 1").unwrap();

        let moves = game.calculate_bishop_moves();
        LongAlgebraicMove::print_list(&moves);

        assert_eq!(moves.len(), 2);

        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("c5b6").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("c5a7").unwrap()));
    }

    #[test]
    fn calculates_bishop_moves_opposite_pinned() {
        let game = Game::from_fen("8/B3N1p1/3r4/2b3Rp/3k4/4b3/8/6QK b - - 0 1").unwrap();

        let moves = game.calculate_bishop_moves();
        LongAlgebraicMove::print_list(&moves);

        assert_eq!(moves.len(), 4);

        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("c5b6").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("c5a7").unwrap()));

        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e3f2").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e3g1").unwrap()));
    }
}
