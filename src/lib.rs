pub mod cgame;
pub mod engine;
pub mod eval;
pub mod magic;
pub mod movegen;
pub mod ml;

#[cfg(test)]
mod integration_tests {
    use cgame::game::Game;
    use movegen::MoveGen;
    use rayon::prelude::*;

    use super::*;

    fn perft(depth: u8, game: Game) -> usize {
        if depth == 0 {
            return 1;
        }

        let moves = game.generate_legal_moves();

        moves
            .into_par_iter() // Parallelize over legal moves
            .map(|lmove| {
                let new_game = game.apply_move(&lmove); // Apply the move
                perft(depth - 1, new_game) // Recursive call for child nodes
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
