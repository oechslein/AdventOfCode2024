use {{crate_name}}::*;
use divan::AllocProfiler;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();


fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn {{crate_name}}_part1() {
    {{crate_name}}_part1::process(divan::black_box(&include_str!("../input1.txt").replace('\r', "")))
    .unwrap();
}

#[divan::bench]
fn {{crate_name}}_part2() {
    {{crate_name}}_part2::process(divan::black_box(&include_str!("../input2.txt").replace('\r', "")))
    .unwrap();
}