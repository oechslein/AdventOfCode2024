use day23::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn day23_part1() {
    day23_part1::process(divan::black_box(&include_str!("../input1.txt").replace('\r', "")))
    .unwrap();
}

#[divan::bench]
fn day23_part2() {
    day23_part2::process(divan::black_box(&include_str!("../input2.txt").replace('\r', "")))
    .unwrap();
}