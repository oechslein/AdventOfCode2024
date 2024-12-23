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

use std::fmt::Debug;
use std::ops::RangeInclusive;
use std::path::PathBuf;
use std::time::Instant;

pub use num::integer::div_rem;
pub use num::integer::gcd;
pub use num::integer::lcm;

// parallel split iterator: https://tavianator.com/2022/parallel_graph_search.html
pub use spliter::{ParallelSpliterator, Spliterator};

//use itertools::Itertools;

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

/// A macro to cache the result of an expression which is multithreading safe.
///
/// # Arguments
/// * `$cache_type` - The type of the cache (e.g. `FxHashMap<String, usize>`)
/// * `$cache_init` - The initial value for the cache (e.g. `FxHashMap::default()`)
/// * `$key` - The key (as expression) to use for caching
/// * `$expression` - The expression to evaluate and cache
#[macro_export]
macro_rules! cache_it {
    ($cache_type:ty, $cache_init:expr, $key:expr, $expression:expr) => {{
        // LazyLock to initialize a static RwLock with the cache
        // RwLock is used to allow concurrent reads while ensuring exclusive writes
        static CACHE: std::sync::LazyLock<std::sync::RwLock<$cache_type>> =
            std::sync::LazyLock::new(|| std::sync::RwLock::new($cache_init));

        let key = ($key);
        // Lock the cache in this block
        {
            let cache = CACHE.read().unwrap();
            if let Some(value) = cache.get(&key) {
                return value.clone();
            }
        }

        let value = ($expression);

        // Lock the cache in this block
        {
            let mut cache = CACHE.write().unwrap();
            cache.insert(key, value.clone());
            value
        }
    }};
}

/// A macro to cache the result of an expression which is multithreading safe using a FxHashMap.
///
/// # Arguments
/// * `$cache_key_type` - The type of the key for the FxHashMap
/// * `$cache_value_type` - The type of the value for the FxHashMap
/// * `$key` - The key (as expression) to use for caching
/// * `$expression` - The expression to evaluate and cache
#[macro_export]
macro_rules! cache_it_with_fxhashmap {
    ($cache_key_type:ty, $cache_value_type:ty, $key:expr, $expression:expr) => {{
        utils::cache_it!(fxhash::FxHashMap<$cache_key_type, $cache_value_type>, fxhash::FxHashMap::default(), $key, $expression)
    }};
}
