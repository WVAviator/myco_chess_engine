use rusqlite::Connection;

use crate::{
    game::game::Game,
    hash::zobrist::ZobristHash,
    moves::simple_move::SimpleMove,
    pgn::{
        parser::parse_pgn_file,
        pgn::{GameResult, PGN},
    },
};

use super::{connection::get_connection, schema::MovesEntry};

// Maximum number of moves in a game to train
const TRAINING_LIMIT: usize = 16;

pub struct DatabaseTrainingSession {
    file_path: String,
    pgn_data: Vec<PGN>,
    connection: Connection,
}

impl DatabaseTrainingSession {
    pub fn new(file_path: &str) -> Result<Self, anyhow::Error> {
        let pgn_data = parse_pgn_file(file_path)?;
        let connection = get_connection();

        MovesEntry::create_tables(&connection)?;

        Ok(Self {
            file_path: file_path.to_string(),
            pgn_data,
            connection,
        })
    }

    pub fn start(&mut self) -> Result<(), anyhow::Error> {
        println!("training moves from {}", self.file_path);

        let total_games = self.pgn_data.len();
        let mut current_total = 0;

        for pgn in &self.pgn_data {
            current_total += 1;
            print!(
                "\rtraining game {}/{}: {} vs. {}, {}                             ",
                current_total, total_games, pgn.white, pgn.black, pgn.event
            );
            let mut game = Game::new_default();

            let mut moves_iter = pgn.moves.iter().take(TRAINING_LIMIT * 2 + 1);

            match pgn.result {
                GameResult::White => {
                    while let Some(m) = moves_iter.next() {
                        let sm = SimpleMove::from(m);
                        let entry = MovesEntry::new(game.zobrist(), vec![sm.to_algebraic()]);
                        entry.insert(&mut self.connection)?;
                        game = game.apply_move(&sm);

                        // Skip the next move and only add moves for white
                        if let Some(m) = moves_iter.next() {
                            game = game.apply_move(&SimpleMove::from(m));
                        }
                    }
                }
                GameResult::Black => {
                    // Skip the first move and only add moves for black
                    if let Some(m) = moves_iter.next() {
                        game = game.apply_move(&SimpleMove::from(m));
                    }
                    while let Some(m) = moves_iter.next() {
                        let sm = SimpleMove::from(m);
                        let entry = MovesEntry::new(game.zobrist(), vec![sm.to_algebraic()]);
                        entry.insert(&mut self.connection)?;
                        game = game.apply_move(&sm);

                        if let Some(m) = moves_iter.next() {
                            game = game.apply_move(&SimpleMove::from(m));
                        }
                    }
                }
                GameResult::Draw => {
                    for m in moves_iter {
                        let sm = SimpleMove::from(m);
                        let entry = MovesEntry::new(game.zobrist(), vec![sm.to_algebraic()]);
                        entry.insert(&mut self.connection)?;
                        game = game.apply_move(&sm);
                    }
                }
                GameResult::Abandonment => continue,
            }
        }
        println!("\nfinished");

        Ok(())
    }
}
