use arrayvec::ArrayVec;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use myco_chess_engine::{
    game::game::Game, hash::zobrist::ZobristHash, magic::initialize_magic_maps,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("zobrist", |b| {
        initialize_magic_maps();
        let games: ArrayVec<Game, 100> = FEN_STRS
            .into_iter()
            .map(|s| Game::from_fen(s).unwrap())
            .collect();

        b.iter(|| {
            for game in games.iter() {
                let _ = black_box(game.zobrist());
            }
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

const FEN_STRS: [&str; 100] = [
    "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1",
    "rnbqkbnr/pppp1ppp/4p3/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2",
    "rnbqkbnr/pppp1ppp/4p3/8/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 2",
    "rnbqkbnr/ppp2ppp/4p3/3p4/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 3",
    "rnbqkbnr/ppp2ppp/4p3/3p4/3PP3/8/PPPN1PPP/R1BQKBNR b KQkq - 1 3",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/3PP3/8/PPPN1PPP/R1BQKBNR w KQkq - 2 4",
    "rnbqkb1r/ppp2ppp/4pn2/3pP3/3P4/8/PPPN1PPP/R1BQKBNR b KQkq - 0 4",
    "rnbqkb1r/pppn1ppp/4p3/3pP3/3P4/8/PPPN1PPP/R1BQKBNR w KQkq - 1 5",
    "rnbqkb1r/pppn1ppp/4p3/3pP3/3P1P2/8/PPPN2PP/R1BQKBNR b KQkq - 0 5",
    "rnbqkb1r/pp1n1ppp/4p3/2ppP3/3P1P2/8/PPPN2PP/R1BQKBNR w KQkq - 0 6",
    "rnbqkb1r/pp1n1ppp/4p3/2ppP3/3P1P2/2P5/PP1N2PP/R1BQKBNR b KQkq - 0 6",
    "r1bqkb1r/pp1n1ppp/2n1p3/2ppP3/3P1P2/2P5/PP1N2PP/R1BQKBNR w KQkq - 1 7",
    "r1bqkb1r/pp1n1ppp/2n1p3/2ppP3/3P1P2/2P2N2/PP4PP/R1BQKBNR b KQkq - 2 7",
    "r1bqkb1r/pp1n1ppp/2n1p3/3pP3/3p1P2/2P2N2/PP4PP/R1BQKBNR w KQkq - 0 8",
    "r1bqkb1r/pp1n1ppp/2n1p3/3pP3/3P1P2/5N2/PP4PP/R1BQKBNR b KQkq - 0 8",
    "r1bqkb1r/pp1n2pp/2n1pp2/3pP3/3P1P2/5N2/PP4PP/R1BQKBNR w KQkq - 0 9",
    "r1bqkb1r/pp1n2pp/2n1pp2/3pP3/3P1P2/3B1N2/PP4PP/R1BQK1NR b KQkq - 1 9",
    "r1bqk2r/pp1n2pp/2n1pp2/3pP3/1b1P1P2/3B1N2/PP4PP/R1BQK1NR w KQkq - 2 10",
    "r1bqk2r/pp1n2pp/2n1pp2/3pP3/1b1P1P2/3B1N2/PP1B2PP/R2QK1NR b KQkq - 3 10",
    "r1b1k2r/pp1n2pp/1qn1pp2/3pP3/1b1P1P2/3B1N2/PP1B2PP/R2QK1NR w KQkq - 4 11",
    "r1b1k2r/pp1n2pp/1qn1pp2/3pP3/1b1P1P2/3B1N2/PP1BN1PP/R2QK2R b KQkq - 5 11",
    "r1b1k2r/pp1n2pp/1qn1p3/3pp3/1b1P1P2/3B1N2/PP1BN1PP/R2QK2R w KQkq - 0 12",
    "r1b1k2r/pp1n2pp/1qn1p3/3pP3/1b1P4/3B1N2/PP1BN1PP/R2QK2R b KQkq - 0 12",
    "r1b2rk1/pp1n2pp/1qn1p3/3pP3/1b1P4/3B1N2/PP1BN1PP/R2QK2R w KQ - 1 13",
    "r1b2rk1/pp1n2pp/1qn1p3/3pP3/1b1P4/P2B1N2/1P1BN1PP/R2QK2R b KQ - 0 13",
    "r1b2rk1/pp1nb1pp/1qn1p3/3pP3/3P4/P2B1N2/1P1BN1PP/R2QK2R w KQ - 1 14",
    "r1b2rk1/pp1nb1pp/1qn1p3/3pP3/3P4/P2B1N2/1PQBN1PP/R3K2R b KQ - 2 14",
    "r1b3k1/pp1nb1pp/1qn1p3/3pP3/3P4/P2B1r2/1PQBN1PP/R3K2R w KQ - 0 15",
    "r1b3k1/pp1nb1pp/1qn1p3/3pP3/3P4/P2B1P2/1PQBN2P/R3K2R b KQ - 0 15",
    "r1b3k1/pp1nb1pp/1q2p3/3pP3/3n4/P2B1P2/1PQBN2P/R3K2R w KQ - 0 16",
    "r1b3k1/pp1nb1pp/1q2p3/3pP3/3N4/P2B1P2/1PQB3P/R3K2R b KQ - 0 16",
    "r1b3k1/pp1nb1pp/4p3/3pP3/3q4/P2B1P2/1PQB3P/R3K2R w KQ - 0 17",
    "r1b3k1/pp1nb1pp/4p3/3pP3/3q4/P2B1P2/1PQB3P/2KR3R b - - 1 17",
    "r1b3k1/pp2b1pp/4p3/3pn3/3q4/P2B1P2/1PQB3P/2KR3R w - - 0 18",
    "r1b3k1/pp2b1pB/4p3/3pn3/3q4/P4P2/1PQB3P/2KR3R b - - 0 18",
    "r1b4k/pp2b1pB/4p3/3pn3/3q4/P4P2/1PQB3P/2KR3R w - - 1 19",
    "r1b4k/pp2b1pB/4p3/3pn3/3q4/P4P2/1PQB3P/1K1R3R b - - 2 19",
    "r1b4k/pp2b1pB/4p3/3pn3/7q/P4P2/1PQB3P/1K1R3R w - - 3 20",
    "r1b4k/pp2b1pB/4p3/3pn3/7q/P1B2P2/1PQ4P/1K1R3R b - - 4 20",
    "r1b4k/pp4pB/4pb2/3pn3/7q/P1B2P2/1PQ4P/1K1R3R w - - 5 21",
    "r1b4k/pp4pB/4pb2/3pn3/5P1q/P1B5/1PQ4P/1K1R3R b - - 0 21",
    "r1b4k/pp4pB/4pb2/3p4/2n2P1q/P1B5/1PQ4P/1K1R3R w - - 1 22",
    "r1b4k/pp4pB/4pB2/3p4/2n2P1q/P7/1PQ4P/1K1R3R b - - 0 22",
    "r1b4k/pp4pB/4pq2/3p4/2n2P2/P7/1PQ4P/1K1R3R w - - 0 23",
    "r1b4k/pp4p1/4pq2/3p4/2n2P2/P2B4/1PQ4P/1K1R3R b - - 1 23",
    "r1b4k/p5p1/4pq2/1p1p4/2n2P2/P2B4/1PQ4P/1K1R3R w - - 0 24",
    "r1b4k/p5p1/4pq2/1p1p4/2n2P2/P2B4/1P2Q2P/1K1R3R b - - 1 24",
    "r6k/p2b2p1/4pq2/1p1p4/2n2P2/P2B4/1P2Q2P/1K1R3R w - - 2 25",
    "r6k/p2b2p1/4pq2/1p1p4/2n2P2/P2B4/1P2Q2P/1K1R2R1 b - - 3 25",
    "r3b2k/p5p1/4pq2/1p1p4/2n2P2/P2B4/1P2Q2P/1K1R2R1 w - - 4 26",
    "r3b2k/p5p1/4pq2/1p1p4/2n2P2/P2B4/1P2Q2P/1K2R1R1 b - - 5 26",
    "r6k/p4bp1/4pq2/1p1p4/2n2P2/P2B4/1P2Q2P/1K2R1R1 w - - 6 27",
    "r6k/p4bp1/4pq2/1p1p4/2n2P2/P2B2R1/1P2Q2P/1K2R3 b - - 7 27",
    "2r4k/p4bp1/4pq2/1p1p4/2n2P2/P2B2R1/1P2Q2P/1K2R3 w - - 8 28",
    "2r4k/p4bp1/4pq2/1p1p4/2n2P2/P2B2R1/1P2Q2P/1K4R1 b - - 9 28",
    "2r4k/p4bp1/3npq2/1p1p4/5P2/P2B2R1/1P2Q2P/1K4R1 w - - 10 29",
    "2r4k/p4bR1/3npq2/1p1p4/5P2/P2B4/1P2Q2P/1K4R1 b - - 0 29",
    "2r4k/p4bR1/4pq2/1p1p1n2/5P2/P2B4/1P2Q2P/1K4R1 w - - 1 30",
    "2r4k/p4b2/4pq2/1p1p1nR1/5P2/P2B4/1P2Q2P/1K4R1 b - - 2 30",
    "7k/p1r2b2/4pq2/1p1p1nR1/5P2/P2B4/1P2Q2P/1K4R1 w - - 3 31",
    "7k/p1r2b2/4pq2/1p1p1BR1/5P2/P7/1P2Q2P/1K4R1 b - - 0 31",
    "7k/p1r2b2/5q2/1p1p1pR1/5P2/P7/1P2Q2P/1K4R1 w - - 0 32",
    "7k/p1r2b2/5q2/1p1p1p1R/5P2/P7/1P2Q2P/1K4R1 b - - 1 32",
    "rnbqkbnr/pp3ppp/4p3/2pp4/3PP3/8/PPPN1PPP/R1BQKBNR w KQkq - 0 4",
    "rnbqkbnr/pp3ppp/4p3/2pP4/3P4/8/PPPN1PPP/R1BQKBNR b KQkq - 0 4",
    "rnb1kbnr/pp3ppp/4p3/2pq4/3P4/8/PPPN1PPP/R1BQKBNR w KQkq - 0 5",
    "rnb1kbnr/pp3ppp/4p3/2pq4/3P4/5N2/PPPN1PPP/R1BQKB1R b KQkq - 1 5",
    "rnb1kbnr/pp3ppp/4p3/3q4/3p4/5N2/PPPN1PPP/R1BQKB1R w KQkq - 0 6",
    "rnb1kbnr/pp3ppp/4p3/3q4/2Bp4/5N2/PPPN1PPP/R1BQK2R b KQkq - 1 6",
    "rnb1kbnr/pp3ppp/3qp3/8/2Bp4/5N2/PPPN1PPP/R1BQK2R w KQkq - 2 7",
    "rnb1kbnr/pp3ppp/3qp3/8/2Bp4/5N2/PPPN1PPP/R1BQ1RK1 b kq - 3 7",
    "rnb1kb1r/pp3ppp/3qpn2/8/2Bp4/5N2/PPPN1PPP/R1BQ1RK1 w kq - 4 8",
    "rnb1kb1r/pp3ppp/3qpn2/8/2Bp4/1N3N2/PPP2PPP/R1BQ1RK1 b kq - 5 8",
    "r1b1kb1r/pp3ppp/2nqpn2/8/2Bp4/1N3N2/PPP2PPP/R1BQ1RK1 w kq - 6 9",
    "r1b1kb1r/pp3ppp/2nqpn2/8/2BN4/5N2/PPP2PPP/R1BQ1RK1 b kq - 0 9",
    "r1b1kb1r/pp3ppp/3qpn2/8/2Bn4/5N2/PPP2PPP/R1BQ1RK1 w kq - 0 10",
    "r1b1kb1r/pp3ppp/3qpn2/8/2BN4/8/PPP2PPP/R1BQ1RK1 b kq - 0 10",
    "r1b1kb1r/1p3ppp/p2qpn2/8/2BN4/8/PPP2PPP/R1BQ1RK1 w kq - 0 11",
    "r1b1kb1r/1p3ppp/p2qpn2/8/2B5/5N2/PPP2PPP/R1BQ1RK1 b kq - 1 11",
    "r1b1kb1r/5ppp/p2qpn2/1p6/2B5/5N2/PPP2PPP/R1BQ1RK1 w kq - 0 12",
    "r1b1kb1r/5ppp/p2qpn2/1p6/8/3B1N2/PPP2PPP/R1BQ1RK1 b kq - 1 12",
    "r3kb1r/1b3ppp/p2qpn2/1p6/8/3B1N2/PPP2PPP/R1BQ1RK1 w kq - 2 13",
    "r3kb1r/1b3ppp/p2qpn2/1p6/P7/3B1N2/1PP2PPP/R1BQ1RK1 b kq - 0 13",
    "r3kb1r/1b3ppp/p2qp3/1p6/P5n1/3B1N2/1PP2PPP/R1BQ1RK1 w kq - 1 14",
    "r3kb1r/1b3ppp/p2qp3/1p6/P5n1/3B1N2/1PP2PPP/R1BQR1K1 b kq - 2 14",
    "r3kb1r/1b3ppp/pq2p3/1p6/P5n1/3B1N2/1PP2PPP/R1BQR1K1 w kq - 3 15",
    "r3kb1r/1b3ppp/pq2p3/1p6/P5n1/3B1N2/1PP1QPPP/R1B1R1K1 b kq - 4 15",
    "r3k2r/1b3ppp/pq2p3/1pb5/P5n1/3B1N2/1PP1QPPP/R1B1R1K1 w kq - 5 16",
    "r3k2r/1b3ppp/pq2p3/1pb5/P5n1/3B1N2/1PP1QPPP/R1B2RK1 b kq - 6 16",
    "r3k2r/1b3ppp/pq2p3/2b5/Pp4n1/3B1N2/1PP1QPPP/R1B2RK1 w kq - 0 17",
    "r3k2r/1b3ppp/pq2p3/2b5/Pp4n1/3B1N1P/1PP1QPP1/R1B2RK1 b kq - 0 17",
    "r3k2r/1b3ppp/pq2pn2/2b5/Pp6/3B1N1P/1PP1QPP1/R1B2RK1 w kq - 1 18",
    "r3k2r/1b3ppp/pq2pn2/2b3B1/Pp6/3B1N1P/1PP1QPP1/R4RK1 b kq - 2 18",
    "r3k2r/1b3ppp/pq2p3/2b3Bn/Pp6/3B1N1P/1PP1QPP1/R4RK1 w kq - 3 19",
    "r3k2r/1b3ppp/pq2p3/2b4n/Pp6/3BBN1P/1PP1QPP1/R4RK1 b kq - 4 19",
    "r3k2r/1b3ppp/pq2p3/7n/Pp6/3BbN1P/1PP1QPP1/R4RK1 w kq - 0 20",
    "r3k2r/1b3ppp/pq2p3/7n/Pp6/3BQN1P/1PP2PP1/R4RK1 b kq - 0 20",
    "r3k2r/1b3ppp/p3p3/7n/Pp6/3BqN1P/1PP2PP1/R4RK1 w kq - 0 21",
    "r3k2r/1b3ppp/p3p3/7n/Pp6/3BPN1P/1PP3P1/R4RK1 b kq - 0 21",
    "r3k2r/1b3ppp/p3p3/8/Pp6/3BPNnP/1PP3P1/R4RK1 w kq - 1 22",
];
