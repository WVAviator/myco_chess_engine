use super::game::Game;

pub struct MoveGenerator {
    game: Game,
}

impl MoveGenerator {
    pub fn new(game: Game) -> Self {
        Self { game }
    }
    pub fn generate(&self) -> Result<Vec<Game>, anyhow::Error> {
        Ok(vec![])
    }
}
