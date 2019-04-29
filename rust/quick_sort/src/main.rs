#[macro_use]
extern crate clap;
extern crate num_cpus;
extern crate rayon;

use clap::{App, Arg};
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};

fn main() {
    let (qs_config, input_path) = parse_arguments();

    let f = BufReader::new(File::open(input_path).unwrap());

    let all_lines: Vec<String> = f
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();

    let mut inp_num_vec: Vec<u32> = all_lines.iter().map(|s| s.parse().unwrap()).collect();

    // Set the number of threads for rayon
    rayon::ThreadPoolBuilder::new()
        .num_threads(qs_config.num_threads as usize)
        .build_global()
        .unwrap();

    do_runs(&qs_config, &mut inp_num_vec.as_mut_slice());
}

// Configuration file, reflects command line options
#[derive(Copy, Clone)]
pub struct QSConfig {
    pub num_elements: u32,
    pub sort_order: u32,
    pub num_of_runs: u32,
    pub num_threads: u32,
    pub code_config: u32,
}

pub fn parse_arguments() -> (QSConfig, String) {
    // Create arugment matches
    let matches = App::new("Quick_Sort")
        .version("1.0")
        .author("Nishal & Supradeep")
        // Argument Parsing for all arguments of Quicksort
        .arg(
            Arg::with_name("NUM_ELEMENTS")
                .short("n")
                .long("num_vals")
                .value_name("NUM_ELEMENTS")
                .help("number of elements in the array to sort (default: 2000))"),
        )
        .arg(
            Arg::with_name("SORT_ORDER")
                .short("s")
                .long("sort")
                .value_name("SORT_ORDER")
                .help("enter 0 to sort ascending, 1 for descending (default: 0))"),
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
        .arg(
            Arg::with_name("FILE_PATH")
                .short("i")
                .long("input")
                .value_name("FILE_PATH")
                .help("enter input file path here (default: ./10000.txt)"),
        )
        .get_matches();

    // Find number of cpus available
    let max_threads = num_cpus::get();

    // Match and store all values of the arguments
    let num_elements = value_t!(matches.value_of("IMAGE_SIZE"), u32).unwrap_or(10000);
    let sort_order = value_t!(matches.value_of("SORT_ORDER"), u32).unwrap_or(0);
    let num_of_runs = value_t!(matches.value_of("NUM_OF_RUNS"), u32).unwrap_or(3);
    let num_threads =
        value_t!(matches.value_of("NUMBER_OF_THREADS"), u32).unwrap_or(max_threads as u32);
    let code_config = value_t!(matches.value_of("CODE"), u32).unwrap_or(0);
    let file_path = matches.value_of("FILE_PATH").unwrap_or("./10000.txt");

    // Check if values are correct for the mandelbrot program
    assert!(num_elements > 0);
    assert!((sort_order == 0) || (sort_order == 1));
    assert!(num_threads > 0);
    assert!(num_of_runs > 0);
    assert!(code_config < 3);
    assert_eq!(file_path.is_empty(), false);

    //
    println!("Configuration: \nnum_elements: {}, sort_order: {}, num_threads: {}, num_of_runs: {}, code_config: {}, input_path: {:?} \n",
        num_elements, sort_order, num_threads, num_of_runs, code_config, file_path);

    // Return the struct that can be used by the functions
    (
        QSConfig {
            num_elements: num_elements,
            sort_order: sort_order,
            num_threads: num_threads,
            num_of_runs: num_of_runs,
            code_config: code_config,
        },
        file_path.to_string(),
    )
}

pub fn do_runs(qs_config: &QSConfig, num_vec: &mut [u32]) {
    let num_runs = qs_config.num_of_runs;
    assert!(num_runs > 0);

    let mut stable_serial_time = Duration::new(10000, 0);
    let mut unstable_serial_time = Duration::new(10000, 0);
    let mut stable_par_time = Duration::new(10000, 0);
    let mut unstable_par_time = Duration::new(10000, 0);

    if (qs_config.code_config == 0) || (qs_config.code_config == 2) {
        // Stable serial sort
        for _ in 0..num_runs {
            let serial_start = Instant::now();
            stable_sort_serial(&qs_config, num_vec);
            let serial_end = Instant::now();

            stable_serial_time =
                std::cmp::min(stable_serial_time, serial_end.duration_since(serial_start));
        }

        // Check correctness
        assert!(is_sorted(&mut num_vec[..], qs_config.sort_order));

        println!(
            "[sort-stable-rust serial]: \t[{:?}] ms",
            stable_serial_time.as_micros() as f64 / 1000 as f64
        );

        // Unstable serial sort
        for _ in 0..num_runs {
            let serial_start = Instant::now();
            unstable_sort_serial(&qs_config, num_vec);
            let serial_end = Instant::now();

            unstable_serial_time = std::cmp::min(
                unstable_serial_time,
                serial_end.duration_since(serial_start),
            );
        }

        // Check correctness
        assert!(is_sorted(&mut num_vec[..], qs_config.sort_order));

        println!(
            "[sort-unstable-rust serial]: \t[{:?}] ms",
            unstable_serial_time.as_micros() as f64 / 1000 as f64
        );
    }

    if (qs_config.code_config == 0) || (qs_config.code_config == 1) {
        // Stable parallel sort
        for _ in 0..num_runs {
            let stable_par_start = Instant::now();
            stable_sort_par(&qs_config, num_vec);
            let stable_par_end = Instant::now();

            stable_par_time = std::cmp::min(
                stable_par_time,
                stable_par_end.duration_since(stable_par_start),
            );
        }

        println!(
            "[sort-rust stable]: \t\t[{:?}] ms",
            stable_par_time.as_micros() as f64 / 1000 as f64
        );
        if qs_config.code_config == 0 {
            println!(
                "++++ \t\t({:.2}x speedup from {:?} threads)\n",
                stable_serial_time.as_micros() as f64 / stable_par_time.as_micros() as f64,
                qs_config.num_threads
            );
        }

        // Unstable parallel sort
        for _ in 0..num_runs {
            let stable_par_start = Instant::now();
            stable_sort_par(&qs_config, num_vec);
            let stable_par_end = Instant::now();

            unstable_par_time = std::cmp::min(
                unstable_par_time,
                stable_par_end.duration_since(stable_par_start),
            );
        }

        println!(
            "[sort-rust stable]: \t\t[{:?}] ms",
            unstable_par_time.as_micros() as f64 / 1000 as f64
        );
        if qs_config.code_config == 0 {
            println!(
                "++++ \t\t({:.2}x speedup from {:?} threads)\n",
                unstable_serial_time.as_micros() as f64 / unstable_par_time.as_micros() as f64,
                qs_config.num_threads
            );
        }
    }
}

/*************************************
 * Sort functions
 *************************************/
// The serial version of the sorting
pub fn stable_sort_serial(qs_config: &QSConfig, num_vec: &mut [u32]) {
    if qs_config.sort_order == 0 {
        num_vec.sort();
    } else {
        num_vec.sort_by(|a, b| b.cmp(a));
    }
}

pub fn unstable_sort_serial(qs_config: &QSConfig, num_vec: &mut [u32]) {
    if qs_config.sort_order == 0 {
        num_vec.sort_unstable();
    } else {
        num_vec.sort_unstable_by(|a, b| b.cmp(a));
    }
}

// The serial version of the mandelbrot set calculation.
pub fn stable_sort_par(qs_config: &QSConfig, num_vec: &mut [u32]) {
    if qs_config.sort_order == 0 {
        num_vec.par_sort();
    } else {
        num_vec.par_sort_by(|a, b| b.cmp(a));
    }
}

// The serial version of the mandelbrot set calculation.
pub fn unstable_sort_par(qs_config: &QSConfig, num_vec: &mut [u32]) {
    if qs_config.sort_order == 0 {
        num_vec.par_sort_unstable();
    } else {
        num_vec.par_sort_unstable_by(|a, b| b.cmp(a));
    }
}

// Sort Checker
pub fn is_sorted<T: Send + Ord>(v: &[T], sort_order: u32) -> bool {
    if sort_order == 0 {
        (1..v.len()).all(|i| v[i - 1] <= v[i])
    } else {
        (1..v.len()).all(|i| v[i - 1] >= v[i])
    }
}
