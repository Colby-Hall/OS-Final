use std::io;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

mod matrix_builder;

fn main() {
    let dimensions = user_input();
    let thread_number = 4;
    let runs = 100;

    let mut st_times_vec = vec![0f64; 0];
    let mut mt_times_vec = vec![0f64; 0];

    for _ in 0..runs {
        multiplication_test(
            dimensions,
            thread_number,
            &mut st_times_vec,
            &mut mt_times_vec,
        );
    }

    for e in &st_times_vec {
        println!("{}", e);
    }

    //assert_eq!(st_result, mt_result);
}

fn user_input() -> usize {
    println!("Input one number for the nxn matrix.");

    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Oops");

    return input.trim().parse::<usize>().unwrap();
}

fn computation_time(now: Instant, time_vec: &mut Vec<f64>) {
    let elapsed_time = now.elapsed();
    let comp_time =
        elapsed_time.as_secs() as f64 + (elapsed_time.subsec_nanos() as f64) / 1000_000_000.0;
    time_vec.push(comp_time);
}

fn matrix_init(dimension: usize) -> Vec<Vec<usize>> {
    let mat = matrix_builder::Matrix {
        row_number: dimension,
        column_number: dimension,
    };

    return mat.fill_matrix();
}

fn matrix_mult(matrix_one: &Vec<Vec<usize>>, matrix_two: &Vec<Vec<usize>>, size: usize) {
    let mut result_matrix = vec![vec![0usize; size]; size];

    for i in 0..size {
        for j in 0..size {
            let mut prod = 0;
            for k in 0..size {
                prod += matrix_one[i][k].wrapping_mul(matrix_two[k][j]);
                //println!("{}", k);
                //println!("{}", prod);
            }
            //println!("{}", prod);
            result_matrix[i][j] = prod;
        }
    }
    //println!("{}", result_matrix[0][0]);
    /*
    for x in result_matrix.iter() {
        println!("{:?}", x);
    }
    */
}

fn thread_spawn_and_mult(
    matrix_one: Vec<Vec<usize>>,
    matrix_two: Vec<Vec<usize>>,
    size: usize,
    max_threads: usize,
) {
    let result_matrix = vec![vec![0usize; size]; size];

    let arc_results = Arc::new(Mutex::new(result_matrix));

    let mut handles = vec![];

    for _ in 0..max_threads {
        let clonem1 = matrix_one.clone();
        let clonem2 = matrix_two.clone();
        let clonem3 = arc_results.clone();
        let thread_count = handles.len();
        //println!("{}", thread_count);

        handles.push(thread::spawn(move || {
            multithreaded_mult(clonem1, clonem2, clonem3, size, thread_count, max_threads);
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    /*
    for x in arc_results.lock().unwrap().iter() {
        println!("{:?}", x);
    }
    */
}
fn multithreaded_mult(
    matrix_one: Vec<Vec<usize>>,
    matrix_two: Vec<Vec<usize>>,
    result_matrix: std::sync::Arc<std::sync::Mutex<std::vec::Vec<std::vec::Vec<usize>>>>,
    size: usize,
    start_pos: usize,
    max_threads: usize,
) {
    //println!("{}", start_pos);
    for i in (start_pos * size / max_threads)..(start_pos + 1) * size / max_threads {
        for j in 0..size {
            let mut prod = 0;
            for k in 0..size {
                prod += matrix_one[i][k].wrapping_mul(matrix_two[k][j]);
                //println!("{}", k);
                // println!("{}", prod);
            }
            // println!("{}", prod);
            result_matrix.lock().unwrap()[i][j] = prod;
        }
    }
}

fn multiplication_test(
    dimensions: usize,
    thread_number: usize,
    st_vec: &mut Vec<f64>,
    mt_vec: &mut Vec<f64>,
) {
    let matrix_one = matrix_init(dimensions);
    let matrix_two = matrix_init(dimensions);

    let now = Instant::now();

    matrix_mult(&matrix_one, &matrix_two, dimensions);

    computation_time(now, st_vec);

    //println!("Multiplication time (seconds) {}", st_time);

    let no2 = Instant::now();

    thread_spawn_and_mult(matrix_one, matrix_two, dimensions, thread_number);

    computation_time(no2, mt_vec);

    //println!("Multiplication time multithreaded (seconds) {}", mt_time);
}
