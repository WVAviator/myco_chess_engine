use std::{
    io::{BufRead, BufReader},
    time::Instant,
};

use anyhow::{anyhow, bail, Context};
use clap::Parser;
use myco_chess_engine::{
    database::build::DatabaseTrainingSession,
    game::game::Game,
    magic::{get_bishop_magic_map, get_rook_magic_map},
    movegen::MoveGen,
    moves::simple_move::SimpleMove,
    search::quiescence::QuiescenceSearch,
};
use rayon::ThreadPoolBuilder;

fn main() {
    let args = Args::parse();

    if let Some(threads) = args.threads {
        ThreadPoolBuilder::new()
            .num_threads(threads)
            .build_global()
            .expect("failed to build global thread pool");
    }

    if let Some(depth) = args.perft {
        initialize();
        depth_test(depth);
    }

    if let Some(pgn_path) = args.train {
        train_pgn(pgn_path);
    }

    repl();
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Perform a Perft move generation test to the specified depth
    #[arg(long, name = "depth")]
    perft: Option<u8>,

    /// Train moves from the given PGN file to the engine's database
    #[arg(long, name = "pgn file")]
    train: Option<String>,

    /// Explictly set the number of threads the engine should use (default is the number of CPU cores on the host machine)
    #[arg(long, name = "num threads")]
    threads: Option<usize>,
}

fn initialize() {
    get_rook_magic_map();
    get_bishop_magic_map();
}

fn depth_test(depth: u8) {
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

fn train_pgn(pgn_path: String) {
    let mut session =
        DatabaseTrainingSession::new(&pgn_path).expect("failed to load provided pgn file");
    session.start().expect("failed to complete training pgn");
}

pub fn repl() {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin);

    let mut current_game = Game::new_default();

    loop {
        let mut buffer = String::new();
        if reader.read_line(&mut buffer).is_err() {
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
                println!("bestmove {}", best_move.to_algebraic());
                current_game = current_game.apply_move(&best_move);
            }
            "stop" => {}
            "ponderhit" => {}
            "quit" => {
                println!("quitting...");
                break;
            }

            _other => panic!("info string \"unknown command: {}\"", _other),
        }
    }
}

fn process_position_command(cmd: &str) -> Result<Game, anyhow::Error> {
    match cmd
        .split(" ")
        .nth(1)
        .context("expected 'fen' or 'startpos'")?
    {
        "fen" => {
            let fen_str = extract_fen(cmd).ok_or(anyhow!("missing fen string"))?;
            Game::from_fen(&fen_str).context("invalid fen string")
        }
        "startpos" => extract_moves(cmd).context("invalid moves list"),
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

fn extract_moves(command: &str) -> Result<Game, anyhow::Error> {
    if command.starts_with("position startpos") {
        let parts: Vec<&str> = command.splitn(4, ' ').collect();

        if parts.len() == 2 {
            return Ok(Game::new_default());
        }

        return Game::from_uci_startpos(parts[3]);
    }

    Err(anyhow!("could not extract moves from startpos"))
}

fn get_best_move(game: &Game) -> Result<SimpleMove, anyhow::Error> {
    let engine = QuiescenceSearch::new(game, 6, 15);
    let best_move = engine.search();
    Ok(best_move)
}
