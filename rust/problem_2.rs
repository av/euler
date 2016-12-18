mod utils;

/// Finds sum of all even fibonacci numbers
/// up to given number
fn main () {
    println!("{}", even_fibonacci_sum(utils::read_arg()));
}

/// Finds the sum of all even fibonacci numbers which a re
/// less then given upper limit.
fn even_fibonacci_sum(n: i64) -> i64 {
    let mut a = 1;
    let mut b = 1;
    let mut c = a + b;
    let mut sum = 0;

    while c < n {
        sum += c;
        a = b + c;
        b = c + a;
        c = a + b;
    }

    sum
}
