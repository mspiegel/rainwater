use rayon::prelude::*;
use rayon;

use std::cmp;

fn seq_scan(elements: &mut [u32], iden: u32) {
    let len = elements.len();
    let mut max = iden;
    for i in 0..len {
        max = cmp::max(max, elements[i]);
        elements[i] = max;
    }
}

fn scan(elements: &mut Vec<u32>, numthreads: usize) {
    let len = elements.len();
    let chunksize = len / numthreads;
    let mut heads = vec![];
    elements.par_chunks(chunksize)
            .weight_max()
            .map(|chunk| chunk.iter().fold(0, |acc, &x| cmp::max(acc, x)))
            .collect_into(&mut heads);
    heads.insert(0, 0);
    seq_scan(&mut heads, 0);
    elements.par_chunks_mut(chunksize)
            .weight_max()
            .enumerate()
            .for_each(|(index, chunk)| seq_scan(chunk, heads[index]));
}

pub fn capacity(heights: &[u32], numthreads: usize) -> u32 {
    let mut lmax = heights.to_vec();
    let mut rmax = heights.to_vec();
    rmax.reverse();
    capacity_inner(heights, &mut lmax, &mut rmax, numthreads)
}

pub fn capacity_inner(heights: &[u32], lmax: &mut Vec<u32>, rmax: &mut Vec<u32>, numthreads: usize) -> u32 {
    let tail = heights.len() - 1;
    rayon::join(|| scan(lmax, numthreads),
                || scan(rmax, numthreads));
    heights.par_iter()
           .enumerate()
           .map(|(i, &height)| {
               let min = cmp::min(lmax[i], rmax[tail - i]);
               min - height
           })
           .sum()
}



#[test]
fn test_capacity() {
    let heights = vec![2, 6, 3, 5, 2, 8, 1, 4, 2, 2, 5, 3, 5, 7, 4, 1];
    assert_eq!(capacity(&heights, 2), 35);
}
