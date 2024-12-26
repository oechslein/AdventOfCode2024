use day25::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn day25_part1() {
    day25_part1::process(divan::black_box(
        &include_str!("../input1.txt").replace('\r', ""),
    ))
    .unwrap();
}
