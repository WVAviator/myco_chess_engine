use anyhow::{anyhow, bail};

use crate::{
    cgame::{
        constants::{get_file, get_rank},
        simple_move::algebraic_to_u64,
    },
    magic::{
        get_bishop_magic_map, get_rook_magic_map,
        masks::{get_bishop_mask, get_rook_mask},
    },
    movegen::{king::KING_MOVES, knight::KNIGHT_MOVES},
};

use super::{
    board::Board,
    constants::{A_FILE, FIFTH_RANK, FOURTH_RANK, H_FILE, SIXTH_RANK, THIRD_RANK},
    game::{Game, Turn},
};

#[derive(Debug, PartialEq, Clone)]
pub struct ContextualMove {
    turn: Turn,
    castle: Option<CastleType>,
    piece: PieceType,
    orig: u64,
    dest: u64,
    capture: bool,
    promotion: Option<PieceType>,
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
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
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
    pub fn from_algebraic(algebraic: &str, game: &Game) -> Result<Self, anyhow::Error> {
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
        let mut orig: u64;
        let mut dest: u64;
        let mut ambiguous_file: u64 = u64::MAX;
        let mut ambiguous_rank: u64 = u64::MAX;

        if algebraic.ends_with('#') {
            println!("identified checkmate");
            checkmate = true;
        } else if algebraic.ends_with('+') {
            println!("identified check");
            check = true;
        }
        let algebraic = algebraic.trim_end_matches(['+', '#']);

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

        println!("removed annotations, checking remaining: {}", algebraic);

        match algebraic {
            "O-O-O" => {
                castle = Some(Long);
                println!("identified long castle");
                (orig, dest) = match game.turn {
                    Turn::White => (game.board.white_king, 1 << 2),
                    Turn::Black => (game.board.black_king, 1 << 58),
                };
            }
            "O-O" => {
                castle = Some(Short);
                println!("identified short castle");
                (orig, dest) = match game.turn {
                    Turn::White => (game.board.white_king, 1 << 6),
                    Turn::Black => (game.board.black_king, 1 << 62),
                };
            }
            algebraic => {
                let mut chars = algebraic.chars().rev().peekable();

                println!("checking for promotions");
                match chars.peek() {
                    Some('Q') => {
                        println!("found queen promotion");
                        promotion = Some(PieceType::Queen);
                        chars.next();
                        chars.next();
                    }
                    Some('B') => {
                        println!("found bishop promotion");
                        promotion = Some(PieceType::Bishop);
                        chars.next();
                        chars.next();
                    }
                    Some('R') => {
                        println!("found rook promotion");
                        promotion = Some(PieceType::Rook);
                        chars.next();
                        chars.next();
                    }
                    Some('N') => {
                        println!("found knight promotion");
                        promotion = Some(PieceType::Knight);
                        chars.next();
                        chars.next();
                    }
                    _ => {}
                }

                let rank = chars.next().ok_or(anyhow!("expected destination rank"))?;
                let file = chars.next().ok_or(anyhow!("epected destination file"))?;
                println!("identified rank {} and file {}", rank, file);
                dest = algebraic_to_u64(&format!("{}{}", file, rank));
                println!("converted to dest {}", dest);

                if let Some('x') = chars.peek() {
                    println!("identified capture");
                    capture = true;
                    chars.next();
                }

                let chars: String = chars.collect();
                println!("remaining chars reversed: {}", chars);
                let mut chars = chars.chars().rev().peekable();

                match chars.peek() {
                    Some('B') => {
                        println!("identified bishop move");
                        piece = PieceType::Bishop;
                        chars.next();
                    }
                    Some('R') => {
                        println!("identified rook move");
                        piece = PieceType::Rook;
                        chars.next();
                    }
                    Some('N') => {
                        println!("identified knight move");
                        piece = PieceType::Knight;
                        chars.next();
                    }
                    Some('Q') => {
                        println!("identified queen move");
                        piece = PieceType::Queen;
                        chars.next();
                    }
                    Some('K') => {
                        println!("identified king move");
                        piece = PieceType::King;
                        chars.next();
                    }
                    Some(_) | None => {}
                }

                match chars.next() {
                    Some(file) if !file.is_digit(10) => {
                        println!("identified ambiguous file {}", file);
                        ambiguous_file = get_file(&file);
                        if let Some(rank) = chars.next() {
                            println!("identified double ambiguous {}{}", file, rank);
                            ambiguous_rank = get_rank(&rank);
                        }
                    }
                    Some(rank) if rank.is_digit(10) => {
                        println!("identified ambiguous rank {}", rank);
                        ambiguous_rank = get_rank(&rank)
                    }
                    Some(c) => bail!("unexpected character {}", c),
                    None => {}
                }

                println!("amb file: {}, amb rank: {}", ambiguous_file, ambiguous_rank);

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
            }
        }

        Ok(ContextualMove {
            turn: game.turn.clone(),
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
                .get(self.board.occupied() & get_bishop_mask(dest)),
            PieceType::Rook => get_rook_magic_map()
                .get(dest.trailing_zeros() as usize)
                .expect("failed to load rook magic bitboard")
                .get(self.board.occupied() & get_rook_mask(dest)),
            PieceType::Queen => {
                get_bishop_magic_map()
                    .get(dest.trailing_zeros() as usize)
                    .expect("failed to load bishop magic bitboard")
                    .get(self.board.occupied() & get_bishop_mask(dest))
                    | get_rook_magic_map()
                        .get(dest.trailing_zeros() as usize)
                        .expect("failed to load rook magic bitboard")
                        .get(self.board.occupied() & get_rook_mask(dest))
            }
            PieceType::Pawn => match (&self.turn, capture) {
                (Turn::White, false) => (dest >> 8) | ((dest & FOURTH_RANK) >> 16),
                (Turn::White, true) => ((dest & !A_FILE) >> 9) | ((dest & !H_FILE) >> 7),
                (Turn::Black, false) => (dest << 8) | ((dest & FIFTH_RANK) << 16),
                (Turn::Black, true) => ((dest & !A_FILE) << 7) | ((dest & !H_FILE) << 9),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cgame::simple_move::algebraic_to_u64;

    use super::*;

    #[test]
    fn origin_mask_white_pawn() {
        let game = Game::new_default();
        let mask = game.calculate_origin_mask(&PieceType::Pawn, algebraic_to_u64("e4"), false);

        assert_eq!(mask, 0x101000);
    }

    #[test]
    fn from_algebraic_simple_pawn_move() {
        let game = Game::new_default();
        let cmove = ContextualMove::from_algebraic("e4", &game).unwrap();
        assert_eq!(cmove.piece, PieceType::Pawn);
        assert_eq!(cmove.capture, false);
        assert_eq!(cmove.orig, algebraic_to_u64("e2"));
        assert_eq!(cmove.dest, algebraic_to_u64("e4"));
    }

    #[test]
    fn from_algenbraic_pawn_promotion() {
        let game = Game::from_fen("8/8/5k2/8/8/1K6/6p1/8 b - - 0 1").unwrap();
        let cmove = ContextualMove::from_algebraic("g1=Q", &game).unwrap();

        assert_eq!(cmove.piece, PieceType::Pawn);
        assert_eq!(cmove.capture, false);
        assert_eq!(cmove.orig, algebraic_to_u64("g2"));
        assert_eq!(cmove.dest, algebraic_to_u64("g1"));
        assert_eq!(cmove.promotion, Some(PieceType::Queen));
    }

    #[test]
    fn from_algebraic_ambiguous_move() {
        let game = Game::from_fen("8/8/1N3k2/8/5N2/1K6/6p1/8 w - - 0 1").unwrap();
        let cmove = ContextualMove::from_algebraic("Nbd5+", &game).unwrap();

        assert_eq!(cmove.piece, PieceType::Knight);
        assert_eq!(cmove.capture, false);
        assert_eq!(cmove.orig, algebraic_to_u64("b6"));
        assert_eq!(cmove.dest, algebraic_to_u64("d5"));
    }

    #[test]
    fn from_algebraic_capture() {
        let game = Game::from_fen("8/8/1N3k2/8/2b2N2/1K6/6p1/8 w - - 0 1").unwrap();
        let cmove = ContextualMove::from_algebraic("Nxc4", &game).unwrap();

        assert_eq!(cmove.piece, PieceType::Knight);
        assert_eq!(cmove.capture, true);
        assert_eq!(cmove.orig, algebraic_to_u64("b6"));
        assert_eq!(cmove.dest, algebraic_to_u64("c4"));
    }

    #[test]
    fn from_algebraic_ambiguous_capture_check() {
        let game = Game::from_fen("8/8/1N3k2/3b4/5N2/1K6/6p1/8 w - - 0 1").unwrap();
        let cmove = ContextualMove::from_algebraic("Nbxd5+", &game).unwrap();

        assert_eq!(cmove.piece, PieceType::Knight);
        assert_eq!(cmove.capture, true);
        assert_eq!(cmove.orig, algebraic_to_u64("b6"));
        assert_eq!(cmove.dest, algebraic_to_u64("d5"));
        assert_eq!(cmove.check, true);
    }

    #[test]
    fn from_algebraic_double_ambiguous() {
        let game = Game::from_fen("8/K7/1N3k2/3b4/3q1q2/8/3q2p1/8 b - - 0 1").unwrap();
        let cmove = ContextualMove::from_algebraic("Qf4f2", &game).unwrap();

        assert_eq!(cmove.piece, PieceType::Queen);
        assert_eq!(cmove.capture, false);
        assert_eq!(cmove.orig, algebraic_to_u64("f4"));
        assert_eq!(cmove.dest, algebraic_to_u64("f2"));
    }

    #[test]
    fn from_algebraic_bishop() {
        let game = Game::from_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 1")
            .unwrap();
        let cmove = ContextualMove::from_algebraic("Bd3", &game).unwrap();

        assert_eq!(cmove.piece, PieceType::Bishop);
        assert_eq!(cmove.capture, false);
        assert_eq!(cmove.orig, algebraic_to_u64("f1"));
        assert_eq!(cmove.dest, algebraic_to_u64("d3"));
    }
}
