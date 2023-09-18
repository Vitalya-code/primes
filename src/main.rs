use rayon::prelude::*;
use std::io::{self};
use std::time::Instant;

fn main() -> io::Result<()> {
    // here you can set how many numbers you want to check
    const LIMIT: u32 = 100_000;

    // start the timer
    let now = Instant::now();
    let primes = simple_parralel(LIMIT);
    // we are only measuring speed of the algorithm
    let elapsed = now.elapsed();

    println!("{:?}", primes);
    println!("\nNumber of primes found: {}", primes.len());
    println!("  Last generated prime: {}", primes[primes.len() - 1]);
    println!("          Elapsed time: {:?}", elapsed);

    Ok(())
}

fn simple_parralel(limit: u32) -> Vec<u32> {
    // vectors with capacity are faster than dynamic
    let mut numbers: Vec<u32> = Vec::with_capacity(limit as usize);

    // fill our vector with numbers from 2 to the limit
    for num in 2..=limit {
        numbers.push(num)
    }

    // we are cloning the numbers array two "prime_numbers" with deleting non prime numbers
    // in parallel
    let prime_numbers: Vec<u32> = numbers
        .par_iter()
        .cloned()
        .filter(|&num| is_prime(num))
        .collect();

    // return prime_numbers;
    prime_numbers
}

// this function checks if a number is prime
fn is_prime(num: u32) -> bool {
    if num <= 1 {
        return false;
    }
    if num == 2 {
        return true;
    }
    if num % 2 == 0 {
        return false;
    }

    let mut divisor = 3;
    while divisor * divisor <= num {
        if num % divisor == 0 {
            return false;
        }
        divisor += 2;
    }

    true
}
