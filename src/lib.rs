#![feature(portable_simd)]

pub mod database;
pub mod engine;
pub mod eval;
pub mod game;
pub mod hash;
pub mod magic;

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

    #[test]
    fn perft_1_correct_leaf_node_count() {
        assert_eq!(perft(1, Game::new_default()), 20);
    }

    #[test]
    fn perft_2_correct_leaf_node_count() {
        assert_eq!(perft(2, Game::new_default()), 400);
    }

    #[test]
    fn perft_3_correct_leaf_node_count() {
        assert_eq!(perft(3, Game::new_default()), 8902);
    }

    #[test]
    fn perft_4_correct_leaf_node_count() {
        assert_eq!(perft(4, Game::new_default()), 197281);
    }

    #[test]
    fn perft_5_correct_leaf_node_count() {
        assert_eq!(perft(5, Game::new_default()), 4865609);
    }

    #[ignore = "slow test"]
    #[test]
    fn perft_6_correct_leaf_node_count() {
        assert_eq!(perft(6, Game::new_default()), 119060324);
    }
}
