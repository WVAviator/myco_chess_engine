use std::{fs, path::Path};

use anyhow::{anyhow, bail, Context};

use super::pgn::PGN;

pub fn parse_pgn_file(file_path: &str) -> Result<Vec<PGN>, anyhow::Error> {
    if !Path::new(file_path).exists() {
        bail!("could not find provided pgn file");
    }

    print!("loading pgn file at {}...", file_path);

    let file_contents = fs::read_to_string(file_path).context("could not read pgn file")?;

    let mut parts = file_contents.split("\r\n\r\n");

    let mut pgn_list = Vec::new();

    while let Some(metadata) = parts.next() {
        let movetext = parts
            .next()
            .ok_or(anyhow!("expected movetext to go with metadata"))?;

        match PGN::new(metadata, movetext) {
            Ok(pgn) => pgn_list.push(pgn),
            Err(_) => {}
        }
    }

    println!(" finished");

    Ok(pgn_list)
}
