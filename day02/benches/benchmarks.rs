use day02::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn day02_part1() {
    day02_part1::process(divan::black_box(include_str!(
        "../input1.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn day02_part2() {
    day02_part2::process(divan::black_box(include_str!(
        "../input2.txt",
    )))
    .unwrap();
}