use day18::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn day18_part1() {
    day18_part1::process(divan::black_box(&include_str!("../input1.txt").replace('\r', "")), 70+1, 1024)
    .unwrap();
}

#[divan::bench]
fn day18_part2() {
    day18_part2::process(divan::black_box(&include_str!("../input2.txt").replace('\r', "")), 70+1)
    .unwrap();
}