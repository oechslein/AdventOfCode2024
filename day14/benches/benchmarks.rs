use day14::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn day14_part1() {
    day14_part1::process(divan::black_box(include_str!(
        "../input1.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn day14_part2() {
    day14_part2::process(divan::black_box(include_str!(
        "../input2.txt",
    )))
    .unwrap();
}