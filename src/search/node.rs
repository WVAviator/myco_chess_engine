use std::{cell::OnceCell, simd::Simd};

use arrayvec::ArrayVec;

use crate::{
    eval::{mvvlva::MVVLVAEval, piece::PieceEval},
    game::game::{Game, Turn},
    hash::zobrist::ZobristHash,
    movegen::MoveGen,
    moves::simple_move::SimpleMove,
};

#[derive(Debug)]
pub struct Node {
    game: Game,
    black_vision: OnceCell<Simd<u64, 8>>,
    white_vision: OnceCell<Simd<u64, 8>>,
    zobrist: OnceCell<u64>,
    legal_moves: OnceCell<ArrayVec<SimpleMove, 256>>,
    static_eval: OnceCell<i32>,
}

impl Node {
    pub fn new(game: Game) -> Self {
        let black_vision = OnceCell::new();
        let white_vision = OnceCell::new();
        let zobrist = OnceCell::new();
        let legal_moves = OnceCell::new();
        let static_eval = OnceCell::new();

        Node {
            game,
            black_vision,
            white_vision,
            zobrist,
            legal_moves,
            static_eval,
        }
    }

    pub fn get_black_vision(&self) -> &Simd<u64, 8> {
        self.black_vision
            .get_or_init(|| self.game.generate_vision(&Turn::Black))
    }

    pub fn get_white_vision(&self) -> &Simd<u64, 8> {
        self.white_vision
            .get_or_init(|| self.game.generate_vision(&Turn::White))
    }

    pub fn get_zobrist(&self) -> &u64 {
        self.zobrist.get_or_init(|| self.game.zobrist())
    }

    pub fn get_legal_moves(&self) -> &ArrayVec<SimpleMove, 256> {
        self.legal_moves.get_or_init(|| {
            let mut legal_moves: ArrayVec<(i32, SimpleMove), 256> = self
                .game
                .generate_legal_moves()
                .into_iter()
                .map(|lmove| (self.game.evaluate_mvv_lva(&lmove), lmove))
                .collect();

            legal_moves.sort_unstable_by_key(|eval| eval.0);

            legal_moves.into_iter().map(|eval| eval.1).collect()
        })
    }

    pub fn get_static_eval(&self) -> &i32 {
        self.static_eval
            .get_or_init(|| self.game.calculate_piece_value())
    }

    pub fn apply_move(&self, lmove: &SimpleMove) -> Node {
        let game = self.game.apply_move(lmove);
        Node::new(game)
    }
}
