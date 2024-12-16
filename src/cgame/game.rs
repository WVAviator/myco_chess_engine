use std::pin;

use anyhow::{anyhow, bail, Context};

use crate::cgame::moves::u64_to_algebraic;

use super::{
    board::Board,
    castling_rights::CastlingRights,
    constants::{
        ANTIDIAGONAL_MASKS, A_FILE, B_FILE, DIAGONAL_MASKS, EIGHTH_RANK, FIFTH_RANK, FIRST_RANK,
        FOURTH_RANK, G_FILE, H_FILE, KING_START_POSITIONS, ROOK_START_POSITIONS, SECOND_RANK,
        SEVENTH_RANK,
    },
    moves::{algebraic_to_u64, LongAlgebraicMove},
    raycast::{Direction, Raycast},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
    pub board: Board,
    pub turn: Turn,
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

impl Turn {
    pub fn other(&self) -> Turn {
        match self {
            Turn::White => Turn::Black,
            Turn::Black => Turn::White,
        }
    }
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
        let king = self.board.king(&self.turn);
        let own_pieces = self.board.all_pieces(&self.turn);
        let opponent_vision = self.get_opponent_vision();

        let mut dest_squares = get_king_dest_squares(king);
        dest_squares &= !own_pieces;
        dest_squares &= !opponent_vision;
        // Cannot castle if king is curerntly in check
        if king & opponent_vision == 0 {
            dest_squares |= self
                .castling_rights
                .castling_positions(&self.turn, self.board.occupied() | opponent_vision);
        }

        create_moves(dest_squares, king)
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
        let rooks = self.board.rooks(&self.turn) | self.board.queens(&self.turn);
        let unpinned_rooks = rooks & !self.pinned_pieces;
        let own_pieces = self.board.all_pieces(&self.turn);
        let opponent_pieces = self.board.all_pieces(&self.turn.other());

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

        let mut bb = pinned_bishops;
        while bb != 0 {
            let lsb = bb & (!bb + 1);
            if own_king & get_occupied_diagonals(lsb) != 0 {
                let raycast = Raycast::new(lsb, opponent_pieces, own_pieces);
                let ne = raycast.get_full_ray(&Direction::NE);
                let sw = raycast.get_full_ray(&Direction::SW);
                moves.extend(create_moves(ne | sw, lsb));
            }
            if own_king & get_occupied_antidiagonals(lsb) != 0 {
                let raycast = Raycast::new(lsb, opponent_pieces, own_pieces);
                let nw = raycast.get_full_ray(&Direction::NW);
                let se = raycast.get_full_ray(&Direction::SE);
                moves.extend(create_moves(nw | se, lsb));
            }

            bb &= bb - 1;
        }

        moves
    }

    pub fn king_in_check(&self) -> bool {
        self.calculate_checking_king() != 0
    }

    pub fn calculate_checking_king(&self) -> u64 {
        let king = self.board.king(&self.turn);

        let own_pieces = self.board.all_pieces(&self.turn);
        let opponent_pieces = self.board.all_pieces(&self.turn.other());

        let opponent_pawns = self.board.pawns(&self.turn.other());
        let opponent_knights = self.board.knights(&self.turn.other());
        let opponent_rooks =
            self.board.rooks(&self.turn.other()) | self.board.queens(&self.turn.other());
        let opponent_bishops =
            self.board.bishops(&self.turn.other()) | self.board.queens(&self.turn.other());

        let mut checking_pieces = 0;

        // Pawns diagonal from the king are checking the king
        checking_pieces |= opponent_pawns
            & match self.turn {
                Turn::White => (king << 7) | (king << 9),
                Turn::Black => (king >> 7) | (king >> 9),
            };

        // Knights at knight opposition from the king are checking the king
        checking_pieces |= opponent_knights & get_knight_dest_squares(king);

        let raycast = Raycast::new(king, opponent_pieces, own_pieces);

        // Rooks horizontal or vertical from the king with no obstacles
        Direction::rook_directions().iter().for_each(|dir| {
            checking_pieces |= opponent_rooks & raycast.get_first_hit(dir);
        });

        // Bishops diagonal from the king with no obstacles
        Direction::bishop_directions().iter().for_each(|dir| {
            checking_pieces |= opponent_bishops & raycast.get_first_hit(dir);
        });

        checking_pieces
    }

    pub fn get_white_vision(&self) -> u64 {
        Game::get_board_vision(&self.board, &Turn::White)
    }

    pub fn get_black_vision(&self) -> u64 {
        Game::get_board_vision(&self.board, &Turn::Black)
    }

    pub fn get_board_vision(board: &Board, turn: &Turn) -> u64 {
        let mut vision = 0;

        let own_pieces = board.all_pieces(&turn);
        let opponent_pieces = board.all_pieces(&turn.other());

        let pawns = board.pawns(&turn.other());
        let knights = board.knights(&turn.other());
        let rooks = board.rooks(&turn.other()) | board.queens(&turn.other());
        let bishops = board.bishops(&turn.other()) | board.queens(&turn.other());
        let king = board.king(&turn.other());

        vision |= get_king_dest_squares(king);

        vision |= match turn.other() {
            Turn::White => (pawns << 7) | (pawns << 9),
            Turn::Black => (pawns >> 7) | (pawns >> 9),
        };

        vision |= get_knight_dest_squares(knights);

        let rook_raycast = Raycast::new(rooks, own_pieces | opponent_pieces, 0);
        Direction::rook_directions().iter().for_each(|dir| {
            vision |= rook_raycast.get_full_ray(dir);
        });

        let bishop_raycast = Raycast::new(bishops, own_pieces | opponent_pieces, 0);
        Direction::bishop_directions().iter().for_each(|dir| {
            vision |= bishop_raycast.get_full_ray(dir);
        });

        vision
    }

    /// Gets the vision of all the opponents pieces. Intended for use in determining legal moves for the king.
    pub fn get_opponent_vision(&self) -> u64 {
        Game::get_board_vision(&self.board, &self.turn)
    }

    pub fn simulate_move(&self, lmove: &LongAlgebraicMove) -> bool {
        let mut simulated_board = self.board.clone();
        simulated_board.apply_move(&lmove);

        let opponent_vision = Game::get_board_vision(&simulated_board, &self.turn);

        opponent_vision & simulated_board.king(&self.turn) == 0
    }

    pub fn calculate_legal_moves(&self) -> Vec<LongAlgebraicMove> {
        let mut moves = Vec::new();

        moves.extend(self.calculate_bishop_moves());
        moves.extend(self.calculate_rook_moves());
        moves.extend(self.calculate_pawn_moves());
        moves.extend(self.calculate_king_moves());
        moves.extend(self.calculate_knight_moves());

        moves
            .into_iter()
            .filter(|m| self.simulate_move(m))
            .collect()
    }

    pub fn apply_move(&self, lmove: &LongAlgebraicMove) -> Result<Game, anyhow::Error> {
        let mut new_game = self.clone();

        // Handling enpassant and halfmove clock
        let is_pawn_move = lmove.get_orig()
            & match new_game.turn {
                Turn::White => new_game.board.white_pawns,
                Turn::Black => new_game.board.black_pawns,
            }
            != 0;
        let is_enpassant = lmove.get_dest() & new_game.en_passant != 0 && is_pawn_move;
        let is_capture = lmove.get_dest()
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
            && lmove.get_orig() & (SECOND_RANK | SEVENTH_RANK) != 0
            && lmove.get_dest() & (FOURTH_RANK | FIFTH_RANK) != 0;

        if is_pawn_double_advance {
            match new_game.turn {
                Turn::White => new_game.en_passant = lmove.get_orig() << 8,
                Turn::Black => new_game.en_passant = lmove.get_orig() >> 8,
            }
        } else {
            new_game.en_passant = 0;
        }

        // Handling castling
        let is_rook_move = lmove.get_orig() & ROOK_START_POSITIONS != 0;
        let is_king_move = lmove.get_orig() & KING_START_POSITIONS != 0;

        if is_rook_move {
            if lmove.get_orig() == 0x1 {
                new_game
                    .castling_rights
                    .unset(CastlingRights::WHITE_QUEENSIDE);
            } else if lmove.get_orig() == 0x80 {
                new_game
                    .castling_rights
                    .unset(CastlingRights::WHITE_KINGSIDE);
            } else if lmove.get_orig() == 0x100000000000000 {
                new_game
                    .castling_rights
                    .unset(CastlingRights::BLACK_QUEENSIDE);
            } else if lmove.get_orig() == 0x8000000000000000 {
                new_game
                    .castling_rights
                    .unset(CastlingRights::BLACK_KINGSIDE);
            }
        }

        if is_king_move {
            if lmove.get_orig() == 0x10 {
                new_game
                    .castling_rights
                    .unset(CastlingRights::WHITE_KINGSIDE);
                new_game
                    .castling_rights
                    .unset(CastlingRights::WHITE_QUEENSIDE);
            } else if lmove.get_orig() == 0x1000000000000000 {
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
        if new_game.turn == Turn::White {
            new_game.fullmove_number += 1;
        }

        // Complete move
        new_game.board.apply_move(lmove);

        // Recalculate pins
        new_game.pinned_pieces = new_game.calculate_pinned_pieces();

        Ok(new_game)
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

pub fn get_king_dest_squares(king: u64) -> u64 {
    let w = (king & !A_FILE) >> 1;
    let nw = (king & !A_FILE & !EIGHTH_RANK) << 7;
    let n = (king & !EIGHTH_RANK) << 8;
    let ne = (king & !H_FILE & !EIGHTH_RANK) << 9;
    let e = (king & !H_FILE) << 1;
    let se = (king & !H_FILE & !FIRST_RANK) >> 7;
    let s = (king & !FIRST_RANK) >> 8;
    let sw = (king & !A_FILE & !FIRST_RANK) >> 9;

    w | nw | n | ne | e | se | s | sw
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

        assert_eq!(moves.len(), 3);

        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("a2a3").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("a2b3").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("a2b1").unwrap()));
    }

    #[test]
    fn calculate_king_moves_castles_white() {
        let game =
            Game::from_fen("rn1qk1r1/pbpp1ppp/1p6/2b1p3/4P3/1PNP3N/PBPQBnPP/R3K2R w KQq - 0 1")
                .unwrap();
        let moves = game.calculate_king_moves();
        LongAlgebraicMove::print_list(&moves);

        assert_eq!(moves.len(), 2);

        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e1f1").unwrap()));

        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e1g1").unwrap()));
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
    fn king_cannot_put_self_in_check() {
        let game = Game::from_fen("8/8/8/4k3/1pb2p2/1r3P2/6NK/1n1Q2R1 b - - 0 1").unwrap();
        let moves = game.calculate_king_moves();

        assert_eq!(moves.len(), 3);

        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e5e6").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e5f5").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("e5f6").unwrap()));
    }

    #[test]
    fn king_cannot_castle_through_check() {
        let game = Game::from_fen("8/8/k7/6P1/2b5/8/8/4K2R w K - 0 1").unwrap();
        let moves = game.calculate_king_moves();

        assert!(!moves.contains(&LongAlgebraicMove::from_algebraic("e1g1").unwrap()));
    }

    #[test]
    fn king_cannot_castle_into_check() {
        let game = Game::from_fen("8/8/k7/6P1/3b4/8/8/4K2R w K - 0 1").unwrap();
        let moves = game.calculate_king_moves();

        assert!(!moves.contains(&LongAlgebraicMove::from_algebraic("e1g1").unwrap()));
    }

    #[test]
    fn king_cannot_castle_while_in_check() {
        let game = Game::from_fen("8/8/k7/6P1/1b6/8/8/4K2R w K - 0 1").unwrap();
        let moves = game.calculate_king_moves();

        assert!(!moves.contains(&LongAlgebraicMove::from_algebraic("e1g1").unwrap()));
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
    fn calculates_parallel_rook_moves() {
        let game =
            Game::from_fen("r1bq1rk1/pppn1ppp/3bpn2/3p4/2PP4/5NP1/PP2PPBP/RNBQ1RK1 w - - 0 1")
                .unwrap();

        let moves = game.calculate_rook_moves();

        assert_eq!(moves.len(), 4);

        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("d1d2").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("d1d3").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("d1e1").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("f1e1").unwrap()));
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

    #[test]
    fn calculates_checking_pieces_rook_knight() {
        let game = Game::from_fen("8/B5p1/3rN3/2b4p/3k2R1/4b3/8/6QK b - - 0 1").unwrap();
        let checking_pieces = game.calculate_checking_king();
        assert_eq!(checking_pieces, 0x100040000000); // e6, g4
    }

    #[test]
    fn calculates_checking_pieces_queen_pawn() {
        let game = Game::from_fen("4R3/B5p1/3r4/2b4p/3k4/4P3/8/3Q2QK b - - 0 1").unwrap();
        let checking_pieces = game.calculate_checking_king();
        assert_eq!(checking_pieces, 0x100008); // d1, e3
    }

    #[test]
    fn calculates_opponent_vision() {
        let game = Game::from_fen("8/8/8/8/1pbk1p2/1r3P2/3B2NK/1n1Q3R b - - 0 1").unwrap();
        let opponent_vision = game.get_opponent_vision();
        assert_eq!(opponent_vision, 0xf2f6dcfe);
    }

    #[test]
    fn calculates_opponent_vision_2() {
        let game = Game::from_fen("8/8/8/4k3/1pb2p2/1r3P2/6NK/1n1Q2R1 b - - 0 1").unwrap();
        let opponent_vision = game.get_opponent_vision();
        assert_eq!(opponent_vision, 0x8080808f8fa5cfe);
    }

    #[test]
    fn simluate_move_false_if_illegal() {
        let game = Game::from_fen("8/8/4k3/8/5N2/8/1K6/4R3 b - - 0 1").unwrap();
        let lmove = LongAlgebraicMove::from_algebraic("e6e7").unwrap();
        assert_eq!(game.simulate_move(&lmove), false);
    }

    #[test]
    fn simluate_move_true_if_legal() {
        let game = Game::from_fen("8/8/4k3/8/5N2/8/1K6/4R3 b - - 0 1").unwrap();
        let lmove = LongAlgebraicMove::from_algebraic("e6f6").unwrap();
        assert_eq!(game.simulate_move(&lmove), true);
    }

    #[test]
    fn simluate_move_true_if_blocks_check() {
        let game = Game::from_fen("8/8/4k3/8/8/3b4/1K6/4R3 b - - 0 1").unwrap();
        let block_check = LongAlgebraicMove::from_algebraic("d3e4").unwrap();
        let allow_check = LongAlgebraicMove::from_algebraic("d3f5").unwrap();
        assert_eq!(game.simulate_move(&block_check), true);
        assert_eq!(game.simulate_move(&allow_check), false);
    }

    #[test]
    fn legal_move_count_correct_starting_position() {
        let game = Game::new_default();
        let moves = game.calculate_legal_moves();
        assert_eq!(moves.len(), 20);
    }

    #[test]
    fn legal_move_count_correct_middle_game() {
        let game =
            Game::from_fen("r1bq1rk1/pppn1ppp/3bpn2/3p4/2PP4/5NP1/PP2PPBP/RNBQ1RK1 w - - 0 1")
                .unwrap();
        let moves = game.calculate_legal_moves();
        assert_eq!(moves.len(), 34);
    }

    #[test]
    fn legal_move_count_knight_king_attack() {
        let game = Game::from_fen("8/1k6/8/3N4/5n2/4P3/6KB/r7 w - - 0 1").unwrap();
        let moves = game.calculate_legal_moves();
        assert_eq!(moves.len(), 6);
    }

    #[test]
    fn legal_move_count_queen_king_attack() {
        let game = Game::from_fen("8/1k4q1/8/4N3/8/3nP3/6KB/r7 w - - 0 1").unwrap();
        let moves = game.calculate_legal_moves();
        assert_eq!(moves.len(), 5);
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

        let lmove1 = LongAlgebraicMove::from_algebraic("e2e4").unwrap();
        let lmove2 = LongAlgebraicMove::from_algebraic("e7e5").unwrap();

        let mut game = position1.clone();
        assert_eq!(game, position1);

        game = game.apply_move(&lmove1).unwrap();
        assert_eq!(game, position2);

        game = game.apply_move(&lmove2).unwrap();
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
        let lmove1 = LongAlgebraicMove::from_algebraic("e1g1").unwrap();
        let lmove2 = LongAlgebraicMove::from_algebraic("h8f8").unwrap();

        let mut game = position1.clone();
        assert_eq!(game, position1);

        game = game.apply_move(&lmove1).unwrap();
        assert_eq!(game, position2);

        game = game.apply_move(&lmove2).unwrap();
        assert_eq!(game, position3);
    }
}
