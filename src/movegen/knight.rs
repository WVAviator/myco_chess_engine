use std::sync::OnceLock;

use crate::cgame::{
    constants::{
        A_FILE, B_FILE, EIGHTH_RANK, FIRST_RANK, G_FILE, H_FILE, SECOND_RANK, SEVENTH_RANK,
    },
    game::{Game, Turn},
    moves::LongAlgebraicMove,
};

pub trait KnightMoveGen {
    fn generate_knight_vision(&self, turn: &Turn) -> u64;
    fn generate_psuedolegal_knight_moves(&self) -> Result<Vec<LongAlgebraicMove>, anyhow::Error>;
}

impl KnightMoveGen for Game {
    fn generate_knight_vision(&self, turn: &Turn) -> u64 {
        let mut vision = 0;

        let knights = self.board.knights(turn);
        let own_pieces = self.board.all_pieces(turn);

        let mut remaining_knights = knights;
        while remaining_knights != 0 {
            let current_knight = remaining_knights & (!remaining_knights + 1);
            let possible_destinations = get_knight_moves()
                .get(current_knight.trailing_zeros() as usize)
                .unwrap()
                & !own_pieces;

            vision |= possible_destinations;

            remaining_knights &= remaining_knights - 1;
        }

        vision
    }
    fn generate_psuedolegal_knight_moves(&self) -> Result<Vec<LongAlgebraicMove>, anyhow::Error> {
        let mut moves = Vec::new();

        let knights = self.board.knights(&self.turn);
        let own_pieces = self.board.all_pieces(&self.turn);

        let mut remaining_knights = knights;
        while remaining_knights != 0 {
            let current_knight = remaining_knights & (!remaining_knights + 1);
            let possible_destinations = get_knight_moves()
                .get(current_knight.trailing_zeros() as usize)
                .unwrap()
                & !own_pieces;
            let mut remaining_destinations = possible_destinations;
            while remaining_destinations != 0 {
                let dest = remaining_destinations & (!remaining_destinations + 1);

                moves.push(LongAlgebraicMove::new(current_knight, dest));

                remaining_destinations &= remaining_destinations + 1;
            }
            remaining_knights &= remaining_knights - 1;
        }

        Ok(moves)
    }
}

static KNIGHT_MOVES: OnceLock<Vec<u64>> = OnceLock::new();

fn generate_all_knight_moves() -> Vec<u64> {
    (0..64)
        .into_iter()
        .map(|i| {
            let mut dest = 0;
            let knight = 1 << i;

            dest |= (knight & !A_FILE & !B_FILE & !EIGHTH_RANK) << 6;
            dest |= (knight & !A_FILE & !SEVENTH_RANK & !EIGHTH_RANK) << 15;
            dest |= (knight & !H_FILE & !SEVENTH_RANK & !EIGHTH_RANK) << 17;
            dest |= (knight & !H_FILE & !G_FILE & !EIGHTH_RANK) << 10;
            dest |= (knight & !H_FILE & !G_FILE & !FIRST_RANK) >> 6;
            dest |= (knight & !H_FILE & !SECOND_RANK & !FIRST_RANK) >> 15;
            dest |= (knight & !A_FILE & !SECOND_RANK & !FIRST_RANK) >> 17;
            dest |= (knight & !A_FILE & !B_FILE & !FIRST_RANK) >> 10;

            dest
        })
        .collect()
}

fn get_knight_moves() -> &'static Vec<u64> {
    KNIGHT_MOVES.get_or_init(|| generate_all_knight_moves())
}
