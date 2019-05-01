#[macro_use]
extern crate clap;
extern crate num_cpus;
extern crate rand;
extern crate rayon;

use clap::{App, Arg};
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::time::{Duration, Instant};

fn main() {
    let reduce_config = parse_arguments();

    // Set the number of threads for rayon
    rayon::ThreadPoolBuilder::new()
        .num_threads(reduce_config.num_threads as usize)
        .build_global()
        .unwrap();

    do_runs(&reduce_config);
}

// Configuration file, reflects command line options
#[derive(Copy, Clone)]
pub struct ReduceConfig {
    pub num_elements: u32,
    pub do_square: bool,
    pub num_of_runs: u32,
    pub num_threads: u32,
    pub code_config: u32,
}

pub fn parse_arguments() -> ReduceConfig {
    // Create arugment matches
    let matches = App::new("Reduction")
        .version("1.0")
        .author("Nishal & Supradeep")
        // Argument Parsing for all arguments of Quicksort
        .arg(
            Arg::with_name("NUM_ELEMENTS")
                .short("n")
                .long("num_vals")
                .value_name("NUM_ELEMENTS")
                .help("number of elements in the array to sort (default: 1M))"),
        )
        .arg(
            Arg::with_name("DO_SQUARE")
                .short("p")
                .long("pow")
                .value_name("DO_SQUARE")
                .help("enter 0 for reduce, 1 for square and reduce (default: 1))"),
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
        .arg(
            Arg::with_name("CODE")
                .short("c")
                .long("code")
                .value_name("CODE")
                .help("Enter 0 for all code, 1 for parallel only, 2 for serial only (default: 0)"),
        )
        .get_matches();

    // Find number of cpus available
    let max_threads = num_cpus::get();

    // Match and store all values of the arguments
    let num_elements = value_t!(matches.value_of("NUM_ELEMENTS"), u32).unwrap_or(1000000);
    let do_square = value_t!(matches.value_of("DO_SQUARE"), bool).unwrap_or(false);
    let num_of_runs = value_t!(matches.value_of("NUM_OF_RUNS"), u32).unwrap_or(3);
    let num_threads =
        value_t!(matches.value_of("NUMBER_OF_THREADS"), u32).unwrap_or(max_threads as u32);
    let code_config = value_t!(matches.value_of("CODE"), u32).unwrap_or(0);

    // Check if values are correct for the mandelbrot program
    assert!(num_elements > 0);
    assert!(num_threads > 0);
    assert!(num_of_runs > 0);
    assert!(code_config < 3);

    //
    println!("Configuration: \nnum_elements: {}, do_square: {}, num_threads: {}, num_of_runs: {}, code_config: {}\n",
        num_elements, do_square, num_threads, num_of_runs, code_config);

    // Return the struct that can be used by the functions
    ReduceConfig {
        num_elements: num_elements,
        do_square: do_square,
        num_threads: num_threads,
        num_of_runs: num_of_runs,
        code_config: code_config,
    }
}

pub fn do_runs(reduce_config: &ReduceConfig) {
    let num_runs = reduce_config.num_of_runs;
    assert!(num_runs > 0);

    let mut serial_reduction_time = Duration::new(10000, 0);
    let mut reduce_par_time = Duration::new(10000, 0);

    let range = Uniform::new(0.0, std::f64::MAX);

    let v_orig: Vec<f64> = thread_rng()
        .sample_iter(&range)
        .take(reduce_config.num_elements as usize)
        .collect();

    println!("Generated Random numbers");

    if (reduce_config.code_config == 0) || (reduce_config.code_config == 1) {
        // Serial reduction
        for _ in 0..num_runs {
            let mut v = v_orig.clone();
            let serial_start = Instant::now();
            println!(
                "Sum: {}",
                serial_reduction(reduce_config.do_square, &mut v[..])
            );
            let serial_end = Instant::now();

            serial_reduction_time = std::cmp::min(
                serial_reduction_time,
                serial_end.duration_since(serial_start),
            );
        }

        // Check correctness

        println!(
            "[reduce-rust serial]: \t[{:?}] ms",
            serial_reduction_time.as_micros() as f64 / 1000 as f64
        );
    }

    if (reduce_config.code_config == 0) || (reduce_config.code_config == 2) {
        // Parallel Reduce
        for _ in 0..num_runs {
            let mut v = v_orig.clone();
            let stable_par_start = Instant::now();
            println!(
                "ParallelSum: {}",
                par_reduction(reduce_config.do_square, &mut v[..])
            );
            let stable_par_end = Instant::now();

            reduce_par_time = std::cmp::min(
                reduce_par_time,
                stable_par_end.duration_since(stable_par_start),
            );
        }

        println!(
            "[reduce-rust par]: \t\t[{:?}] ms",
            reduce_par_time.as_micros() as f64 / 1000 as f64
        );
        if reduce_config.code_config == 0 {
            println!(
                "++++ \t\t({:.2}x speedup from {:?} threads)\n",
                serial_reduction_time.as_micros() as f64 / reduce_par_time.as_micros() as f64,
                reduce_config.num_threads
            );
        }
    }
}

/*************************************
 * Sort functions
 *************************************/
// The serial version of the reduction
pub fn serial_reduction(do_square: bool, num_vec: &mut [f64]) -> f64 {
    if do_square == false {
        num_vec.iter().sum()
    } else {
        num_vec.iter().map(|x| x * x).sum()
    }
}

// The serial version of the reduction
pub fn par_reduction(do_square: bool, num_vec: &mut [f64]) -> f64 {
    if do_square == false {
        num_vec.par_iter().sum()
    } else {
        num_vec.par_iter().map(|x| x * x).sum()
    }
}
