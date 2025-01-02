use arrayvec::ArrayVec;

use crate::{
    game::game::{Game, Turn},
    moves::simple_move::SimpleMove,
    util::iter::{BitIndexIterable, BitIterable},
};

pub trait KnightMoveGen {
    fn generate_knight_vision(&self, turn: &Turn, vision: &mut [u64; 8]);
    fn generate_psuedolegal_knight_moves(&self, moves: &mut ArrayVec<SimpleMove, 256>);
}

impl KnightMoveGen for Game {
    fn generate_knight_vision(&self, turn: &Turn, vision: &mut [u64; 8]) {
        let knights = self.board.knights(turn);

        for index in knights.bit_indexes() {
            vision[2] |= KNIGHT_MOVES[index];
        }
    }

    fn generate_psuedolegal_knight_moves(&self, moves: &mut ArrayVec<SimpleMove, 256>) {
        let (knights, own_pieces) = match self.turn {
            Turn::White => (self.board.white[2], self.board.white[6]),
            Turn::Black => (self.board.black[2], self.board.black[6]),
        };

        for current_knight in knights.bits() {
            let possible_destinations =
                KNIGHT_MOVES[current_knight.trailing_zeros() as usize] & !own_pieces;
            for dest in possible_destinations.bits() {
                unsafe { moves.push_unchecked(SimpleMove::new(current_knight, dest)) }
            }
        }
    }
}

pub const KNIGHT_MOVES: [u64; 64] = [
    132096,
    329728,
    659712,
    1319424,
    2638848,
    5277696,
    10489856,
    4202496,
    33816580,
    84410376,
    168886289,
    337772578,
    675545156,
    1351090312,
    2685403152,
    1075839008,
    8657044482,
    21609056261,
    43234889994,
    86469779988,
    172939559976,
    345879119952,
    687463207072,
    275414786112,
    2216203387392,
    5531918402816,
    11068131838464,
    22136263676928,
    44272527353856,
    88545054707712,
    175990581010432,
    70506185244672,
    567348067172352,
    1416171111120896,
    2833441750646784,
    5666883501293568,
    11333767002587136,
    22667534005174272,
    45053588738670592,
    18049583422636032,
    145241105196122112,
    362539804446949376,
    725361088165576704,
    1450722176331153408,
    2901444352662306816,
    5802888705324613632,
    11533718717099671552,
    4620693356194824192,
    288234782788157440,
    576469569871282176,
    1224997833292120064,
    2449995666584240128,
    4899991333168480256,
    9799982666336960512,
    1152939783987658752,
    2305878468463689728,
    1128098930098176,
    2257297371824128,
    4796069720358912,
    9592139440717824,
    19184278881435648,
    38368557762871296,
    4679521487814656,
    9077567998918656,
];

#[cfg(test)]
mod test {
    use arrayvec::ArrayVec;

    use super::*;
    use crate::game::constants::{
        A_FILE, B_FILE, EIGHTH_RANK, FIRST_RANK, G_FILE, H_FILE, SECOND_RANK, SEVENTH_RANK,
    };

    #[test]
    fn calculate_knight_moves() {
        let game = Game::from_fen("6k1/3b4/2P2n2/1P6/3NP3/1b3PN1/2R1P3/1K5R w - - 0 1").unwrap();
        let mut moves = ArrayVec::new();
        game.generate_psuedolegal_knight_moves(&mut moves);

        assert_eq!(moves.len(), 6);

        assert!(moves.contains(&SimpleMove::from_algebraic("d4b3").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("d4e6").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("d4f5").unwrap()));

        assert!(moves.contains(&SimpleMove::from_algebraic("g3h5").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("g3f5").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("g3f1").unwrap()));
    }

    #[ignore = "not a test"]
    #[test]
    fn generate_knight_moves() {
        let mut result = Vec::new();
        for i in 0..64 {
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

            result.push(dest);
        }

        println!("{:?}", result);
    }
}
