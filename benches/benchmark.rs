#[macro_use]

extern crate bencher;
extern crate rainwater;

use bencher::Bencher;

use rainwater::*;

fn cpu_scan_seq_imperative(bench: &mut Bencher) {
    let heights = vec![1; 100_000];

    bench.iter(|| {
        cpu_scan_seq_imperative::capacity(&heights)
    });
}

fn cpu_scan_seq_functional(bench: &mut Bencher) {
    let heights = vec![1; 100_000];

    bench.iter(|| {
        cpu_scan_seq_functional::capacity(&heights)
    });
}

fn cpu_scan_rayon(bench: &mut Bencher) {
    let heights = vec![1; 100_000];

    bench.iter(|| {
        cpu_scan_rayon::capacity(&heights, 32)
    });
}

benchmark_group!(benches, cpu_scan_seq_imperative, cpu_scan_seq_functional, cpu_scan_rayon);
benchmark_main!(benches);
