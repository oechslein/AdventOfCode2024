use day20::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn day20_part1() {
    day20_part1::process(divan::black_box(&include_str!("../input1.txt").replace('\r', "")))
    .unwrap();
}

#[divan::bench]
fn day20_part2() {
    day20_part2::process(divan::black_box(&include_str!("../input2.txt").replace('\r', "")))
    .unwrap();
}