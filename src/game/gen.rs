use super::game::Game;

pub struct MoveGenerator {}

impl MoveGenerator {
    pub fn generate(game: &Game) -> Result<Vec<Game>, anyhow::Error> {
        Ok(vec![])
    }
}
