use crate::moves::contextual_move::ContextualMove;

#[derive(Debug, PartialEq, Clone)]
pub struct PGN {
    pub event: String,
    pub site: String,
    pub date: String,
    pub white: String,
    pub black: String,
    pub result: GameResult,
    pub white_elo: Option<u8>,
    pub black_elo: Option<u8>,
    pub moves: Vec<ContextualMove>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum GameResult {
    White,
    Black,
    Draw,
}

impl PGN {
    pub fn new(metadata: &str, movetext: &str) -> Self {
        let mut event = "";
        let mut site = "";
        let mut date = "";
        let mut round = "";
        let mut white = "";
        let mut black = "";
        let mut result = GameResult::Draw;
        let mut white_elo = None;
        let mut black_elo = None;

        let metadata_entries = metadata
            .split('\n')
            .map(|entry| entry.trim_start_matches('[').trim_end_matches(']'))
            .for_each(|entry| {
                let mut entry_iter = entry.splitn(2, ' ');
                match entry_iter.next() {
                    Some("Event") => event = entry_iter.next().unwrap_or(""),
                    Some("Site") => site = entry_iter.next().unwrap_or(""),
                    Some("Date") => date = entry_iter.next().unwrap_or(""),
                    Some("Round") => round = entry_iter.next().unwrap_or(""),
                    Some("White") => white = entry_iter.next().unwrap_or(""),
                    Some("Black") => black = entry_iter.next().unwrap_or(""),
                    Some("Result") => {
                        black = match entry_iter.next() {
                            Some("1-0") => GameResult::White,
                            Some("0-1") => GameResult::Black,
                            Some("1/2-1/2") => GameResult::Draw,
                            other => panic!("Unknown game result: {}", other),
                        }
                    }
                    Some("WhiteElo") => white_elo = entry_iter.next(),
                    Some("BlackElo") => black_elo = entry_iter.next(),
                }
            });
    }
}
