## Trapping Rain Water

This repository has several implementations of the "Trapping Rain Water"
programming problem. The implementations are written in the [Rust programming
language](https://www.rust-lang.org/en-US/). For an excellent introduction to
this problem I recommend watching the lecture [Four Solutions to a Trivial
Problem](https://www.youtube.com/watch?v=ftcIcn8AmSY) by Guy L. Steele Jr.

The problem is specified by a sequence of positive integers. The sequence
of numbers are interpreted as a histogram (shown below). Imagine that a bucket
of water is poured on top of the histogram. Water will be trapped into the
depressions in the histogram. The goal is to compute the amount of water
that is trapped by the histogram.

![Rain Water Example](/example.jpg)

The key insight to solving this problem is to note that the amount of water
at position _i_ is computed using the max of all heights to the left
and the max of all heights to the right. More precisely the
amount of water at position _i_ is equal to _min(left-max, right-max) -
height[i]_.

### Scan Operation: Imperative Style

[cpu_scan_imperative](/src/cpu_scan_imperative/mod.rs). This is the most
straightforward solution. Traverse the array from
left to right and compute the prefix maximum of the array. Traverse the
array from right to left to compute the suffix maximum of the array.
In a third pass of the array combine the prefix scan and the suffix scan
to compute the amount of water. If the right-to-left scan is performed first,
then the other two passes of the array can be collapsed to a single pass.

![Prefix Scan Approach](/scan.jpg)

This solution requires two passes over the array and the allocation of
a second array of equal length to store the right-to-left maximum values.

### Scan Operation: Functional Style

[cpu_scan_functional](/src/cpu_scan_functional/mod.rs). This is a rewrite of
the previous approach using a functional style of programming.

This solution requires two passes over the array and the allocation of
a second array of equal length to store the right-to-left maximum values.

### Scan Operation: Data Parallel

[cpu_scan_rayon](/src/cpu_scan_rayon/mod.rs). This implementation uses the
[rayon](https://github.com/nikomatsakis/rayon) data parallel library for rust.

### Single Pass

[cpu_single_pass](/src/cpu_single_pass/mod.rs).

### Benchmarks

The following benchmarks were run with rustc 1.11.0 with release build settings.

Input of 10,000 elements:

```
test cpu_scan_functional ... bench:      26,953 ns/iter (+/- 964)
test cpu_scan_imperative ... bench:      22,005 ns/iter (+/- 8,174)
test cpu_scan_rayon      ... bench:     194,073 ns/iter (+/- 6,844)
test cpu_single_pass     ... bench:       6,647 ns/iter (+/- 1,386)
```


Input of 100,000 elements:

```
test cpu_scan_functional ... bench:     257,157 ns/iter (+/- 32,946)
test cpu_scan_imperative ... bench:     226,523 ns/iter (+/- 25,608)
test cpu_scan_rayon      ... bench:     419,996 ns/iter (+/- 16,064)
test cpu_single_pass     ... bench:      62,508 ns/iter (+/- 1,345)
```

Input of 10,000,000 elements:

```
test cpu_scan_functional ... bench:  45,442,651 ns/iter (+/- 8,529,116)
test cpu_scan_imperative ... bench:  41,075,526 ns/iter (+/- 7,101,511)
test cpu_scan_rayon      ... bench:  55,053,761 ns/iter (+/- 3,840,731)
test cpu_single_pass     ... bench:   6,752,193 ns/iter (+/- 626,553)
```
