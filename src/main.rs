use std::io::{self};
use std::time::Instant;

fn sieve_of_eratosthenes(limit: u32) -> Vec<u32> {
    let mut is_prime = vec![true; (limit + 1) as usize];
    is_prime[0] = false;
    is_prime[1] = false;

    for num in 2..=limit {
        if is_prime[num as usize] {
            for multiple in (num * 2..=limit).step_by(num as usize) {
                is_prime[multiple as usize] = false;
            }
        }
    }

    let prime_numbers: Vec<u32> = (2..=limit).filter(|&num| is_prime[num as usize]).collect();

    prime_numbers
}

fn main() -> io::Result<()> {
    let now = Instant::now();
    const LIMIT: u32 = 100_000;

    let primes = sieve_of_eratosthenes(LIMIT);
    let elapsed = now.elapsed();

    println!("{:?}", primes);
    println!("\nNumber of primes found: {}", primes.len());
    println!("  Last generated prime: {}", primes[primes.len() - 1]);
    println!("          Elapsed time: {:?}", elapsed);

    Ok(())
}
