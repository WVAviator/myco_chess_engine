use smallvec::SmallVec;

use crate::{
    game::{
        castling_rights::CastlingRights,
        constants::{A_FILE, EIGHTH_RANK, FIRST_RANK, H_FILE},
        game::{Game, Turn},
    },
    moves::simple_move::SimpleMove,
};

use super::MoveGen;

const CASTLE_MOVE_WK_MASK: u64 = 0x60;
const CASTLE_MOVE_WQ_MASK: u64 = 0xe;
const CASTLE_MOVE_BK_MASK: u64 = 0x6000000000000000;
const CASTLE_MOVE_BQ_MASK: u64 = 0xe00000000000000;

const CASTLE_CHECK_WK_MASK: u64 = 0x70;
const CASTLE_CHECK_WQ_MASK: u64 = 0x1c;
const CASTLE_CHECK_BK_MASK: u64 = 0x7000000000000000;
const CASTLE_CHECK_BQ_MASK: u64 = 0x1c00000000000000;

pub trait KingMoveGen {
    fn generate_pseudolegal_king_moves(&self, moves: &mut SmallVec<[SimpleMove; 256]>);
    fn generate_king_vision(&self, turn: &Turn) -> u64;
}

impl KingMoveGen for Game {
    fn generate_king_vision(&self, turn: &Turn) -> u64 {
        let king = self.board.king(turn);

        // No need to include castling in king vision because it cannot attack with a castle

        KING_MOVES[(king | 0x8000000000000000).trailing_zeros() as usize]
    }

    fn generate_pseudolegal_king_moves(&self, moves: &mut SmallVec<[SimpleMove; 256]>) {
        match self.turn {
            Turn::White => {
                let king = self.board.white[5];
                let own_pieces = self.board.white[6];
                let occupied = self.board.all();
                let opponent_vision = self.generate_vision(&Turn::Black);

                // Ensures there's always a bit set and no index 64
                let destination_squares = KING_MOVES
                    [(king | 0x8000000000000000).trailing_zeros() as usize]
                    & !own_pieces
                    & !opponent_vision;

                let mut remaining_destinations = destination_squares;
                while remaining_destinations != 0 {
                    let next_destination = remaining_destinations & (!remaining_destinations + 1);
                    moves.push(SimpleMove::new(king, next_destination));
                    remaining_destinations &= remaining_destinations - 1;
                }

                if self.castling_rights.is_set(CastlingRights::WHITE_KINGSIDE)
                    && occupied & CASTLE_MOVE_WK_MASK == 0
                    && opponent_vision & CASTLE_CHECK_WK_MASK == 0
                {
                    moves.push(SimpleMove::new(king, 0x40))
                }

                if self.castling_rights.is_set(CastlingRights::WHITE_QUEENSIDE)
                    && occupied & CASTLE_MOVE_WQ_MASK == 0
                    && opponent_vision & CASTLE_CHECK_WQ_MASK == 0
                {
                    moves.push(SimpleMove::new(king, 0x4))
                }
            }
            Turn::Black => {
                let king = self.board.black[5];
                let own_pieces = self.board.black[6];
                let occupied = self.board.all();
                let opponent_vision = self.generate_vision(&Turn::White);

                // Ensures there's always a bit set and no index 64
                let destination_squares = KING_MOVES
                    [(king | 0x8000000000000000).trailing_zeros() as usize]
                    & !own_pieces
                    & !opponent_vision;

                let mut remaining_destinations = destination_squares;
                while remaining_destinations != 0 {
                    let next_destination = remaining_destinations & (!remaining_destinations + 1);
                    moves.push(SimpleMove::new(king, next_destination));
                    remaining_destinations &= remaining_destinations - 1;
                }

                if self.castling_rights.is_set(CastlingRights::BLACK_KINGSIDE)
                    && occupied & CASTLE_MOVE_BK_MASK == 0
                    && opponent_vision & CASTLE_CHECK_BK_MASK == 0
                {
                    moves.push(SimpleMove::new(king, 0x4000000000000000))
                }

                if self.castling_rights.is_set(CastlingRights::BLACK_QUEENSIDE)
                    && occupied & CASTLE_MOVE_BQ_MASK == 0
                    && opponent_vision & CASTLE_CHECK_BQ_MASK == 0
                {
                    moves.push(SimpleMove::new(king, 0x400000000000000))
                }
            }
        }
    }
}

pub const KING_MOVES: [u64; 64] = [
    770,
    1797,
    3594,
    7188,
    14376,
    28752,
    57504,
    49216,
    197123,
    460039,
    920078,
    1840156,
    3680312,
    7360624,
    14721248,
    12599488,
    50463488,
    117769984,
    235539968,
    471079936,
    942159872,
    1884319744,
    3768639488,
    3225468928,
    12918652928,
    30149115904,
    60298231808,
    120596463616,
    241192927232,
    482385854464,
    964771708928,
    825720045568,
    3307175149568,
    7718173671424,
    15436347342848,
    30872694685696,
    61745389371392,
    123490778742784,
    246981557485568,
    211384331665408,
    846636838289408,
    1975852459884544,
    3951704919769088,
    7903409839538176,
    15806819679076352,
    31613639358152704,
    63227278716305408,
    54114388906344448,
    216739030602088448,
    505818229730443264,
    1011636459460886528,
    2023272918921773056,
    4046545837843546112,
    8093091675687092224,
    16186183351374184448,
    13853283560024178688,
    144959613005987840,
    362258295026614272,
    724516590053228544,
    1449033180106457088,
    2898066360212914176,
    5796132720425828352,
    11592265440851656704,
    4665729213955833856,
];

