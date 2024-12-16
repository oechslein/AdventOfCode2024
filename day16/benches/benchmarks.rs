use day16::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn day16_part1() {
    day16_part1::process(divan::black_box(&include_str!("../input1.txt").replace('\r', "")))
    .unwrap();
}

#[divan::bench]
fn day16_part2() {
    day16_part2::process(divan::black_box(&include_str!("../input2.txt").replace('\r', "")))
    .unwrap();
}