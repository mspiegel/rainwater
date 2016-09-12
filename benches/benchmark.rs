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
    random.gen_iter::<u32>().take(1_000_000).map(|v| v % 1024).collect::<Vec<u32>>()
}

fn scan_imperative(bench: &mut Bencher) {
    let heights = gen_heights();

    bench.iter(|| {
        scan_imperative::capacity(&heights)
    });
}

fn scan_imperative_inner(bench: &mut Bencher) {
    let heights = gen_heights();
    let mut rmax = heights.to_vec();

    bench.iter(|| {
        scan_imperative::capacity_inner(&heights, &mut rmax)
    });
}

fn scan_functional(bench: &mut Bencher) {
    let heights = gen_heights();

    bench.iter(|| {
        scan_functional::capacity(&heights)
    });
}

fn scan_rayon(bench: &mut Bencher) {
    let heights = gen_heights();
    bench.iter(|| {
        scan_rayon::capacity(&heights, 32)
    });
}

fn scan_rayon_inner(bench: &mut Bencher) {
    let heights = gen_heights();
    let mut lmax = heights.to_vec();
    let mut rmax = heights.to_vec();
    rmax.reverse();
    bench.iter(|| {
        scan_rayon::capacity_inner(&heights, &mut lmax, &mut rmax, 32)
    });
}

fn onepass_imperative(bench: &mut Bencher) {
    let heights = gen_heights();

    bench.iter(|| {
        onepass_imperative::capacity(&heights)
    });
}

benchmark_group!(benches,
    scan_imperative, scan_imperative_inner,
    scan_functional,
    scan_rayon, scan_rayon_inner,
    onepass_imperative);
benchmark_main!(benches);
