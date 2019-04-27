#[macro_use]
extern crate clap;
extern crate crossbeam;
extern crate num_cpus;
extern crate rayon;

use clap::{App, Arg};
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};

fn main() {
    let fib_config = parse_arguments();

    // Set the number of threads for rayon
    rayon::ThreadPoolBuilder::new()
        .num_threads(fib_config.num_threads as usize)
        .build_global()
        .unwrap();

    do_runs(&fib_config);
}

// Configuration file, reflects command line options
#[derive(Copy, Clone)]
pub struct FibConfig {
    pub n: u64,
    pub num_of_runs: u32,
    pub num_threads: u32,
    pub code_config: u32,
}

pub fn do_runs(fib_config: &FibConfig) {
    let num_runs = fib_config.num_of_runs;
    assert!(num_runs > 0);

    let mut serial_time = Duration::new(100, 0);
    let mut stable_par_time = Duration::new(100, 0);
    // let mut row_parallel_time = Duration::new(100, 0);
    // let mut crossbeam_parallel_time = Duration::new(100, 0);

    let mut serial_fib_val = 0;
    let mut par1_val = 0;

    if (fib_config.code_config == 0) || (fib_config.code_config == 2) {
        for _ in 0..num_runs {
            let serial_start = Instant::now();
            serial_fib_val = fib_serial_iterative(fib_config.n);
            let serial_end = Instant::now();

            serial_time = std::cmp::min(serial_time, serial_end.duration_since(serial_start));
        }

        println!(
            "[fib-rust serial]: \t[{:?}] ms",
            serial_time.as_micros() as f64 / 1000 as f64
        );

        println!("{:?}", serial_fib_val);
    }

    // if (qs_config.code_config == 0) || (qs_config.code_config == 1) {
    //     for _ in 0..num_runs {
    //         let stable_par_start = Instant::now();
    //         unstable_sort_par(&qs_config, num_vec);
    //         let stable_par_end = Instant::now();

    //         stable_par_time = std::cmp::min(
    //             stable_par_time,
    //             stable_par_end.duration_since(stable_par_start),
    //         );
    //     }

    //     println!(
    //         "[sort-rust stable]: \t\t[{:?}] ms",
    //         stable_par_time.as_micros() as f64 / 1000 as f64
    //     );
    //     if qs_config.code_config == 0 {
    //         println!(
    //             "++++ \t\t({:.2}x speedup from {:?} threads)\n",
    //             serial_time.as_micros() as f64 / stable_par_time.as_micros() as f64,
    //             qs_config.num_threads
    //         );
    //     }
    // }

    //     //////////////////////
    //     // Row parallel Test

    //     for _ in 0..num_runs {
    //         let rayon_row_start = Instant::now();
    //         rayon_mandelbrot_row(&mandel_config, image);
    //         let rayon_row_end = Instant::now();

    //         row_parallel_time = std::cmp::min(
    //             row_parallel_time,
    //             rayon_row_end.duration_since(rayon_row_start),
    //         );
    //     }

    //     println!(
    //         "[mandelbrot-rust row]: \t\t\t[{:?}] ms",
    //         row_parallel_time.as_micros() as f64 / 1000 as f64
    //     );

    //     if mandel_config.code_config == 0 {
    //         println!(
    //             "++++ \t\t({:.2}x speedup from {:?} threads) \n",
    //             serial_time.as_micros() as f64 / row_parallel_time.as_micros() as f64,
    //             mandel_config.num_threads
    //         );
    //     }

    //     for _ in 0..num_runs {
    //         let crossbeam_row_start = Instant::now();
    //         crossbeam_manderlbrot_row(&mandel_config, image);
    //         let crossbeam_row_end = Instant::now();

    //         crossbeam_parallel_time = std::cmp::min(
    //             crossbeam_parallel_time,
    //             crossbeam_row_end.duration_since(crossbeam_row_start),
    //         );
    //     }

    //     println!(
    //         "[mandelbrot-rust crossbeam row]: \t[{:?}] ms",
    //         crossbeam_parallel_time.as_micros() as f64 / 1000 as f64
    //     );

    //     if mandel_config.code_config == 0 {
    //         println!(
    //             "++++ \t\t({:.2}x speedup from {:?} threads)\n",
    //             serial_time.as_micros() as f64 / crossbeam_parallel_time.as_micros() as f64,
    //             mandel_config.num_threads
    //         );
    //     }
    // }
}

pub fn parse_arguments() -> FibConfig {
    // Create arugment matches
    let matches = App::new("Fibonacci")
        .version("1.0")
        .author("Nishal & Supradeep")
        // Argument Parsing for all arguments of Quicksort
        .arg(
            Arg::with_name("N")
                .short("n")
                .long("num_vals")
                .value_name("N")
                .help("Number of fibonacci elements to calculate (default: 2000))"),
        )
        .arg(
            Arg::with_name("NUM_OF_RUNS")
                .short("r")
                .long("runs")
                .value_name("NUM_OF_RUNS")
                .help("number of repetitive runs (default: 1)"),
        )
        .arg(
            Arg::with_name("NUMBER_OF_THREADS")
                .short("t")
                .long("num_threads")
                .value_name("NUMBER_OF_THREADS")
                .help("number of threads to use (default: MAX_CPUS)"),
        )
        .get_matches();

    // Find number of cpus available
    let max_threads = num_cpus::get();

    // Match and store all values of the arguments
    let n = value_t!(matches.value_of("N"), u64).unwrap_or(1000);
    let num_of_runs = value_t!(matches.value_of("NUM_OF_RUNS"), u32).unwrap_or(3);
    let num_threads =
        value_t!(matches.value_of("NUMBER_OF_THREADS"), u32).unwrap_or(max_threads as u32);
    let code_config = value_t!(matches.value_of("CODE"), u32).unwrap_or(0);

    // Check if values are correct for the mandelbrot program
    assert!(n > 0);
    assert!(num_threads > 0);
    assert!(num_of_runs > 0);
    assert!(code_config < 3);

    //
    println!(
        "Configuration: \nN: {}, num_threads: {}, num_of_runs: {}, code_config: {}\n",
        n, num_threads, num_of_runs, code_config
    );

    // Return the struct that can be used by the functions
    FibConfig {
        n: n,
        num_threads: num_threads,
        num_of_runs: num_of_runs,
        code_config: code_config,
    }
}

pub fn fib_serial_iterative(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let c = a + b;
        a = b;
        b = c;
    }
    a
}

/// Compute the Fibonacci number recursively, using rayon::join.
/// The larger branch F(N-1) is computed first.
pub fn fib_join_12(n: u64) -> u64 {
    if n < 2 {
        return n;
    }

    let (a, b) = rayon::join(|| fib_join_12(n - 1), || fib_join_12(n - 2));
    a + b
}

/// Compute the Fibonacci number recursively, using rayon::join.
/// The smaller branch F(N-2) is computed first.
pub fn fib_join_21(n: u64) -> u64 {
    if n < 2 {
        return n;
    }

    let (a, b) = rayon::join(|| fib_join_21(n - 2), || fib_join_21(n - 1));
    a + b
}

/// Compute the Fibonacci number iteratively, using rayon::iter::split to parallelize.
fn fibonacci_split_iterative(n: u64) -> u64 {
    use rayon::iter::ParallelIterator;

    rayon::iter::split(n, |n| {
        if n < 2 {
            (n, None)
        } else {
            (n - 2, Some(n - 1))
        }
    })
    .map(fib_serial_iterative)
    .sum()
}
