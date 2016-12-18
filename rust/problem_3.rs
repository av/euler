mod utils;

/// Reads args from console, calculates the sum for parsed num.
fn main () {
    print!("{}", get_largest_prime_factor(utils::read_arg() as u64))
}

/// Returns largest prime factor of given numer
fn get_largest_prime_factor(upper: u64) -> u64 {
    let upper_bound = (upper as f64).sqrt() as u64 + 1;
    let mut i = 1;
    let mut max = 0;

    while i < upper_bound {
        if is_dividable_by(upper, i) && utils::primes::is_prime(i) {
            max = i;
        }

        i += 1;
    }

    max
}

/// Returns true if given `n` is divisable by `div`
fn is_dividable_by(n: u64, div: u64) -> bool {
    (n % div) == 0
}