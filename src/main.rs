//! # Sundaram Sieve 64-bit Command-Line Interface
//!
//! A command-line utility for finding prime numbers within a given interval,
//! using the Interval Generalized Sundaram Sieve for 64-bit integers.
//!
//! This executable allows users to compute primes via terminal arguments and  
//! automatically save the resulting sequence to a text file.

use sieve_sundaram_64::sundaram_sieve_par;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let (limit_down, limit_up) = if args.len() == 3 {
        let down = args[1].parse::<u64>().expect("Invalid lower limit !");
        let up = args[2].parse::<u64>().expect("Invalid upper limit !");

        (down, up)
    } else {
        (5, 5)
    };

    println!("Sieving the integers !");
    let start_time = std::time::Instant::now();
    let primes = match sundaram_sieve_par(limit_down, limit_up) {
        Ok(p) => p,
        Err(e) => {
            println!("Error: {}", e);
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
    println!("Elapsed Time: {:?}", elapsed);

    let filename = format!("primes_from_{}_to_{}.txt", limit_down, limit_up);
    println!("Saving to {} !", filename);

    let save_result = (|| -> std::io::Result<()> {
        use std::io::Write;
        let file = std::fs::File::create(&filename)?;
        let mut writer = std::io::BufWriter::new(file);

        for prime in &primes {
            writeln!(writer, "{}", prime)?;
        }

        writer.flush()?;
        Ok(())
    })();

    match save_result {
        Ok(()) => println!("File saved successfully !"),
        Err(e) => println!("Failed to save file ! : {}", e),
    }
}
