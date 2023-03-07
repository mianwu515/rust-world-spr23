use std::thread;
use num_cpus;
use std::sync::{Arc, Mutex};

const ROWS_A: usize = 3;
//const COLS_A: usize = 2;
const ROWS_B: usize = 2;
const COLS_B: usize = 3;

fn multiply_rows(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>, c: &mut Vec<Vec<i32>>, start_row: usize, end_row: usize) {
    // Calculate the values of the rows in the range [start_row, end_row) of matrix C
    for i in start_row..end_row {
        for j in 0..COLS_B {
            for k in 0..ROWS_B {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
}

fn main() {
    // Initialize matrices A and B
    let a = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let b = vec![vec![7, 8, 9], vec![10, 11, 12]];

    // Initialize matrix C with zeros
    let c = vec![vec![0; COLS_B]; ROWS_A];

    // Create a vector to hold the thread handles
    let mut handles = vec![];

    // Use Arc and Mutex to safely share the matrices between threads
    let a_shared = Arc::new(a);
    let b_shared = Arc::new(b);
    let c_shared = Arc::new(Mutex::new(c));

    // Spawn a thread for each row of matrix A
    let rows_per_thread = ROWS_A / num_cpus::get();
    let mut start_row = 0;
    let mut end_row = 0;
    for i in 0..num_cpus::get() {
        let thread_start_row = start_row;
        let thread_end_row = if i == num_cpus::get() - 1 {
            ROWS_A
        } else {
            end_row + rows_per_thread
        };
        let a_shared = a_shared.clone();
        let b_shared = b_shared.clone();
        let c_shared = c_shared.clone();
        let handle = thread::spawn(move || {
            let mut c_local = c_shared.lock().unwrap();
            multiply_rows(&a_shared, &b_shared, &mut c_local, thread_start_row, thread_end_row);
        });
        handles.push(handle);
        start_row = end_row;
        end_row = thread_end_row;
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Print matrix C
    let c_final = c_shared.lock().unwrap();
    for i in 0..ROWS_A {
        for j in 0..COLS_B {
            print!("{} ", c_final[i][j]);
        }
        println!("");
    }
}
