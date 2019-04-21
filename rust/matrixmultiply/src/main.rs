#[macro_use]
extern crate clap;
extern crate crossbeam;
extern crate num_cpus;
extern crate rayon;

use clap::{App, Arg};
use num::complex::Complex32;
use rayon::prelude::*;
use std::time::Instant;

// C = A * B
fn main() {
    println!("Hello, world!");
}

// Configuration file, reflects command line options
#[derive(Copy, Clone)]
pub struct MatMulConfig {
    pub size: u32,
    pub num_threads: u32,
    pub num_of_runs: u32,
    pub code_config: u32,
}

pub fn do_runs(matmul_config: &MatMulConfig, A: &mut [u32], B: &mut [u32], C: &mut [u32]) {
    let num_runs = matmul_config.num_of_runs;

    if (matmul_config.code_config % 2) == 0 {
        let serial_start = Instant::now();
        for r in 0..num_runs {
            println!("Serial Code Run {}", r + 1);
            // mandelbrot_serial(&mandel_config, image);
        }
        let serial_end = Instant::now();

        println!(
            "Serial Code Execution time: {:?}",
            serial_end.duration_since(serial_start) / num_runs
        );
    }
}

pub fn parse_arguments() -> MatMulConfig {
    // Create arugment matches
    let matches = App::new("Matrix_Multiply")
        .version("1.0")
        .author("Nishal & Supradeep")
        // Argument Parsing for all arguments of Mandelbrot
        .arg(
            Arg::with_name("SIZE")
                .long("s")
                .value_name("SIZE")
                .help("size of the square matrix (default: 1024)"),
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
    let size = value_t!(matches.value_of("SIZE"), u32).unwrap_or(1024);
    let num_of_runs = value_t!(matches.value_of("NUM_OF_RUNS"), u32).unwrap_or(1);
    let num_threads =
        value_t!(matches.value_of("NUMBER_OF_THREADS"), u32).unwrap_or(max_threads as u32);
    let view = value_t!(matches.value_of("VIEW_NUM"), u32).unwrap_or(1);
    let code_config = value_t!(matches.value_of("CODE"), u32).unwrap_or(0);

    // Check if values are correct for the mandelbrot program
    assert!(size > 0);
    assert!(num_threads > 0);
    assert!(num_of_runs > 0);
    assert!((code_config > 0) && (code_config < 3));

    //
    println!(
        "Configuration: \nsize={} num_threads: {}, num_of_runs: {}, view: {}",
        size, num_threads, num_of_runs, view
    );

    // Return the struct that can be used by the functions
    MatMulConfig {
        size: size,
        num_threads: num_threads,
        num_of_runs: num_of_runs,
        code_config: code_config,
    }
}

pub fn matmul_serial(matmul_config: &MatMulConfig) {
    let size = matmul_config.size;
    for i in 0..size {
        for j in 0..size {
            for k in 0..size {
                C[i][j] = A[i][k] * B[k][j];
            }
        }
    }
}
