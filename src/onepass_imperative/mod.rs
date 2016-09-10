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

struct State {
    index: usize,
    forward: bool,
    max: u32,
}

impl State {
    fn step(&mut self) {
        if self.forward {
            self.index += 1;
        } else {
            self.index -= 1;
        }
    }

    fn next(heights: &[u32], lmax: usize, rmax: usize) -> State {
        if heights[lmax] <= heights[rmax] {
            State {
                index: lmax + 1,
                forward: true,
                max: heights[lmax],
            }
        } else {
            State {
                index: rmax - 1,
                forward: false,
                max: heights[rmax],
            }
        }
    }
}



pub fn capacity(heights: &[u32]) -> u32 {
    let len = heights.len();
    let mut capacity = 0;
    let mut lmax = left_max(heights);
    let mut rmax = right_max(heights);
    if lmax == len || rmax == len {
        return 0;
    }
    let mut state = State::next(heights, lmax, rmax);
    while (lmax < state.index) && (state.index < rmax) {
        if state.max >= heights[state.index] {
            capacity += state.max - heights[state.index];
            state.step();
        } else {
            if state.forward {
                lmax = state.index;
            } else {
                rmax = state.index;
            }
            state = State::next(heights, lmax, rmax);
        }
    }
    capacity
}

#[test]
fn test_capacity() {
    let heights = vec![2, 6, 3, 5, 2, 8, 1, 4, 2, 2, 5, 3, 5, 7, 4, 1];
    assert_eq!(capacity(&heights), 35);
}
