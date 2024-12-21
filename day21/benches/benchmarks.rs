use day21::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn day21_part1() {
    day21_part1::process(divan::black_box(&include_str!("../input1.txt").replace('\r', "")))
    .unwrap();
}

#[divan::bench]
fn day21_part2() {
    day21_part2::process(divan::black_box(&include_str!("../input2.txt").replace('\r', "")))
    .unwrap();
}