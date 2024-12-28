use smallvec::SmallVec;

use crate::{
    game::game::{Game, Turn},
    magic::{get_rook_magic_map, masks::get_rook_mask},
    moves::simple_move::SimpleMove,
};

pub trait RookMoveGen {
    fn generate_rook_vision(&self, turn: &Turn) -> u64;
    fn generate_pseudolegal_rook_moves(&self, moves: &mut SmallVec<[SimpleMove; 256]>);
}

impl RookMoveGen for Game {
    fn generate_rook_vision(&self, turn: &Turn) -> u64 {
        let mut vision = 0;

        match turn {
            Turn::White => {
                let rook_pieces = self.board.white[1] | self.board.white[4];
                let player_pieces = self.board.white_pieces();
                let opponent_pieces = self.board.black_pieces();

                let mut remaining_rooks = rook_pieces;
                while remaining_rooks != 0 {
                    let current_rook = remaining_rooks & (!remaining_rooks + 1);
                    let blockers = (player_pieces | opponent_pieces) & get_rook_mask(current_rook);

                    let rook_moves =
                        get_rook_magic_map()[current_rook.trailing_zeros() as usize].get(blockers);

                    vision |= rook_moves;

                    remaining_rooks &= remaining_rooks - 1;
                }

                vision
            }
            Turn::Black => {
                let rook_pieces = self.board.black[1] | self.board.black[4];
                let player_pieces = self.board.black_pieces();
                let opponent_pieces = self.board.white_pieces();

                let mut remaining_rooks = rook_pieces;
                while remaining_rooks != 0 {
                    let current_rook = remaining_rooks & (!remaining_rooks + 1);
                    let blockers = (player_pieces | opponent_pieces) & get_rook_mask(current_rook);

                    let rook_moves =
                        get_rook_magic_map()[current_rook.trailing_zeros() as usize].get(blockers);

                    vision |= rook_moves;

                    remaining_rooks &= remaining_rooks - 1;
                }

                vision
            }
        }
    }

    fn generate_pseudolegal_rook_moves(&self, moves: &mut SmallVec<[SimpleMove; 256]>) {
        match self.turn {
            Turn::White => {
                let rook_pieces = self.board.white[1] | self.board.white[4];
                let player_pieces = self.board.white[6];
                let opponent_pieces = self.board.black[6];

                let mut remaining_rooks = rook_pieces;
                while remaining_rooks != 0 {
                    let current_rook = remaining_rooks & (!remaining_rooks + 1);
                    let blockers = (player_pieces | opponent_pieces) & get_rook_mask(current_rook);

                    let rook_moves = get_rook_magic_map()[current_rook.trailing_zeros() as usize]
                        .get(blockers)
                        & !player_pieces;

                    let mut remaining_rook_moves = rook_moves;
                    while remaining_rook_moves != 0 {
                        let dest = remaining_rook_moves & (!remaining_rook_moves + 1);
                        moves.push(SimpleMove::new(current_rook, dest));
                        remaining_rook_moves &= remaining_rook_moves - 1;
                    }

                    remaining_rooks &= remaining_rooks - 1;
                }
            }
            Turn::Black => {
                let rook_pieces = self.board.black[1] | self.board.black[4];
                let player_pieces = self.board.black[6];
                let opponent_pieces = self.board.white[6];

                let mut remaining_rooks = rook_pieces;
                while remaining_rooks != 0 {
                    let current_rook = remaining_rooks & (!remaining_rooks + 1);
                    let blockers = (player_pieces | opponent_pieces) & get_rook_mask(current_rook);

                    let rook_moves = get_rook_magic_map()[current_rook.trailing_zeros() as usize]
                        .get(blockers)
                        & !player_pieces;

                    let mut remaining_rook_moves = rook_moves;
                    while remaining_rook_moves != 0 {
                        let dest = remaining_rook_moves & (!remaining_rook_moves + 1);
                        moves.push(SimpleMove::new(current_rook, dest));
                        remaining_rook_moves &= remaining_rook_moves - 1;
                    }

                    remaining_rooks &= remaining_rooks - 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calculates_basic_rook_moves() {
        let game = Game::from_fen("k7/8/3p4/3r3p/8/B2N4/qb6/7K b - - 0 1").unwrap();

        let mut moves = SmallVec::new();
        game.generate_pseudolegal_rook_moves(&mut moves);

        assert_eq!(moves.len(), 10);

        assert!(moves.contains(&SimpleMove::from_algebraic("a2a1").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("a2a3").unwrap()));

        assert!(moves.contains(&SimpleMove::from_algebraic("d5c5").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("d5b5").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("d5a5").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("d5e5").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("d5f5").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("d5g5").unwrap()));

        assert!(moves.contains(&SimpleMove::from_algebraic("d5d4").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("d5d3").unwrap()));
    }

    #[test]
    fn calculates_parallel_rook_moves() {
        let game =
            Game::from_fen("r1bq1rk1/pppn1ppp/3bpn2/3p4/2PP4/5NP1/PP2PPBP/RNBQ1RK1 w - - 0 1")
                .unwrap();

        let mut moves = SmallVec::new();
        game.generate_pseudolegal_rook_moves(&mut moves);

        assert_eq!(moves.len(), 4);

        assert!(moves.contains(&SimpleMove::from_algebraic("d1d2").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("d1d3").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("d1e1").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("f1e1").unwrap()));
    }
}
