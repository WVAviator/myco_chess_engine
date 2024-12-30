use std::time::Instant;

use crate::{game::game::Game, movegen::MoveGen};

pub fn perft_test(depth: u8) {
    let start = Instant::now();

    let count = perft(depth, Game::new_default());

    let elapsed = start.elapsed();
    println!("Total moves generated: {}", count);
    println!("Time elapsed: {}ms", elapsed.as_millis());
    println!("Average NPS: {}", count as f32 / elapsed.as_secs_f32())
}

fn perft(depth: u8, game: Game) -> usize {
    if depth == 0 {
        return 1;
    }

    let moves = game.generate_legal_moves();

    moves
        .into_iter()
        .map(|lmove| {
            let new_game = game.apply_move(&lmove);
            perft(depth - 1, new_game)
        })
        .sum()
}
