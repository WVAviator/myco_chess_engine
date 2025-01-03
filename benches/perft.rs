use criterion::{black_box, criterion_group, criterion_main, Criterion};
use myco_chess_engine::{
    game::game::Game,
    magic::{get_bishop_magic_map, get_rook_magic_map, initialize_magic_maps},
    performance::perft::perft,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("perft 6", |b| {
        initialize_magic_maps();
        let game = Game::new_default();
        b.iter(|| perft(6, game))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
