use std::{
    io::{BufRead, BufReader},
    time::Duration,
};

use anyhow::{anyhow, bail, Context};
use rust_chess_engine::cgame::{engine::SimpleEngine, game::Game, moves::LongAlgebraicMove};

fn main() {
    repl();
}

pub fn repl() {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin);

    let mut current_game = Game::new_default();

    loop {
        let mut buffer = String::new();
        if let Err(_) = reader.read_line(&mut buffer) {
            panic!("info string \"failed to initialize stdin\" ");
        }

        match buffer.as_str().trim_end() {
            "uci" => {
                println!("id name Myco");
                println!("id author WVAviator");
                println!("uciok");
            }
            "debug on" => {}
            "debug off" => {}
            "isready" => {
                println!("readyok");
            }
            cmd if cmd.starts_with("setoption") => {}
            cmd if cmd.starts_with("registration") => {}
            "ucinewgame" => {}
            cmd if cmd.starts_with("position") => {
                current_game = process_position_command(cmd).unwrap_or_else(|error| {
                    panic!("info string invalid position\n{}", error);
                });
            }
            cmd if cmd.starts_with("go") => {
                let best_move = get_best_move(&current_game).unwrap_or_else(|error| {
                    panic!("info string could not calculate\n{}", error);
                });
                println!(
                    "bestmove {}",
                    best_move.to_algebraic().unwrap_or_else(|error| {
                        panic!(
                            "info string could not convert to algebraic notation\n{}",
                            error
                        );
                    })
                );
                current_game = current_game.apply_move(&best_move).unwrap_or_else(|error| {
                    panic!("info string failed to apply calculated move\n{}", error)
                });
            }
            "stop" => {}
            "ponderhit" => {}
            "quit" => break,

            _other => panic!("info string \"unknown command: {}\"", _other),
        }
    }
}

fn process_position_command(cmd: &str) -> Result<Game, anyhow::Error> {
    match cmd
        .split(" ")
        .skip(1)
        .next()
        .context("expected 'fen' or 'startpos'")?
    {
        "fen" => {
            let fen_str = extract_fen(cmd).ok_or(anyhow!("missing fen string"))?;
            return Game::from_fen(&fen_str).context("invalid fen string");
        }
        "startpos" => {
            let moves = extract_moves(cmd).context("could not extract moves from command")?;
            let mut game = Game::new_default();
            for lmove in moves {
                game = game.apply_move(&lmove)?;
            }
            return Ok(game);
        }
        _ => bail!("invalid position command"),
    }
}

fn extract_fen(command: &str) -> Option<String> {
    if command.starts_with("position fen") {
        let parts: Vec<&str> = command.splitn(3, ' ').collect();
        if parts.len() > 2 {
            return Some(parts[2].to_string());
        }
    }
    None
}

fn extract_moves(command: &str) -> Result<Vec<LongAlgebraicMove>, anyhow::Error> {
    if command.starts_with("position startpos") {
        let parts: Vec<&str> = command.splitn(4, ' ').collect();
        if parts.len() == 2 {
            return Ok(Vec::new());
        }

        return parts[3]
            .split(' ')
            .map(|lan| LongAlgebraicMove::from_algebraic(lan))
            .collect();
    }

    Err(anyhow!("could not extract moves from startpos"))
}

fn get_best_move(game: &Game) -> Result<LongAlgebraicMove, anyhow::Error> {
    let engine = SimpleEngine::new(&game);
    let best_move = engine.get_best_move(3, Duration::from_secs(3))?;
    Ok(best_move)
}
