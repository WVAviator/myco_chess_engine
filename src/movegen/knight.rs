use std::sync::OnceLock;

use anyhow::anyhow;

use crate::cgame::{
    constants::{
        A_FILE, B_FILE, EIGHTH_RANK, FIRST_RANK, G_FILE, H_FILE, SECOND_RANK, SEVENTH_RANK,
    },
    game::{Game, Turn},
    moves::LongAlgebraicMove,
};

pub trait KnightMoveGen {
    fn generate_knight_vision(&self, turn: &Turn) -> Result<u64, anyhow::Error>;
    fn generate_psuedolegal_knight_moves(&self) -> Result<Vec<LongAlgebraicMove>, anyhow::Error>;
}

impl KnightMoveGen for Game {
    fn generate_knight_vision(&self, turn: &Turn) -> Result<u64, anyhow::Error> {
        let mut vision = 0;

        let knights = self.board.knights(turn);

        let mut remaining_knights = knights;
        while remaining_knights != 0 {
            let current_knight = remaining_knights & (!remaining_knights + 1);

            let possible_destinations = get_knight_moves()
                .get(current_knight.trailing_zeros() as usize)
                .ok_or(anyhow!("unable to retrieve precomputed knight move"))?;

            vision |= possible_destinations;

            remaining_knights &= remaining_knights - 1;
        }

        Ok(vision)
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
                .ok_or(anyhow!("unable to retrieve precomputed knight move"))?
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

pub fn get_knight_moves() -> &'static Vec<u64> {
    KNIGHT_MOVES.get_or_init(|| generate_all_knight_moves())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calculate_knight_moves() {
        let game = Game::from_fen("6k1/3b4/2P2n2/1P6/3NP3/1b3PN1/2R1P3/1K5R w - - 0 1").unwrap();
        let moves = game.generate_psuedolegal_knight_moves().unwrap();

        assert_eq!(moves.len(), 6);

        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("d4b3").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("d4e6").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("d4f5").unwrap()));

        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("g3h5").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("g3f5").unwrap()));
        assert!(moves.contains(&LongAlgebraicMove::from_algebraic("g3f1").unwrap()));
    }
}
