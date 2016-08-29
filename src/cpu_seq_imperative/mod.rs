use std::cmp;

fn scan_max_asc(input: &[u32], output: &mut [u32]) {
    let len = input.len();
    let mut max = 0;
    for i in 0..len {
        max = cmp::max(max, input[i]);
        output[i] = max;
    }
}

fn scan_max_dsc(input: &[u32], output: &mut [u32]) {
    let len = input.len();
    let mut max = 0;
    for i in (0..len).rev() {
        max = cmp::max(max, input[i]);
        output[i] = max;
    }
}

pub fn capacity(heights: Vec<u32>) -> u32 {
    let len = heights.len();
    let mut lmax = vec![0; len];
    let mut rmax = vec![0; len];
    scan_max_asc(&heights, &mut lmax);
    scan_max_dsc(&heights, &mut rmax);
    let mut sum = 0;
    for i in 0..len {
        let min = cmp::min(lmax[i], rmax[i]);
        let delta = min - heights[i];
        sum += delta;
    }
    sum
}

#[test]
fn test_capacity() {
    let heights = vec![2, 6, 3, 5, 2, 8, 1, 4, 2, 2, 5, 3, 5, 7, 4, 1];
    assert_eq!(capacity(heights), 35);
}
