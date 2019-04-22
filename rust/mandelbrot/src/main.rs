#[macro_use]
extern crate clap;
extern crate crossbeam;
extern crate num_cpus;
extern crate rayon;

use clap::{App, Arg};
use rayon::prelude::*;
use std::time::{Instant, Duration};


fn main() {
    let mandel_config = parse_arguments();

    // Create
    let mut image: Vec<u32> = vec![0; (mandel_config.img_size * mandel_config.img_size) as usize];

    // Set the number of threads for rayon
    rayon::ThreadPoolBuilder::new()
        .num_threads(mandel_config.num_threads as usize)
        .build_global()
        .unwrap();

    do_runs(&mandel_config, &mut image);
}

// Configuration file, reflects command line options
#[derive(Copy, Clone)]
pub struct MandelConfig {
    pub re1: f32,
    pub re2: f32,
    pub img1: f32,
    pub img2: f32,
    pub x_step: f32,
    pub y_step: f32,
    pub max_iter: u32,
    pub img_size: u32,
    pub num_threads: u32,
    pub num_of_runs: u32,
    pub code_config: u32,
}

pub fn do_runs(mandel_config: &MandelConfig, image: &mut [u32]) {
    let num_runs = mandel_config.num_of_runs;
    assert!(num_runs > 0);

    let mut serial_time = Duration::new(100, 0);
    let mut pixel_parallel_time = Duration::new(100, 0);
    let mut row_parallel_time = Duration::new(100, 0);
    let mut crossbeam_parallel_time = Duration::new(100, 0);

    if (mandel_config.code_config == 0) || (mandel_config.code_config == 2) {
        for _ in 0..num_runs {
            let serial_start = Instant::now();
            mandelbrot_serial(&mandel_config, image);
            let serial_end = Instant::now();

            serial_time = std::cmp::min(serial_time, serial_end.duration_since(serial_start));
        }

        println!(
            "[mandelbrot-rust serial]: \t[{:?}] ms",
            serial_time.as_micros() as f64 / 1000 as f64
        );
    }

    if (mandel_config.code_config == 0) || (mandel_config.code_config == 1) {

        for _ in 0..num_runs {
            let rayon_pixel_start = Instant::now();
            rayon_mandelbrot_pixel(&mandel_config, image);
            let rayon_pixel_end = Instant::now();

            pixel_parallel_time = std::cmp::min(pixel_parallel_time, rayon_pixel_end.duration_since(rayon_pixel_start));
        }

        println!(
            "[mandelbrot-rust pixel]: \t\t[{:?}] ms",
            pixel_parallel_time.as_micros() as f64 / 1000 as f64
        );
        if mandel_config.code_config == 0 {
            println!(
                "++++ \t\t({:.2}x speedup from {:?} threads)\n",
                serial_time.as_micros() as f64 / pixel_parallel_time.as_micros() as f64, mandel_config.num_threads
            );
        }

        //////////////////////
        // Row parallel Test

        for _ in 0..num_runs {
            let rayon_row_start = Instant::now();
            rayon_mandelbrot_row(&mandel_config, image);
            let rayon_row_end = Instant::now();

            row_parallel_time = std::cmp::min(row_parallel_time, rayon_row_end.duration_since(rayon_row_start));
        }

        println!(
            "[mandelbrot-rust row]: \t\t\t[{:?}] ms",
            row_parallel_time.as_micros() as f64 / 1000 as f64
        );

        if mandel_config.code_config == 0 {
            println!(
                "++++ \t\t({:.2}x speedup from {:?} threads) \n",
                serial_time.as_micros() as f64/ row_parallel_time.as_micros() as f64, mandel_config.num_threads
            );
        }

        for _ in 0..num_runs {
            let crossbeam_row_start = Instant::now();
            crossbeam_manderlbrot_row(&mandel_config, image);
            let crossbeam_row_end = Instant::now();

            crossbeam_parallel_time = std::cmp::min(crossbeam_parallel_time, crossbeam_row_end.duration_since(crossbeam_row_start));
        }

        println!(
            "[mandelbrot-rust crossbeam row]: \t[{:?}] ms",
            crossbeam_parallel_time.as_micros() as f64 / 1000 as f64
        );

        if mandel_config.code_config == 0 {
            println!(
                "++++ \t\t({:.2}x speedup from {:?} threads)\n",
                serial_time.as_micros() as f64/ crossbeam_parallel_time.as_micros() as f64, mandel_config.num_threads
            );
        }
    }
}

