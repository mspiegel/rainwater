#[macro_use]

extern crate bencher;
extern crate rainwater;
extern crate rand;

use bencher::Bencher;
use rainwater::*;
use rand::{Rng, SeedableRng, StdRng};

fn gen_heights() -> Vec<u32> {
    let seed: &[_] = &[1, 2, 3, 4];
    let mut random: StdRng = SeedableRng::from_seed(seed);
    random.gen_iter::<u32>().take(100_000).map(|v| v % 1024).collect::<Vec<u32>>()
}

fn cpu_scan_imperative(bench: &mut Bencher) {
    let heights = gen_heights();

    bench.iter(|| {
        cpu_scan_imperative::capacity(&heights)
    });
}

fn cpu_scan_functional(bench: &mut Bencher) {
    let heights = gen_heights();

    bench.iter(|| {
        cpu_scan_functional::capacity(&heights)
    });
}

fn cpu_scan_rayon(bench: &mut Bencher) {
    let heights = gen_heights();

    bench.iter(|| {
        cpu_scan_rayon::capacity(&heights, 32)
    });
}

benchmark_group!(benches, cpu_scan_imperative, cpu_scan_functional, cpu_scan_rayon);
benchmark_main!(benches);