#[allow(dead_code)]
const fn generate_all_king_moves() -> [u64; 64] {
    let mut moves = [0; 64];
    let mut i = 0;
    while i < 64 {
        let mut dest = 0;
        let king = 1 << i;

        dest |= (king & !A_FILE) >> 1;
        dest |= (king & !A_FILE & !EIGHTH_RANK) << 7;
        dest |= (king & !EIGHTH_RANK) << 8;
        dest |= (king & !H_FILE & !EIGHTH_RANK) << 9;
        dest |= (king & !H_FILE) << 1;
        dest |= (king & !H_FILE & !FIRST_RANK) >> 7;
        dest |= (king & !FIRST_RANK) >> 8;
        dest |= (king & !A_FILE & !FIRST_RANK) >> 9;

        moves[i] = dest;

        i += 1;
    }
    moves
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn calculate_simple_king_moves() {
        let game = Game::from_fen("8/6k1/8/8/8/1n6/KP6/8 w - - 0 1").unwrap();
        let mut moves = SmallVec::new();
        game.generate_pseudolegal_king_moves(&mut moves);

        assert_eq!(moves.len(), 3);

        assert!(moves.contains(&SimpleMove::from_algebraic("a2a3").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("a2b3").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("a2b1").unwrap()));
    }

    #[test]
    fn calculate_king_moves_castles_white() {
        let game =
            Game::from_fen("rn1qk1r1/pbpp1ppp/1p6/2b1p3/4P3/1PNP3N/PBPQBnPP/R3K2R w KQq - 0 1")
                .unwrap();
        let mut moves = SmallVec::new();
        game.generate_pseudolegal_king_moves(&mut moves);

        assert_eq!(moves.len(), 2);

        assert!(moves.contains(&SimpleMove::from_algebraic("e1f1").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("e1g1").unwrap()));
    }

    #[test]
    fn calculate_king_moves_castles_black_forfeit() {
        let game =
            Game::from_fen("rn1qk1r1/pbpp1ppp/1p6/2b1p3/4P3/1PNP3N/PBPQBnPP/R3K2R b KQq - 0 1")
                .unwrap();
        let mut moves = SmallVec::new();
        game.generate_pseudolegal_king_moves(&mut moves);

        assert_eq!(moves.len(), 2);

        assert!(moves.contains(&SimpleMove::from_algebraic("e8e7").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("e8f8").unwrap()));
    }

    #[test]
    fn king_cannot_put_self_in_check() {
        let game = Game::from_fen("8/8/8/4k3/1pb2p2/1r3P2/6NK/1n1Q2R1 b - - 0 1").unwrap();
        let mut moves = SmallVec::new();
        game.generate_pseudolegal_king_moves(&mut moves);

        assert_eq!(moves.len(), 3);

        assert!(moves.contains(&SimpleMove::from_algebraic("e5e6").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("e5f5").unwrap()));
        assert!(moves.contains(&SimpleMove::from_algebraic("e5f6").unwrap()));
    }

    #[test]
    fn king_cannot_castle_through_check() {
        let game = Game::from_fen("8/8/k7/6P1/2b5/8/8/4K2R w K - 0 1").unwrap();
        let mut moves = SmallVec::new();
        game.generate_pseudolegal_king_moves(&mut moves);

        assert!(!moves.contains(&SimpleMove::from_algebraic("e1g1").unwrap()));
    }

    #[test]
    fn king_cannot_castle_into_check() {
        let game = Game::from_fen("8/8/k7/6P1/3b4/8/8/4K2R w K - 0 1").unwrap();
        let mut moves = SmallVec::new();
        game.generate_pseudolegal_king_moves(&mut moves);

        assert!(!moves.contains(&SimpleMove::from_algebraic("e1g1").unwrap()));
    }

    #[test]
    fn king_cannot_castle_while_in_check() {
        let game = Game::from_fen("8/8/k7/6P1/1b6/8/8/4K2R w K - 0 1").unwrap();
        let mut moves = SmallVec::new();
        game.generate_pseudolegal_king_moves(&mut moves);

        assert!(!moves.contains(&SimpleMove::from_algebraic("e1g1").unwrap()));
    }

    #[test]
    fn cannot_castle_while_in_check_black() {
        let game =
            Game::from_fen("1n2k2r/4bpp1/3ppn1p/pB6/4P1P1/1PN1BP2/1P5P/2KR3R b k - 1 40").unwrap();
        let mut moves = SmallVec::new();
        game.generate_pseudolegal_king_moves(&mut moves);

        assert!(!moves.contains(&SimpleMove::from_algebraic("e8g8").unwrap()));
    }

    #[test]
    fn cannot_castle_while_in_check_white() {
        let moves_string =
            "g1f3 d7d5 b1c3 g8f6 d2d4 c7c5 c1g5 f6e4 e2e3 d8a5 f1b5 b8d7 d1b1 e4c3 b2c3 a5c3";
        let game = Game::from_uci_startpos(moves_string).unwrap();
        let mut moves = SmallVec::new();
        game.generate_pseudolegal_king_moves(&mut moves);

        assert!(!moves.contains(&SimpleMove::from_algebraic("e1g1").unwrap()));
    }

    #[ignore = "not a test"]
    #[test]
    fn king_moves() {
        println!("[{:?}]", generate_all_king_moves());
    }
}
