use day04::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn day04_part1() {
    day04_part1::process(divan::black_box(include_str!(
        "../input1.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn day04_part2() {
    day04_part2::process(divan::black_box(include_str!(
        "../input2.txt",
    )))
    .unwrap();
}