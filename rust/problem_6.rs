mod utils;

fn main() {
    println!("{}", calculate_difference(
        utils::read_arg() as u64
    ));
}

/// Calculates difference between sum of squares and square
/// of sums of number up to giben bound
///
/// 3 -> (1 + 2 + 3)^2 - (1^2 + 2^2 + 3^2) -> 22
fn calculate_difference(upper: u64) -> u64 {
    // Hello, inclusive ranges!
    // TODO: fix to ... when feature is standard
    let upper_bound = upper + 1;

    let sum_of_squares = (1..upper_bound)
        .map(|x| x * x)
        .fold(0, |total, next| total + next);

    let square_of_sum = (1..upper_bound)
            .fold(0, |total, next| total + next)
            .pow(2);

    square_of_sum - sum_of_squares
}