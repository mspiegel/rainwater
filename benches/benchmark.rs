#[macro_use]

extern crate bencher;
extern crate rand;

extern crate rainwater;

use bencher::Bencher;
use rand::{Rng, SeedableRng, StdRng};

use rainwater::{cpu_seq_imperative,cpu_seq_functional};

fn cpu_seq_imperative(bench: &mut Bencher) {
    let seed: &[_] = &[1, 2, 3, 4];
    let mut random: StdRng = SeedableRng::from_seed(seed);
    let heights: Vec<u32> = random.gen_iter().take(1_000).collect();

    bench.iter(|| {
        cpu_seq_imperative::capacity(&heights)
    });
}

fn cpu_seq_functional(bench: &mut Bencher) {
    let seed: &[_] = &[1, 2, 3, 4];
    let mut random: StdRng = SeedableRng::from_seed(seed);
    let heights: Vec<u32> = random.gen_iter().take(1_000).collect();

    bench.iter(|| {
        cpu_seq_functional::capacity(&heights)
    });
}

benchmark_group!(benches, cpu_seq_imperative, cpu_seq_functional);
benchmark_main!(benches);
