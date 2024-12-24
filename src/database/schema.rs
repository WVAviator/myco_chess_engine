use anyhow::Context;
use rusqlite::Connection;

pub struct MovesEntry {
    pub hash: u64,
    pub moves: Vec<String>,
}

impl MovesEntry {
    pub fn new(hash: u64, moves: Vec<String>) -> Self {
        Self { hash, moves }
    }

    pub fn create_tables(connection: &Connection) -> Result<(), anyhow::Error> {
        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS positions (
                    hash INTEGER NOT NULL PRIMARY KEY
                )",
                [],
            )
            .context("failed to create positions table in sqlite")?;
        connection
            .execute(
                "CREATE TABLE IF NOT EXISTS moves (
                    hash INTEGER NOT NULL,
                    move TEXT NOT NULL,
                    FOREIGN KEY (hash) REFERENCES positions(hash)
                )",
                [],
            )
            .context("failed to create moves table in sqlite")?;

        Ok(())
    }

    pub fn insert(&self, connection: &mut Connection) -> Result<(), anyhow::Error> {
        let tx = connection.transaction()?;

        tx.execute("INSERT INTO positions (hash) VALUES (?1)", [self.hash])?;

        for mv in &self.moves {
            tx.execute(
                "INSERT INTO moves (hash, move) VALUES (?1, ?2)",
                rusqlite::params![self.hash, mv],
            )?;
        }

        tx.commit()?;
        Ok(())
    }

    pub fn find_by_hash(connection: &Connection, hash: u64) -> Result<Option<Self>, anyhow::Error> {
        let mut stmt = connection.prepare("SELECT move FROM moves WHERE hash = ?1")?;
        let moves = stmt
            .query_map([hash], |row| Ok(row.get(0)?))?
            .collect::<Result<Vec<String>, rusqlite::Error>>()?;

        if moves.is_empty() {
            Ok(None)
        } else {
            Ok(Some(MovesEntry { hash, moves }))
        }
    }
}