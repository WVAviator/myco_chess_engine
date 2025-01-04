use criterion::{black_box, criterion_group, criterion_main, Criterion};
use myco_chess_engine::{
    game::game::Game, magic::initialize_magic_maps, search::quiescence::QuiescenceSearch,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("search", |b| {
        initialize_magic_maps();
        let game =
            Game::from_fen("7k/p1r2b2/4pq2/1p1p1BR1/5P2/P7/1P2Q2P/1K4R1 b - - 0 31").unwrap();
        let search = QuiescenceSearch::new(&game, 6, 10);

        b.iter(|| {
            let _ = black_box(search.search());
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
