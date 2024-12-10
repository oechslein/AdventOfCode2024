use day10::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn day10_part1() {
    day10_part1::process(divan::black_box(include_str!(
        "../input1.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn day10_part2() {
    day10_part2::process(divan::black_box(include_str!(
        "../input2.txt",
    )))
    .unwrap();
}