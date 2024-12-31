use anyhow::{anyhow, bail};

use crate::{
    game::{
        constants::{get_file, get_rank, A_FILE, FIFTH_RANK, FOURTH_RANK, H_FILE},
        game::{Game, Turn},
    },
    magic::{
        get_bishop_magic_map, get_rook_magic_map,
        masks::{get_bishop_mask, get_rook_mask},
    },
    movegen::{king::KING_MOVES, knight::KNIGHT_MOVES},
    moves::common::algebraic_to_u64,
};

use super::common::PieceType;

#[derive(Debug, PartialEq, Clone)]
pub struct ContextualMove {
    turn: Turn,
    castle: Option<CastleType>,
    piece: PieceType,
    pub orig: u64,
    pub dest: u64,
    capture: bool,
    pub promotion: Option<PieceType>,
    check: bool,
    checkmate: bool,
    annotation: Option<Annotation>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CastleType {
    Short,
    Long,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Annotation {
    Good,
    Mistake,
    Brilliant,
    Blunder,
    Interesting,
    Dubious,
}

impl ContextualMove {
    pub fn from_algebraic(move_str: &str, game: &Game) -> Result<Self, anyhow::Error> {
        use Annotation::*;
        use CastleType::*;
        use PieceType::*;

        let mut castle = None;
        let mut promotion = None;
        let mut annotation = None;
        let mut check = false;
        let mut checkmate = false;
        let mut capture = false;
        let mut piece = Pawn;
        let orig: u64;
        let dest: u64;
        let mut ambiguous_file: u64 = u64::MAX;
        let mut ambiguous_rank: u64 = u64::MAX;

        if move_str.ends_with('#') {
            checkmate = true;
        } else if move_str.ends_with('+') {
            check = true;
        }
        let algebraic = move_str.trim_end_matches(['+', '#']);

        if algebraic.contains("!!") {
            annotation = Some(Brilliant);
        } else if algebraic.contains("??") {
            annotation = Some(Blunder);
        } else if algebraic.contains("!") {
            annotation = Some(Good);
        } else if algebraic.contains("?") {
            annotation = Some(Mistake);
        } else if algebraic.contains("!?") {
            annotation = Some(Interesting);
        } else if algebraic.contains("?!") {
            annotation = Some(Dubious);
        }
        let algebraic = algebraic.trim_end_matches(['!', '?']);

        match algebraic {
            "O-O-O" => {
                castle = Some(Long);
                (orig, dest) = match game.turn {
                    Turn::White => (game.board.white[5], 1 << 2),
                    Turn::Black => (game.board.black[5], 1 << 58),
                };
            }
            "O-O" => {
                castle = Some(Short);
                (orig, dest) = match game.turn {
                    Turn::White => (game.board.white[5], 1 << 6),
                    Turn::Black => (game.board.black[5], 1 << 62),
                };
            }
            algebraic => {
                let mut chars = algebraic.chars().rev().peekable();

                match chars.peek() {
                    Some('Q') => {
                        promotion = Some(PieceType::Queen);
                        chars.next();
                        chars.next();
                    }
                    Some('B') => {
                        promotion = Some(PieceType::Bishop);
                        chars.next();
                        chars.next();
                    }
                    Some('R') => {
                        promotion = Some(PieceType::Rook);
                        chars.next();
                        chars.next();
                    }
                    Some('N') => {
                        promotion = Some(PieceType::Knight);
                        chars.next();
                        chars.next();
                    }
                    _ => {}
                }

                let rank = chars
                    .next()
                    .ok_or(anyhow!("expected destination rank in {}", move_str))?;
                let file = chars
                    .next()
                    .ok_or(anyhow!("expected destination file in {}", move_str))?;
                dest = algebraic_to_u64(&format!("{}{}", file, rank))?;

                if let Some('x') = chars.peek() {
                    capture = true;
                    chars.next();
                }

                let chars: String = chars.collect();
                let mut chars = chars.chars().rev().peekable();

                match chars.peek() {
                    Some('B') => {
                        piece = PieceType::Bishop;
                        chars.next();
                    }
                    Some('R') => {
                        piece = PieceType::Rook;
                        chars.next();
                    }
                    Some('N') => {
                        piece = PieceType::Knight;
                        chars.next();
                    }
                    Some('Q') => {
                        piece = PieceType::Queen;
                        chars.next();
                    }
                    Some('K') => {
                        piece = PieceType::King;
                        chars.next();
                    }
                    Some(_) | None => {}
                }

                match chars.next() {
                    Some(file) if !file.is_ascii_digit() => {
                        ambiguous_file = get_file(&file);
                        if let Some(rank) = chars.next() {
                            ambiguous_rank = get_rank(&rank);
                        }
                    }
                    Some(rank) if rank.is_ascii_digit() => ambiguous_rank = get_rank(&rank),
                    Some(c) => bail!("unexpected character {} in move {}", c, move_str),
                    None => {}
                }

                orig = game.calculate_origin_mask(&piece, dest, capture)
                    & ambiguous_file
                    & ambiguous_rank
                    & match piece {
                        Pawn => game.board.pawns(&game.turn),
                        Rook => game.board.rooks(&game.turn),
                        Knight => game.board.knights(&game.turn),
                        Bishop => game.board.bishops(&game.turn),
                        Queen => game.board.queens(&game.turn),
                        King => game.board.king(&game.turn),
                    };

                if orig.count_ones() != 1 {
                    bail!(
                        "invalid or ambiguous origin square for move {}. Got {}. Game: {}",
                        move_str,
                        orig,
                        game.to_fen(),
                    );
                }
            }
        }

        Ok(ContextualMove {
            turn: game.turn,
            castle,
            piece,
            orig,
            dest,
            capture,
            promotion,
            check,
            checkmate,
            annotation,
        })
    }
}

pub trait OriginMask {
    fn calculate_origin_mask(&self, piece: &PieceType, dest: u64, capture: bool) -> u64;
}

impl OriginMask for Game {
    fn calculate_origin_mask(&self, piece: &PieceType, dest: u64, capture: bool) -> u64 {
        match piece {
            PieceType::King => KING_MOVES[dest.trailing_zeros() as usize],
            PieceType::Knight => KNIGHT_MOVES[dest.trailing_zeros() as usize],
            PieceType::Bishop => get_bishop_magic_map()
                .get(dest.trailing_zeros() as usize)
                .expect("failed to load bishop magic bitboard")
                .get(self.board.all() & get_bishop_mask(dest)),
            PieceType::Rook => get_rook_magic_map()
                .get(dest.trailing_zeros() as usize)
                .expect("failed to load rook magic bitboard")
                .get(self.board.all() & get_rook_mask(dest)),
            PieceType::Queen => {
                get_bishop_magic_map()
                    .get(dest.trailing_zeros() as usize)
                    .expect("failed to load bishop magic bitboard")
                    .get(self.board.all() & get_bishop_mask(dest))
                    | get_rook_magic_map()
                        .get(dest.trailing_zeros() as usize)
                        .expect("failed to load rook magic bitboard")
                        .get(self.board.all() & get_rook_mask(dest))
            }
            PieceType::Pawn => match (&self.turn, capture) {
                (Turn::White, false) => {
                    (dest >> 8) | ((((dest & FOURTH_RANK) >> 8) & !self.board.all()) >> 8)
                }
                (Turn::White, true) => ((dest & !A_FILE) >> 9) | ((dest & !H_FILE) >> 7),
                (Turn::Black, false) => {
                    (dest << 8) | ((((dest & FIFTH_RANK) << 8) & !self.board.all()) << 8)
                }
                (Turn::Black, true) => ((dest & !A_FILE) << 7) | ((dest & !H_FILE) << 9),
            },
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn origin_mask_white_pawn() {
        let game = Game::new_default();
        let mask =
            game.calculate_origin_mask(&PieceType::Pawn, algebraic_to_u64("e4").unwrap(), false);

        assert_eq!(mask, 0x101000);
    }

    #[test]
    fn from_algebraic_simple_pawn_move() {
        let game = Game::new_default();
        let cmove = ContextualMove::from_algebraic("e4", &game).unwrap();
        assert_eq!(cmove.piece, PieceType::Pawn);
        assert!(!cmove.capture);
        assert_eq!(cmove.orig, algebraic_to_u64("e2").unwrap());
        assert_eq!(cmove.dest, algebraic_to_u64("e4").unwrap());
    }

    #[test]
    fn from_algenbraic_pawn_promotion() {
        let game = Game::from_fen("8/8/5k2/8/8/1K6/6p1/8 b - - 0 1").unwrap();
        let cmove = ContextualMove::from_algebraic("g1=Q", &game).unwrap();

        assert_eq!(cmove.piece, PieceType::Pawn);
        assert!(!cmove.capture);
        assert_eq!(cmove.orig, algebraic_to_u64("g2").unwrap());
        assert_eq!(cmove.dest, algebraic_to_u64("g1").unwrap());
        assert_eq!(cmove.promotion, Some(PieceType::Queen));
    }

    #[test]
    fn from_algebraic_ambiguous_move() {
        let game = Game::from_fen("8/8/1N3k2/8/5N2/1K6/6p1/8 w - - 0 1").unwrap();
        let cmove = ContextualMove::from_algebraic("Nbd5+", &game).unwrap();

        assert_eq!(cmove.piece, PieceType::Knight);
        assert!(!cmove.capture);
        assert_eq!(cmove.orig, algebraic_to_u64("b6").unwrap());
        assert_eq!(cmove.dest, algebraic_to_u64("d5").unwrap());
    }

    #[test]
    fn from_algebraic_capture() {
        let game = Game::from_fen("8/8/1N3k2/8/2b2N2/1K6/6p1/8 w - - 0 1").unwrap();
        let cmove = ContextualMove::from_algebraic("Nxc4", &game).unwrap();

        assert_eq!(cmove.piece, PieceType::Knight);
        assert!(cmove.capture);
        assert_eq!(cmove.orig, algebraic_to_u64("b6").unwrap());
        assert_eq!(cmove.dest, algebraic_to_u64("c4").unwrap());
    }

    #[test]
    fn from_algebraic_ambiguous_capture_check() {
        let game = Game::from_fen("8/8/1N3k2/3b4/5N2/1K6/6p1/8 w - - 0 1").unwrap();
        let cmove = ContextualMove::from_algebraic("Nbxd5+", &game).unwrap();

        assert_eq!(cmove.piece, PieceType::Knight);
        assert!(cmove.capture);
        assert_eq!(cmove.orig, algebraic_to_u64("b6").unwrap());
        assert_eq!(cmove.dest, algebraic_to_u64("d5").unwrap());
        assert!(cmove.check);
    }

    #[test]
    fn from_algebraic_double_ambiguous() {
        let game = Game::from_fen("8/K7/1N3k2/3b4/3q1q2/8/3q2p1/8 b - - 0 1").unwrap();
        let cmove = ContextualMove::from_algebraic("Qf4f2", &game).unwrap();

        assert_eq!(cmove.piece, PieceType::Queen);
        assert!(!cmove.capture);
        assert_eq!(cmove.orig, algebraic_to_u64("f4").unwrap());
        assert_eq!(cmove.dest, algebraic_to_u64("f2").unwrap());
    }

    #[test]
    fn from_algebraic_bishop() {
        let game = Game::from_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 1")
            .unwrap();
        let cmove = ContextualMove::from_algebraic("Bd3", &game).unwrap();

        assert_eq!(cmove.piece, PieceType::Bishop);
        assert!(!cmove.capture);
        assert_eq!(cmove.orig, algebraic_to_u64("f1").unwrap());
        assert_eq!(cmove.dest, algebraic_to_u64("d3").unwrap());
    }

    #[test]
    fn pawn_capture_white() {
        let game =
            Game::from_fen("rnbqkbnr/ppp2ppp/4p3/3p4/2P1P3/8/PP1P1PPP/RNBQKBNR w KQkq - 0 1")
                .unwrap();
        let cmove = ContextualMove::from_algebraic("exd5", &game).unwrap();

        assert_eq!(cmove.piece, PieceType::Pawn);
        assert!(cmove.capture);
        assert_eq!(cmove.orig, algebraic_to_u64("e4").unwrap());
        assert_eq!(cmove.dest, algebraic_to_u64("d5").unwrap());
    }
}
