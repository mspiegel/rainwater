extern crate rainwater;

use rainwater::scan_rayon;

fn main() {
    let heights = vec![1; 1_000_000];
    for _ in 0..100_000 {
        scan_rayon::capacity(&heights, 32);
    }

}
