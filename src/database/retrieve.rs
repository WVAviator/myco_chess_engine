use anyhow::bail;
use rand::random;
use rusqlite::Connection;

use crate::{cgame::game::Game, hash::zobrist::ZobristHash, moves::simple_move::SimpleMove};

use super::schema::MovesEntry;

pub trait MoveRetrieval {
    fn random_database_move(
        &self,
        connection: &Connection,
    ) -> Result<Option<SimpleMove>, anyhow::Error>;
}

impl MoveRetrieval for Game {
    fn random_database_move(
        &self,
        connection: &Connection,
    ) -> Result<Option<SimpleMove>, anyhow::Error> {
        let hash = self.zobrist();
        match MovesEntry::find_by_hash(connection, hash) {
            Ok(Some(entry)) => {
                let random_index = random::<usize>() % entry.moves.len();
                let lmove = SimpleMove::from_algebraic(entry.moves.get(random_index).unwrap())?;

                Ok(Some(lmove))
            }
            Ok(None) => Ok(None),
            _ => bail!("failed to fetch data from the database"),
        }
    }
}
