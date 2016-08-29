use std::cmp;

fn scan_max(max: &mut u32, elem: u32) -> Option<u32> {
    *max = cmp::max(*max, elem);
    Some(*max)
}

pub fn capacity(heights: Vec<u32>) -> u32 {

    let left_max = heights.iter()
                          .scan(0, |max, &e| scan_max(max, e));

    let right_max = heights.iter()
                           .rev()
                           .scan(0, |max, &e| scan_max(max, e))
                           .collect::<Vec<u32>>()
                           .into_iter()
                           .rev();

    let min = left_max.zip(right_max).map(|(l, r)| cmp::min(l, r));

    let delta = heights.iter().zip(min).map(|(h, m)| m - h);

    delta.fold(0, |sum, x| sum + x)
}

#[test]
fn test_capacity() {
    let heights = vec![2, 6, 3, 5, 2, 8, 1, 4, 2, 2, 5, 3, 5, 7, 4, 1];
    assert_eq!(capacity(heights), 35);
}
