use std::ops::Range;
use std::cmp;

fn left_max(heights: &[u32]) -> usize {
    let len = heights.len();
    let mut max = 0;
    for i in 0..len {
        if heights[i] > max {
            max = heights[i];
        } else {
            return i - 1;
        }
    }
    len
}

fn right_max(heights: &[u32]) -> usize {
    let len = heights.len();
    let mut max = 0;
    for i in (0..len).rev() {
        if heights[i] > max {
            max = heights[i];
        } else {
            return i + 1;
        }
    }
    len
}

struct Bar {
    left: u32,
    right: u32,
}

impl Bar {
    fn new(left: u32, right: u32) -> Self {
        return Bar {
            left: left,
            right: right,
        };
    }
}

pub fn capacity(heights: &[u32], tasks: usize) -> u32 {
    let (lpos, rpos) = (left_max(heights), right_max(heights));
    if lpos == rpos {
        return 0;
    }
    let heights = &heights[lpos..(rpos + 1)];
    let len = heights.len();
    let minsize = len / tasks;
    let pivots = create_pivots(heights, minsize, 0);
    do_capacity(heights,
                &pivots,
                Bar::new(heights[0], heights[len - 1]),
                0..len)
}

fn create_pivots(heights: &[u32], minsize: usize, offset: usize) -> Vec<usize> {
    let len = heights.len();
    if len <= minsize {
        let mut max = 0;
        let mut index = 0;
        for i in 0..len {
            if heights[i] > max {
                max = heights[i];
                index = i;
            }
        }
        vec![index + offset]
    } else {
        let mid = len / 2;
        let (left, right) = heights.split_at(mid);
        let mut p1 = create_pivots(left, minsize, offset);
        let p2 = create_pivots(right, minsize, offset + mid);
        p1.extend(p2);
        p1
    }
}

fn do_capacity(heights: &[u32], pivots: &[usize], bar: Bar, range: Range<usize>) -> u32 {
    if pivots.len() < 2 {
        do_capacity_seq(heights, bar, range)
    } else {
        let mut max = 0;
        let mut pivot_index = 0;
        for i in 0..pivots.len() {
            let next = pivots[i];
            let height = heights[next];
            if height > max {
                max = height;
                pivot_index = i;
            }
        }
        let index = pivots[pivot_index];
        let lpivot = &pivots[..pivot_index];
        let rpivot = &pivots[(pivot_index + 1)..];
        let lrange = (range.start)..index;
        let rrange = index..(range.end);
        do_capacity(heights,
                    lpivot,
                    Bar::new(bar.left, cmp::max(max, bar.right)),
                    lrange) +
        do_capacity(heights,
                    rpivot,
                    Bar::new(cmp::max(max, bar.left), bar.right),
                    rrange)
    }
}

fn do_capacity_seq(heights: &[u32], bar: Bar, range: Range<usize>) -> u32 {
    let max = cmp::min(bar.left, bar.right);
    let mut capacity = 0;
    for i in range {
        if max > heights[i] {
            capacity += max - heights[i];
        }
    }
    capacity
}

#[test]
fn test_capacity() {
    let heights = vec![2, 6, 3, 5, 2, 8, 1, 4, 2, 2, 5, 3, 5, 7, 4, 1];
    assert_eq!(capacity(&heights, 8), 35);
}