pub fn parse_arguments() -> MandelConfig {
    // Create arugment matches
    let matches = App::new("Mandelbrot_Rust")
        .version("1.0")
        .author("Nishal & Supradeep")
        // Argument Parsing for all arguments of Mandelbrot
        .arg(
            Arg::with_name("REAL1")
                .long("re0")
                .value_name("REAL1")
                .help("left real part (default: -2.167)"),
        )
        .arg(
            Arg::with_name("REAL2")
                .long("re1")
                .value_name("REAL2")
                .help("right real part (default: 1.167)"),
        )
        .arg(
            Arg::with_name("IMAGINARY1")
                .long("im0")
                .value_name("IMAGINARY1")
                .help("lower part (default: -1.50)"),
        )
        .arg(
            Arg::with_name("IMAGINARY2")
                .long("im1")
                .value_name("IMAGINARY2")
                .help("upper part (default: 1.50)"),
        )
        .arg(
            Arg::with_name("MAX_ITER")
                .long("max_iters")
                .value_name("MAX_ITER")
                .help("maximum number of iterations (default: 2048)"),
        )
        .arg(
            Arg::with_name("IMAGE_SIZE")
                .short("s")
                .long("img_size")
                .value_name("IMAGE_SIZE")
                .help("size of image in pixel (square, default: 4096, must be a power of two)"),
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
            Arg::with_name("VIEW_NUM")
                .short("v")
                .long("view")
                .value_name("VIEW_NUM")
                .help("the view number to observe (default: 1)"),
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
    let re1 = value_t!(matches.value_of("REAL1"), f32).unwrap_or(-2.167);
    let re2 = value_t!(matches.value_of("REAL2"), f32).unwrap_or(1.167);
    let img1 = value_t!(matches.value_of("IMAGINARY1"), f32).unwrap_or(-1.5);
    let img2 = value_t!(matches.value_of("IMAGINARY2"), f32).unwrap_or(1.5);
    let max_iter = value_t!(matches.value_of("MAX_ITER"), u32).unwrap_or(2048);
    let img_size = value_t!(matches.value_of("IMAGE_SIZE"), u32).unwrap_or(4096);
    let num_of_runs = value_t!(matches.value_of("NUM_OF_RUNS"), u32).unwrap_or(3);
    let num_threads =
        value_t!(matches.value_of("NUMBER_OF_THREADS"), u32).unwrap_or(max_threads as u32);
    let view = value_t!(matches.value_of("VIEW_NUM"), u32).unwrap_or(1);
    let code_config = value_t!(matches.value_of("CODE"), u32).unwrap_or(0);

    // Check if values are correct for the mandelbrot program
    assert!(re1 < re2);
    assert!(img1 < img2);
    assert!(max_iter > 0);
    assert!(img_size > 0);
    assert!(num_threads > 0);
    assert!(view < 7);
    assert!(num_of_runs > 0);
    assert!(code_config < 3);

    // Find new scaled values for view
    let (x0, x1, y0, y1) = scale_and_shift(re1, re2, img1, img2, view);

    //
    println!("Configuration: \nre1: {:.3}, re2: {:.3}, img1: {:.3}, img2: {:.3}, max_iter: {}, img_size: {}, num_threads: {}, num_of_runs: {}, view: {}, code_config: {} \n",
        x0, x1, y0, y1, max_iter, img_size, num_threads, num_of_runs, view, code_config);

    // Calculate the step size
    let x_step = (x1 - x0) / (img_size as f32);
    let y_step = (y1 - y0) / (img_size as f32);

    // Return the struct that can be used by the functions
    MandelConfig {
        re1: x0,
        re2: x1,
        img1: y0,
        img2: y1,
        x_step: x_step,
        y_step: y_step,
        max_iter: max_iter,
        img_size: img_size,
        num_threads: num_threads,
        num_of_runs: num_of_runs,
        code_config: code_config,
    }
}

// Function that shifts and scales according to the view given
pub fn scale_and_shift(
    inp_x0: f32,
    inp_x1: f32,
    inp_y0: f32,
    inp_y1: f32,
    view: u32,
) -> (f32, f32, f32, f32) {
    // Same Magic arrays as for C++ code for different views
    let scale = vec![0.01, 1.0, 0.015, 0.02, 0.02, 0.02, 0.002];
    let shift_x = vec![0.0, 0.0, -0.98, 0.35, 0.0, -1.5, -1.4];
    let shift_y = vec![0.0, 0.0, 0.30, 0.05, 0.73, 0.0, 0.0];

    // Convert u32 to usize
    let view_num = view as usize;

    // Create mutable object for input complex numbers
    let mut x0 = inp_x0;
    let mut x1 = inp_x1;
    let mut y0 = inp_y0;
    let mut y1 = inp_y1;

    // Performs scaling of the value
    x0 *= scale[view_num];
    x1 *= scale[view_num];
    y0 *= scale[view_num];
    y1 *= scale[view_num];

    // Performs Shifting on the value
    x0 += shift_x[view_num];
    x1 += shift_x[view_num];
    y0 += shift_y[view_num];
    y1 += shift_y[view_num];

    return (x0, x1, y0, y1);
}

/*************************************
 * Mandelbrot functions
 *************************************/
// The serial version of the mandelbrot set calculation.
pub fn mandelbrot_serial(mandel_config: &MandelConfig, image: &mut [u32]) {
    for y in 0..mandel_config.img_size {
        for x in 0..mandel_config.img_size {
            let xf = mandel_config.re1 + x as f32 * mandel_config.x_step;
            let yf = mandel_config.img1 + y as f32 * mandel_config.y_step;

            let index = ((y * mandel_config.img_size) + x) as usize;

            image[index] = mandel_iter(mandel_config.max_iter, xf, yf);
        }
    }
}

// Parallel version with Rayon using Pixel wise parallelism
pub fn rayon_mandelbrot_pixel(mandel_config: &MandelConfig, image: &mut [u32]) {
    image.par_iter_mut().enumerate().for_each(|(n, pixel)| {
        let y = (n as u32) / mandel_config.img_size;
        let x = (n as u32) - (y * mandel_config.img_size);

        let xf = mandel_config.re1 + x as f32 * mandel_config.x_step;
        let yf = mandel_config.img1 + y as f32 * mandel_config.y_step;

        *pixel = mandel_iter(mandel_config.max_iter, xf, yf);
    });
}

// Parallel version with Rayon using Row wise parallelism
pub fn rayon_mandelbrot_row(mandel_config: &MandelConfig, image: &mut [u32]) {
    image
        .par_chunks_mut(mandel_config.img_size as usize)
        .enumerate()
        .for_each(|(y, slice)| {
            for x in 0..mandel_config.img_size {
                let xf = mandel_config.re1 + x as f32 * mandel_config.x_step;
                let yf = mandel_config.img1 + y as f32 * mandel_config.y_step;

                slice[x as usize] = mandel_iter(mandel_config.max_iter, xf, yf);
            }
        });
}

pub fn crossbeam_manderlbrot_row(mandel_config: &MandelConfig, image: &mut [u32]) {
    crossbeam::scope(|scope| {
        for (y, slice) in image
            .chunks_mut(mandel_config.img_size as usize)
            .enumerate()
        {
            scope.spawn(move |_| {
                for x in 0..mandel_config.img_size {
                    let xf = mandel_config.re1 + x as f32 * mandel_config.x_step;
                    let yf = mandel_config.img1 + y as f32 * mandel_config.y_step;

                    slice[x as usize] = mandel_iter(mandel_config.max_iter, xf, yf);
                }
            });
        }
    })
    .unwrap();
}

// The inner iteration loop of the mandelbrot calculation
// See https://en.wikipedia.org/wiki/Mandelbrot_set
pub fn mandel_iter(max_iter: u32, c_re: f32, c_im: f32) -> u32 {
    let mut z_re = c_re;
    let mut z_im = c_im;

    let mut iter = 0;

    while ((z_re * z_re + z_im * z_im) <= 4.0) && (iter < max_iter) {
        let new_re = z_re * z_re - z_im * z_im;
        let new_im = 2.0 * z_re * z_im;
        
        z_re = c_re + new_re;
        z_im = c_im + new_im;
        iter = iter + 1;
    }
    iter
}