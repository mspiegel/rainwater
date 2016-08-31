extern crate rainwater;

use rainwater::cpu_scan_rayon;

fn main() {
    let heights = vec![1; 1_000_000];
    for _ in 0..100_000 {
        cpu_scan_rayon::capacity(&heights, 32);
    }

}
