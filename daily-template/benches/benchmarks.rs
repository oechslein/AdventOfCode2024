use {{crate_name}}::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn {{crate_name}}_part1() {
    {{crate_name}}_part1::process(divan::black_box(include_str!(
        "../input1.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn {{crate_name}}_part2() {
    {{crate_name}}_part2::process(divan::black_box(include_str!(
        "../input2.txt",
    )))
    .unwrap();
}