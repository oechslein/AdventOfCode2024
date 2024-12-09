use day_08::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn day_08_part1() {
    day_08_part1::process(divan::black_box(include_str!(
        "../input1.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn day_08_part2() {
    day_08_part2::process(divan::black_box(include_str!(
        "../input2.txt",
    )))
    .unwrap();
}