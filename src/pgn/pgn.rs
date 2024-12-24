use anyhow::{bail, Context};
use regex::Regex;

use crate::{
    cgame::game::Game,
    moves::{
        contextual_move::{self, ContextualMove},
        simple_move::SimpleMove,
    },
};

#[derive(Debug, PartialEq, Clone)]
pub struct PGN {
    pub event: String,
    pub site: String,
    pub date: String,
    pub round: String,
    pub white: String,
    pub black: String,
    pub result: GameResult,
    pub white_elo: Option<u32>,
    pub black_elo: Option<u32>,
    pub moves: Vec<ContextualMove>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum GameResult {
    White,
    Black,
    Draw,
}

impl PGN {
    pub fn new(metadata: &str, movetext: &str) -> Result<Self, anyhow::Error> {
        let mut event = String::new();
        let mut site = String::new();
        let mut date = String::new();
        let mut round = String::new();
        let mut white = String::new();
        let mut black = String::new();
        let mut result = GameResult::Draw;
        let mut white_elo = None;
        let mut black_elo = None;

        for entry in metadata
            .split('\n')
            .map(|entry| entry.trim().trim_start_matches('[').trim_end_matches(']'))
        {
            let mut entry_iter = entry.splitn(2, ' ');
            match entry_iter.next() {
                Some("Event") => {
                    event = entry_iter
                        .next()
                        .unwrap_or("")
                        .trim_start_matches("\"")
                        .trim_end_matches("\"")
                        .to_string()
                }
                Some("Site") => {
                    site = entry_iter
                        .next()
                        .unwrap_or("")
                        .trim_start_matches("\"")
                        .trim_end_matches("\"")
                        .to_string()
                }
                Some("Date") => {
                    date = entry_iter
                        .next()
                        .unwrap_or("")
                        .trim_start_matches("\"")
                        .trim_end_matches("\"")
                        .to_string()
                }
                Some("Round") => {
                    round = entry_iter
                        .next()
                        .unwrap_or("")
                        .trim_start_matches("\"")
                        .trim_end_matches("\"")
                        .to_string()
                }
                Some("White") => {
                    white = entry_iter
                        .next()
                        .unwrap_or("")
                        .trim_start_matches("\"")
                        .trim_end_matches("\"")
                        .to_string()
                }
                Some("Black") => {
                    black = entry_iter
                        .next()
                        .unwrap_or("")
                        .trim_start_matches("\"")
                        .trim_end_matches("\"")
                        .to_string()
                }
                Some("Result") => {
                    result = match entry_iter.next() {
                        Some("\"1-0\"") => GameResult::White,
                        Some("\"0-1\"") => GameResult::Black,
                        Some("\"1/2-1/2\"") => GameResult::Draw,
                        Some(other) => bail!("Unknown game result: {}", other),
                        _ => bail!("Missing game result."),
                    }
                }
                Some("WhiteElo") => {
                    white_elo = entry_iter
                        .next()
                        .map(|elo| elo.trim_start_matches("\"").trim_end_matches("\"").parse())
                        .and_then(Result::ok);
                }
                Some("BlackElo") => {
                    black_elo = entry_iter
                        .next()
                        .map(|elo| elo.parse())
                        .and_then(Result::ok)
                }
                Some(other) => {
                    println!("unsupported metadata item: {}", other)
                }
                None => {}
            }
        }

        let re = Regex::new(r"[\d]+\.")?;

        let algebraic_moves = re.replace_all(movetext, "");
        let mut algebraic_moves = algebraic_moves.split_whitespace();
        let mut moves = Vec::new();
        let mut game = Game::new_default();

        while let Some(next_move) = algebraic_moves.next() {
            match next_move {
                "1/2-1/2" => continue,
                "1-0" => continue,
                "0-1" => continue,
                _ => {}
            }
            let current_move = ContextualMove::from_algebraic(next_move, &game)
                .context("could not read algebraic move")?;
            game = game.apply_move(&SimpleMove::from(&current_move));
            moves.push(current_move);
        }

        Ok(PGN {
            event,
            site,
            date,
            round,
            white,
            black,
            result,
            white_elo,
            black_elo,
            moves,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn properly_parses_pgn() {
        let metadata = r#"
            [Event "Candidats Tournament"]
            [Site "Amsterdam"]
            [Date "1956.??.??"]
            [Round "17"]
            [White "Szabo, Laszlo"]
            [Black "Petrosian, Tigran V"]
            [Result "1/2-1/2"]
            [WhiteElo ""]
            [BlackElo ""]
            [ECO "E89"]
        "#;
        let movetext = r#"
            1.c4 g6 2.d4 Bg7 3.Nc3 d6 4.e4 Nf6 5.f3 O-O 6.Be3 e5 7.Nge2 c6 8.d5 cxd5
            9.cxd5 a6 10.Qd2 Nbd7 11.Nc1 Nh5 12.Nd3 f5 13.O-O-O Nb6 14.Nb4 Bd7 15.Kb1 Rc8
            16.Qf2 Na4 17.Nxa4 Bxa4 18.b3 Bd7 19.Bb6 Qe8 20.Qd2 fxe4 21.fxe4 Bb5 22.Nd3 Nf6
            23.Qb4 Qe7 24.Nb2 Bh6 25.Bxb5 axb5 26.Rhe1 Qd7 27.h3 Ne8 28.Nd3 Nc7 29.Be3 Bxe3
            30.Rxe3 Na6 31.Qd2 Nc5  1/2-1/2
        "#;

        let pgn = PGN::new(metadata, movetext).unwrap();

        assert_eq!(pgn.moves.len(), 62);
        assert_eq!(pgn.result, GameResult::Draw);
        assert_eq!(pgn.event, "Candidats Tournament");
        assert_eq!(pgn.white_elo, None);
    }

    #[test]
    fn properly_parses_pgn2() {
        let metadata = r#"
            [Event "35th ECC Women 2019"]
            [Site "Budva MNE"]
            [Date "2019.11.13"]
            [Round "4.3"]
            [White "Tomilova,E"]
            [Black "Khotenashvili,B"]
            [Result "1/2-1/2"]
            [WhiteElo "2345"]
            [BlackElo "2446"]
            [ECO "E91"]
        "#;
        let movetext = r#"
            1.c4 g6 2.e4 Bg7 3.d4 d6 4.Nc3 Nf6 5.Nf3 O-O 6.Be2 Nbd7 7.e5 Ne8 8.h4 c5
            9.exd6 Nxd6 10.h5 cxd4 11.Nxd4 Nf6 12.hxg6 hxg6 13.Be3 Nf5 14.Nxf5 Qxd1+
            15.Rxd1 Bxf5 16.b3 b6 17.Nb5 Be4 18.Bf3 Bxf3 19.gxf3 Rfc8 20.a4 a6 21.Nc3 Rab8
            22.Ke2 Rb7 23.Nd5 Nxd5 24.Rxd5 Kf8 25.f4 f5 26.Rhd1 Ke8 27.R1d3 Bf6 28.Kd1 Bg7
            29.a5 bxa5 30.Rxa5 Rc6 31.Rad5 Rc8 32.Kc2 Bf6 33.Bd2 Bh4 34.Rh3 Bf6 35.Rg3 Kf7
            36.Ra5 Rc6 37.Rd3 Bh4 38.f3 Ke8 39.Rad5 Rb8 40.Bc3 Kf7 41.Rd1 Bf6 42.Rd8 Rxd8
            43.Rxd8 e5 44.Rd5 exf4 45.Bd2 Ke6 46.Bxf4 g5 47.Bd2 Rd6 48.Ra5 Rc6 49.Kd3 Rd6+
            50.Kc2 Rc6 51.Kd3 Rd6+  1/2-1/2
        "#;

        let pgn = PGN::new(metadata, movetext).unwrap();

        assert_eq!(pgn.moves.len(), 102);
        assert_eq!(pgn.result, GameResult::Draw);
        assert_eq!(pgn.site, "Budva MNE");
        assert_eq!(pgn.white, "Tomilova,E");
        assert_eq!(pgn.white_elo, Some(2345));
    }
}
