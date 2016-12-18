mod utils;

fn main() {
    println!("{}", find_nth_prime(
        utils::read_arg() as u64
    ));
}

/// Finds nth prime using the Prime struct
/// implementing the std::iterator::Iterator trait
/// 6     -> 13
/// 10001 -> 104743
fn find_nth_prime(upper: u64) -> u64 {
    let mut primes = utils::primes::Prime::new();
    let mut n = 0;

    while { n += 1; n < upper }  {
        primes.next();
    }

    primes.next().unwrap()
}