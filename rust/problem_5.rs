mod utils;

fn main() {
    println!("{}", find_smallest_multiple(
        utils::read_arg()
    ))
}

/// Finds least multiple of numbers up to given one
///
/// 1                -> 1
/// 2  -> 1, 2       -> 2
/// 3  -> 1, 2, 3    -> 6
/// 4  -> 1, 2, 3, 4 -> 12
/// 10 -> 1   ..  10 -> 2520
fn find_smallest_multiple(upper: i64) -> u64 {
    let mut result = 1;
    let primes = utils::primes::generate_primes(upper as u64);

    for i in primes.iter() {
        let float_i = *i as f64;
        let a = ((upper as f64).ln() / float_i.ln()).floor() as u32;

        result = result * i.pow(a);
    }

    result
}

