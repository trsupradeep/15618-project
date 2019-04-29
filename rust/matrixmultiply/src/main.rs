#[macro_use]
extern crate clap;
extern crate crossbeam;
extern crate num_cpus;
extern crate rayon;

use clap::{App, Arg};
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::time::{Duration, Instant};

const MULT_CHUNK: usize = 1 * 1024;
const LINEAR_CHUNK: usize = 64 * 1024;

macro_rules! RM {
    ($row:expr, $col:expr, $W:expr) => {
        ($row * $W + $col) as usize
    };
}

fn main() {
    let matmul_config = parse_arguments();

    let mut rng = thread_rng();

    let num_mat_elements = matmul_config.size * matmul_config.size;

    let mut m_a: Vec<i32> = vec![1; num_mat_elements as usize];

    let mut m_b: Vec<i32> = vec![1; num_mat_elements as usize];

    // Set the number of threads for rayon
    rayon::ThreadPoolBuilder::new()
        .num_threads(matmul_config.num_threads as usize)
        .build_global()
        .unwrap();

    do_runs(&matmul_config, &mut m_a, &mut m_b);
}

// Configuration file, reflects command line options
#[derive(Copy, Clone)]
pub struct MatMulConfig {
    pub size: u64,
    pub num_threads: u32,
    pub num_of_runs: u32,
    pub code_config: u32,
}

pub fn do_runs(matmul_config: &MatMulConfig, m_a: &mut [i32], m_b: &mut [i32]) {
    let num_runs = matmul_config.num_of_runs;

    let mut serial_time = Duration::new(100, 0);
    let mut row_parallel_time = Duration::new(100, 0);
    let mut quad_parallel_time = Duration::new(100, 0);

    let num_mat_elements = matmul_config.size * matmul_config.size;

    let mut serial_m_c: Vec<i32> = vec![0; num_mat_elements as usize];

    if (matmul_config.code_config == 0) || (matmul_config.code_config == 2) {
        for _ in 0..num_runs {
            let serial_start = Instant::now();
            matmul_serial(matmul_config.size as usize, m_a, m_b, &mut serial_m_c[..]);
            // matmul_seq(m_a, m_b, &mut serial_m_c[..]);
            let serial_end = Instant::now();

            serial_time = std::cmp::min(serial_time, serial_end.duration_since(serial_start));
        }

        println!(
            "[matmul-rust serial]: \t[{:?}] ms",
            serial_time.as_micros() as f64 / 1000 as f64
        );
    }

    if (matmul_config.code_config == 0) || (matmul_config.code_config == 1) {
        let mut par_row_m_c: Vec<i32> = vec![0; num_mat_elements as usize];

        for _ in 0..num_runs {
            let par_start = Instant::now();
            matmul_par_row(matmul_config.size as usize, m_a, m_b, &mut par_row_m_c[..]);
            let par_end = Instant::now();

            row_parallel_time = std::cmp::min(row_parallel_time, par_end.duration_since(par_start));
        }

        println!(
            "[matmul-rust par_row]: \t[{:?}] ms",
            row_parallel_time.as_micros() as f64 / 1000 as f64
        );

        if matmul_config.code_config == 0 {
            assert_eq!(par_row_m_c, serial_m_c);
            println!(
                "++++ \t\t({:.2}x speedup from {:?} threads)\n",
                serial_time.as_micros() as f64 / row_parallel_time.as_micros() as f64,
                matmul_config.num_threads
            );
        }

        /////////////////////////////////////////////
        // let mut par_quad_m_c: Vec<i32> = vec![0; num_mat_elements as usize];

        // for _ in 0..num_runs {
        //     let par_start = Instant::now();
        //     matmulz(m_a, m_b, &mut par_quad_m_c[..]);
        //     let par_end = Instant::now();

        //     quad_parallel_time =
        //         std::cmp::min(quad_parallel_time, par_end.duration_since(par_start));
        // }

        // assert_eq!(par_quad_m_c, serial_m_c);

        // println!(
        //     "[matmul-rust par_row]: \t[{:?}] ms",
        //     quad_parallel_time.as_micros() as f64 / 1000 as f64
        // );

        // if matmul_config.code_config == 0 {
        //     println!(
        //         "++++ \t\t({:.2}x speedup from {:?} threads)\n",
        //         serial_time.as_micros() as f64 / quad_parallel_time.as_micros() as f64,
        //         matmul_config.num_threads
        //     );
        // }
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

pub fn matmul_serial(size: usize, m_a: &[i32], m_b: &[i32], m_c: &mut [i32]) {
    let iter_c = m_c.chunks_mut(size);
    let iter_a = m_a.chunks(size);

    iter_c
        .zip(iter_a)
        .enumerate()
        .for_each(|(_, (c_slice, a_slice))| {
            c_slice.iter_mut().enumerate().for_each(|(col, c)| {
                *c = a_slice
                    .iter()
                    .enumerate()
                    .map(|(index, addr)| *addr * m_b[RM!(index, col as usize, size)])
                    .sum()
            });
        });
}

pub fn matmul_par_row(size: usize, m_a: &[i32], m_b: &[i32], m_c: &mut [i32]) {
    let iter_c = m_c.par_chunks_mut(size);
    let iter_a = m_a.par_chunks(size);

    iter_c
        .zip(iter_a)
        .enumerate()
        .for_each(|(_, (c_slice, a_slice))| {
            c_slice.iter_mut().enumerate().for_each(|(col, c)| {
                *c = a_slice
                    .iter()
                    .enumerate()
                    .map(|(index, addr)| *addr * m_b[RM!(index, col as usize, size)])
                    .sum()
            });
        });
}

pub fn matmul_seq(m_a: &[i32], m_b: &[i32], dest: &mut [i32]) {
    // Multiply in row-major order.
    // D[i,j] = sum for all k A[i,k] * B[k,j]
    let bits = dest.len().trailing_zeros() / 2;
    let n = 1 << bits;
    for i in 0..n {
        for j in 0..n {
            let mut sum = 0;
            for k in 0..n {
                sum +=
                    unsafe { m_a.get_unchecked(i << bits | k) * m_b.get_unchecked(k << bits | j) };
            }
            dest[i << bits | j] = sum;
        }
    }
}

// Multiply two square power of two matrices, given in Z-order.
pub fn matmulz(a: &[i32], b: &[i32], dest: &mut [i32]) {
    if a.len() <= MULT_CHUNK {
        let bits = dest.len().trailing_zeros() / 2;
        let size = 1 << bits;
        matmul_serial(size, a, b, dest);
        return;
    }

    // Allocate uninitialized scratch space.
    let mut tmp = vec![0; dest.len()];

    let (a1, a2, a3, a4) = quarter_chunks(a);
    let (b1, b2, b3, b4) = quarter_chunks(b);
    {
        let (d1, d2, d3, d4) = quarter_chunks_mut(dest);
        let (t1, t2, t3, t4) = quarter_chunks_mut(&mut tmp[..]);
        // Multiply 8 submatrices
        join8(
            || matmulz(a1, b1, d1),
            || matmulz(a1, b2, d2),
            || matmulz(a3, b1, d3),
            || matmulz(a3, b2, d4),
            || matmulz(a2, b3, t1),
            || matmulz(a2, b4, t2),
            || matmulz(a4, b3, t3),
            || matmulz(a4, b4, t4),
        );
    }

    // Sum each quarter
    rmatsum(tmp.as_mut(), dest);
}

fn quarter_chunks<'a>(v: &'a [i32]) -> (&'a [i32], &'a [i32], &'a [i32], &'a [i32]) {
    let mid = v.len() / 2;
    let quarter = mid / 2;
    let (left, right) = v.split_at(mid);
    let (a, b) = left.split_at(quarter);
    let (c, d) = right.split_at(quarter);
    (a, b, c, d)
}

fn quarter_chunks_mut<'a>(
    v: &'a mut [i32],
) -> (&'a mut [i32], &'a mut [i32], &'a mut [i32], &'a mut [i32]) {
    let mid = v.len() / 2;
    let quarter = mid / 2;
    let (left, right) = v.split_at_mut(mid);
    let (a, b) = left.split_at_mut(quarter);
    let (c, d) = right.split_at_mut(quarter);
    (a, b, c, d)
}

