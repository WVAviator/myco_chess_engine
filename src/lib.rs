#![feature(portable_simd, core_intrinsics)]

pub mod cache;
pub mod database;
pub mod engine;
pub mod eval;
pub mod game;
pub mod hash;
pub mod magic;
pub mod search;
pub mod util;

#[cfg(feature = "pytorch")]
pub mod ml;

pub mod movegen;
pub mod moves;
pub mod pgn;

#[cfg(test)]
mod integration_tests {
    use game::game::Game;
    use movegen::MoveGen;
    use rayon::prelude::*;

    use super::*;

    fn perft(depth: u8, game: Game) -> usize {
        println!("perft depth {}", depth);
        if depth == 0 {
            println!("returning 1");
            return 1;
        }

        println!("generating moves... ");
        let moves = game.generate_legal_moves();
        println!("generated {} legal moves", moves.len());

        moves
            .into_par_iter()
            .map(|lmove| {
                let new_game = game.apply_move(lmove);
                println!("applied move {}", lmove);
                perft(depth - 1, new_game)
            })
            .sum()
    }

    #[ignore = "run with increased stack size"]
    #[test]
    fn perft_1_correct_leaf_node_count() {
        assert_eq!(perft(1, Game::new_default()), 20);
    }

    #[ignore = "run with increased stack size"]
    #[test]
    fn perft_2_correct_leaf_node_count() {
        assert_eq!(perft(2, Game::new_default()), 400);
    }

    #[ignore = "run with increased stack size"]
    #[test]
    fn perft_3_correct_leaf_node_count() {
        assert_eq!(perft(3, Game::new_default()), 8902);
    }

    #[ignore = "run with increased stack size"]
    #[test]
    fn perft_4_correct_leaf_node_count() {
        assert_eq!(perft(4, Game::new_default()), 197281);
    }

    #[ignore = "run with increased stack size"]
    #[test]
    fn perft_5_correct_leaf_node_count() {
        assert_eq!(perft(5, Game::new_default()), 4865609);
    }

    #[ignore = "run with increased stack size"]
    #[test]
    fn perft_6_correct_leaf_node_count() {
        assert_eq!(perft(6, Game::new_default()), 119060324);
    }
}
