use std::cmp;

pub fn capacity(heights: &[u32]) -> u32 {
    let mut rmax = heights.to_vec();
    capacity_inner(heights, &mut rmax)
}

pub fn capacity_inner(heights: &[u32], rmax: &mut Vec<u32>) -> u32 {
    let len = heights.len();
    let mut max = 0;
    for i in (0..len).rev() {
        max = cmp::max(max, rmax[i]);
        rmax[i] = max;
    }
    let mut lmax = 0;
    let mut sum = 0;
    for i in 0..len {
        lmax = cmp::max(lmax, heights[i]);
        let min = cmp::min(lmax, rmax[i]);
        let delta = min - heights[i];
        sum += delta;
    }
    sum
}

#[test]
fn test_capacity() {
    let heights = vec![2, 6, 3, 5, 2, 8, 1, 4, 2, 2, 5, 3, 5, 7, 4, 1];
    assert_eq!(capacity(&heights), 35);
}
