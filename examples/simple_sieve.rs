//! # Sundaram Sieve 64-bit Basic Usage Example
//!
//! A simple programmatic example demonstrating how to use the 'sieve_sundaram_64' 
//! library to find prime numbers within a specified numerical interval.
//!
//! This example showcases how to call the core 'sundaram_sieve' function, handle 
//! its Result type, measure execution time, and inspect the resulting prime 
//! numbers collection directly within Rust code.

use sieve_sundaram_64::sundaram_sieve_seq;

fn main() {
    let limit_down: u64 = 10;
    let limit_up: u64 = 100;

    let start_time = std::time::Instant::now();

    let primes = match sundaram_sieve_seq(limit_down, limit_up) {
        Ok(primes_vec) => primes_vec,
        Err(err_msg) => {
            println!("ERROR : {}", err_msg);
            std::process::exit(1);
        }
    };

    let elapsed = start_time.elapsed();

    println!(
        "Interval [ {} , {} ] ; Count: {}",
        limit_down,
        limit_up,
        primes.len()
    );
    println!("Primes: {:?}", primes);
    println!("Elapsed Time: {:?}", elapsed);
}
