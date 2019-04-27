#[macro_use]
extern crate clap;
extern crate crossbeam;
extern crate num_cpus;
extern crate rayon;

use clap::{App, Arg};
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::time::{Duration, Instant};

macro_rules! RM {
    ($row:expr, $col:expr, $W:expr) => {
        ($row * $W + $col) as usize
    };
}

fn main() {
    let matmul_config = parse_arguments();

    let mut rng = rand::thread_rng();

    let num_mat_elements = matmul_config.size * matmul_config.size;

    // let A: Vec<i32> = (0..num_mat_elements)
    //     .map(|_| rng.gen_range(-10, 10))
    //     .collect();

    let mut A: Vec<i32> = vec![1; num_mat_elements as usize];
    A[1] = 2;
    A[2] = 3;
    A[3] = 4;

    let mut B = vec![1; num_mat_elements as usize];
    B[1] = 2;
    B[2] = 3;
    B[3] = 4;

    let mut C: Vec<i32> = vec![0; num_mat_elements as usize];

    // Set the number of threads for rayon
    rayon::ThreadPoolBuilder::new()
        .num_threads(matmul_config.num_threads as usize)
        .build_global()
        .unwrap();

    do_runs(&matmul_config, &mut A, &mut B, &mut C);
}

// Configuration file, reflects command line options
#[derive(Copy, Clone)]
pub struct MatMulConfig {
    pub size: u64,
    pub num_threads: u32,
    pub num_of_runs: u32,
    pub code_config: u32,
}

pub fn do_runs(matmul_config: &MatMulConfig, A: &mut [i32], B: &mut [i32], C: &mut [i32]) {
    let num_runs = matmul_config.num_of_runs;

    let mut serial_time = Duration::new(100, 0);
    let mut stable_par_time = Duration::new(100, 0);
    let mut row_parallel_time = Duration::new(100, 0);
    // let mut crossbeam_parallel_time = Duration::new(100, 0);

    if (matmul_config.code_config == 0) || (matmul_config.code_config == 2) {
        for _ in 0..num_runs {
            let serial_start = Instant::now();
            matmul_serial(matmul_config, A, B, C);
            let serial_end = Instant::now();

            serial_time = std::cmp::min(serial_time, serial_end.duration_since(serial_start));
        }

        // println!("A: {:?}", A);
        // println!("B: {:?}", B);
        // println!("C: {:?}", C);

        // Check correctness

        println!(
            "[matmul-rust serial]: \t[{:?}] ms",
            serial_time.as_micros() as f64 / 1000 as f64
        );
    }

    if (matmul_config.code_config == 0) || (matmul_config.code_config == 2) {
        for _ in 0..num_runs {
            let par_start = Instant::now();
            matmul_par(matmul_config, A, B, C);
            let par_end = Instant::now();

            row_parallel_time = std::cmp::min(row_parallel_time, par_end.duration_since(par_start));
        }

        println!(
            "[matmul-rust serial]: \t[{:?}] ms",
            row_parallel_time.as_micros() as f64 / 1000 as f64
        );

        if matmul_config.code_config == 0 {
            println!(
                "++++ \t\t({:.2}x speedup from {:?} threads)\n",
                serial_time.as_micros() as f64 / row_parallel_time.as_micros() as f64,
                matmul_config.num_threads
            );
        }
    }

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

pub fn parse_arguments() -> MatMulConfig {
    // Create arugment matches
    let matches = App::new("Matrix_Multiply")
        .version("1.0")
        .author("Nishal & Supradeep")
        // Argument Parsing for all arguments of Mandelbrot
        .arg(
            Arg::with_name("SIZE")
                .short("s")
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
    let size = value_t!(matches.value_of("SIZE"), u64).unwrap_or(1000);
    let num_of_runs = value_t!(matches.value_of("NUM_OF_RUNS"), u32).unwrap_or(1);
    let num_threads =
        value_t!(matches.value_of("NUMBER_OF_THREADS"), u32).unwrap_or(max_threads as u32);
    let code_config = value_t!(matches.value_of("CODE"), u32).unwrap_or(0);

    // Check if values are correct for the mandelbrot program
    assert!(size > 0);
    assert!(num_threads > 0);
    assert!(num_of_runs > 0);
    assert!(code_config < 3);

    //
    println!(
        "Configuration: \nsize={} num_threads: {}, num_of_runs: {}",
        size, num_threads, num_of_runs
    );

    // Return the struct that can be used by the functions
    MatMulConfig {
        size: size,
        num_threads: num_threads,
        num_of_runs: num_of_runs,
        code_config: code_config,
    }
}

pub fn matmul_serial(matmul_config: &MatMulConfig, A: &[i32], B: &[i32], C: &mut [i32]) {
    let size = matmul_config.size;
    let height = size;
    let width = size;
    for i in 0..height {
        for j in 0..width {
            for k in 0..size {
                C[RM!(i, j, size)] += A[RM!(i, k, size)] * B[RM!(k, j, size)];
            }
        }
    }
}

pub fn matmul_par(matmul_config: &MatMulConfig, A: &mut [i32], B: &mut [i32], C: &mut [i32]) {
    let size = matmul_config.size;
    let height = size;
    let width = size;
    println!("Parallel");

    let iter_a = A.par_chunks_mut(matmul_config.size as usize);
    let iter_c = C.par_chunks_mut(matmul_config.size as usize);

    iter_a
        .zip(iter_c)
        .enumerate()
        .for_each(|(_, (a_slice, c_slice))| {
            let size = matmul_config.size as usize;
            for col in 0..size {
                let mut c = 0 as i32;
                a_slice.iter().enumerate().for_each(|(index, addr)| {
                    c += *addr * B[RM!(index, col as usize, size as usize)];
                });
                c_slice[col as usize] = c;
            }
        });
}