// Any layout works, we're just adding by element.
fn rmatsum(src: &[i32], dest: &mut [i32]) {
    dest.par_iter_mut()
        .zip(src.par_iter())
        .for_each(|(d, s)| *d += *s);
}

fn rmatsub(src: &[i32], dest: &mut [i32]) {
    dest.par_iter_mut()
        .zip(src.par_iter())
        .for_each(|(d, s)| *d -= *s);
}

fn rcopy(src: &[i32], dest: &mut [i32]) {
    if dest.len() <= LINEAR_CHUNK {
        dest.copy_from_slice(src);
        return;
    }

    let mid = dest.len() / 2;
    let (s1, s2) = src.split_at(mid);
    let (d1, d2) = dest.split_at_mut(mid);
    rayon::join(|| rcopy(s1, d1), || rcopy(s2, d2));
}

fn join4<F1, F2, F3, F4, R1, R2, R3, R4>(f1: F1, f2: F2, f3: F3, f4: F4) -> (R1, R2, R3, R4)
where
    F1: FnOnce() -> R1 + Send,
    R1: Send,
    F2: FnOnce() -> R2 + Send,
    R2: Send,
    F3: FnOnce() -> R3 + Send,
    R3: Send,
    F4: FnOnce() -> R4 + Send,
    R4: Send,
{
    let ((r1, r2), (r3, r4)) = rayon::join(|| rayon::join(f1, f2), || rayon::join(f3, f4));
    (r1, r2, r3, r4)
}

fn join8<F1, F2, F3, F4, F5, F6, F7, F8, R1, R2, R3, R4, R5, R6, R7, R8>(
    f1: F1,
    f2: F2,
    f3: F3,
    f4: F4,
    f5: F5,
    f6: F6,
    f7: F7,
    f8: F8,
) -> (R1, R2, R3, R4, R5, R6, R7, R8)
where
    F1: FnOnce() -> R1 + Send,
    R1: Send,
    F2: FnOnce() -> R2 + Send,
    R2: Send,
    F3: FnOnce() -> R3 + Send,
    R3: Send,
    F4: FnOnce() -> R4 + Send,
    R4: Send,
    F5: FnOnce() -> R5 + Send,
    R5: Send,
    F6: FnOnce() -> R6 + Send,
    R6: Send,
    F7: FnOnce() -> R7 + Send,
    R7: Send,
    F8: FnOnce() -> R8 + Send,
    R8: Send,
{
    let (((r1, r2), (r3, r4)), ((r5, r6), (r7, r8))) = rayon::join(
        || rayon::join(|| rayon::join(f1, f2), || rayon::join(f3, f4)),
        || rayon::join(|| rayon::join(f5, f6), || rayon::join(f7, f8)),
    );
    (r1, r2, r3, r4, r5, r6, r7, r8)
}
