use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::time::{Duration, Instant};

///
///
/// # Arguments
///
/// * `fun`: A function pointer or closure, taking a `&std` as arg.
///
/// returns: The value of type [`T`] returned by running `fun` on `input.txt` in the project directory.
///
/// # Examples
///
/// ```
/// use my_helpers::solve;
///
/// println!("Length of s: {}" , solve(|s| s.len()));
/// ```
/// ```
/// use my_helpers::solve;
///
/// fn stringify(input: &str) -> String { input.to_string() }
/// println!("s as String: {}" , solve(stringify));
/// ```
pub fn solve<F, T>(fun: F) -> T
where
    F: Fn(&str) -> T,
    T: Display,
{
    let mut input = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    fun(input.trim())
}

/**
* The struct returned by [`benchmark`].
* Implements [`core::fmt::Debug`].
*/
#[allow(dead_code)]
#[derive(Debug)]
pub struct Benchmarks {
    average: Duration,
    min: Duration,
    max: Duration,
    runs: u16,
}

impl From<Vec<Duration>> for Benchmarks {
    fn from(value: Vec<Duration>) -> Self {
        Self {
            runs: value.len() as u16,
            average: value.iter().sum::<Duration>() / value.len() as u32,
            min: value.iter().min().unwrap().clone(),
            max: value.iter().max().unwrap().clone(),
        }
    }
}

///
///
/// # Arguments
///
/// * `fun`: A function pointer or closure, taking a `&str` as arg.
/// * `runs`: The amount of times to run `fun`.
///
/// returns: A [`Benchmarks`] containing the times it took to run `fun` on `input.txt` in the project directory.
///
/// # Examples
///
/// ```
/// use my_helpers::benchmark;
///
/// println!("{:?} measured for calculating the length of s.", benchmark(|s| s.len(), 255));
/// ```
/// ```
/// use my_helpers::benchmark;
///
/// fn stringify(input: &str) -> String { input.to_string() }
/// println!("{:?} measured for cloning into a String.", benchmark(stringify, 255));
/// ```
pub fn benchmark<'a, F, T>(fun: F, runs: u16) -> Benchmarks
where
    F: Fn(&str) -> T,
    T: Display,
{
    let mut input = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let durations = (0..runs)
        .map(|_| {
            let start = Instant::now();
            let _ = fun(input.trim());
            start.elapsed()
        })
        .collect::<Vec<_>>();
    Benchmarks::from(durations)
}

#[macro_export]
macro_rules! bench_and_solve {
    ($f1:expr, $f2:expr, $runs:expr) => {
        println!("Solution: {}", self::solve($f1));
        println!("{:?}", self::benchmark($f1, $runs));
        println!("Solution: {}", self::solve($f2));
        println!("{:?}", self::benchmark($f2, $runs));
    };
}
