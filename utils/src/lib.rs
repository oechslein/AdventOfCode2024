//! Various Utility functions

#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::enum_glob_use,
    clippy::many_single_char_names,
    clippy::must_use_candidate
)]
#![deny(missing_docs)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::doc_markdown)]

use std::cmp::Reverse;
use std::fmt::Debug;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;
use std::path::PathBuf;
use std::time::Instant;

pub use num::integer::div_rem;
pub use num::integer::gcd;
pub use num::integer::lcm;

// parallel split iterator: https://tavianator.com/2022/parallel_graph_search.html
pub use spliter::{ParallelSpliterator, Spliterator};

//use itertools::Itertools;

/// debug println x
pub fn printlnit<T: Debug>(x: &T) {
    println!("{x:?}");
}

/// Allows cargo run to be called in dayXY and in root folder
pub fn correct_folder(file_name: &str) -> PathBuf {
    let mut file_path = PathBuf::from(file_name);
    if !file_path.exists() {
        if let Some(file_name) = file_path.file_name() {
            file_path = PathBuf::from(file_name);
        }
    }
    file_path
}

/// Reads a file and return its content as a string
pub fn file_to_string(file_name: &str) -> String {
    fs::read_to_string(correct_folder(file_name)).unwrap()
}

/// Reads a file, splits per newline and returns an iterator
pub fn file_to_lines(file_name: &str) -> impl Iterator<Item = String> {
    BufReader::new(File::open(correct_folder(file_name)).unwrap())
        .lines()
        .map_while(Result::ok)
}

/// Converts an iterator with str to an iterator with "T"
pub fn convert_str_iter<'a, T>(
    input: impl Iterator<Item = &'a str> + 'a,
) -> impl Iterator<Item = T> + 'a
where
    T: std::str::FromStr,
    <T>::Err: Debug,
{
    input.map(|x| str_to(x))
}

/// Converts an str to a type (and unwraps it)
pub fn str_to<T>(input: &str) -> T
where
    T: std::str::FromStr,
    <T>::Err: Debug,
{
    str::parse::<T>(input).unwrap()
}

/// Converts item back from Reverse(item)
pub fn unreverse<T>(reversed_item: Reverse<T>) -> T {
    reversed_item.0
}

/// Splits given String split into chunks separated by empty lines
pub fn split_by_empty_lines<T>(contents: &str) -> impl Iterator<Item = T> + '_
where
    T: std::str::FromStr,
    <T>::Err: Debug,
{
    convert_str_iter::<T>(contents.split("\n\n"))
}

/// Splits given String split into chunks separated by empty lines
pub fn split_by_newline<T>(contents: &str) -> impl Iterator<Item = T> + '_
where
    T: std::str::FromStr,
    <T>::Err: Debug,
{
    convert_str_iter::<T>(contents.lines())
}

/// Splits given String, trim each lines, filters empty lines and parse each line into wished type
pub fn parse_input_items<T>(contents: &str) -> Vec<T>
where
    T: std::str::FromStr,
    <T>::Err: Debug,
{
    contents
        .split('\n')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|n| n.parse::<T>().unwrap())
        .collect()
}

/// Runs given function, prints result and used duration
pub fn with_measure<T: Debug>(title: &str, f: fn() -> T) -> T {
    let start = Instant::now();
    let res = f();
    let duration = start.elapsed();
    println!(
        "{} result: {:?} (elapsed time is: {:?} / {} millisecs)",
        title,
        res,
        duration,
        duration.as_secs_f32() * 1000.0
    );
    res
}

/// Returns a range from "from" to "to" (if to is smaller than from a range from "to" to "from" is returned)
pub fn inclusive_range_always<T: PartialOrd>(from: T, to: T) -> RangeInclusive<T> {
    if from < to {
        from..=to
    } else {
        to..=from
    }
}
