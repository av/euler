mod utils;

/// Calculates the sum of numbers divisible by 3 or 5 up to
/// bound entered via console
fn main () {
    println!("{}", find_divisables_sum(utils::read_arg()));
}

/// Finds the sum of all numbers less then `upper`
fn find_divisables_sum(upper: i64) -> i64 {
   sum_divisible_by(upper, 3) + sum_divisible_by(upper, 5) - sum_divisible_by(upper, 15)
}

/// Finds the sum of all numbers less then given `upper`
/// which is dividable by `num`
fn sum_divisible_by(upper: i64, num: i64) -> i64 {
    let p = upper / num;

    num * (p * (p + 1)) / 2
}