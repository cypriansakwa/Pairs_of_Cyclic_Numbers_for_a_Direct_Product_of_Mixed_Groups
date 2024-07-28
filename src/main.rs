use num::integer::gcd;

// Function to check if a number is prime
fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// Function to compute Euler's totient function
fn euler_totient(n: u64) -> u64 {
    let mut result = n;
    let mut p = 2;
    let mut num = n;
    while p * p <= num {
        if num % p == 0 {
            while num % p == 0 {
                num /= p;
            }
            result -= result / p;
        }
        p += 1;
    }
    if num > 1 {
        result -= result / num;
    }
    result
}

// Function to generate a vector of primes less than or equal to a given maximum
fn generate_primes(max: u64) -> Vec<u64> {
    let mut primes = Vec::new();
    for num in 2..=max {
        if is_prime(num) {
            primes.push(num);
        }
    }
    primes
}

// Function to check if (n, m) pair satisfies the condition
fn is_valid_pair(n: u64, m: u64) -> bool {
    let phi_m = euler_totient(m);
    gcd(n, phi_m) == 1
}

fn main() {
    // Define range for n
    let n_range = (1, 10);
    // Define maximum value for m
    let max_m = 10;
    
    // Generate all primes up to max_m
    let primes = generate_primes(max_m);
    
    // Iterate through all possible (n, m) pairs and print the valid ones
    for n in n_range.0..=n_range.1 {
        for &m in &primes {
            if is_valid_pair(n, m) {
                println!("Generated pair: (n, m) = ({}, {})", n, m);
            }
        }
    }
}









