use day17::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn day17_part1() {
    day17_part1::process(divan::black_box(&include_str!("../input1.txt").replace('\r', "")))
    .unwrap();
}

#[divan::bench]
fn day17_part2() {
    day17_part2::process(divan::black_box(&include_str!("../input2.txt").replace('\r', "")))
    .unwrap();
}