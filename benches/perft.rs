use criterion::{criterion_group, criterion_main, Criterion};
use myco_chess_engine::{
    cache::{configure_global_cache, CacheConfiguration},
    game::game::Game,
    magic::initialize_magic_maps,
    performance::perft::perft,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("perft 6", |b| {
        initialize_magic_maps();
        configure_global_cache(CacheConfiguration::disabled());
        let game = Game::new_default();
        b.iter(|| perft(6, game))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
