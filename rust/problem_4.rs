use std::ops::Range;

mod utils;

/// Finds a largest palindrome made from product of n-digit numbers
fn main() {
    println!("{}", find_largest_palindrome(
        to_range(utils::read_arg() as u8)
    ));
}

/// Finds the largest palindrome for any combination
/// of multipliers for a given range
///
/// find_largest_palindrome(10..99) -> 8448
fn find_largest_palindrome(range: Range<u32>) -> u64 {
    let mut largest: u64 = 0;

    for i in range.clone() {
        for j in range.clone().rev() {
            let mult: u64 = (i * j) as u64;

            if is_palindrome(mult) && mult > largest {
                largest = mult
            }
        }
    }

    largest
}

/// Returns true if given number is palindrome
///
/// is_palindrome(202) -> true
/// is_palindrome(201) -> false
fn is_palindrome(num: u64) -> bool {
    let str = num.to_string();

    str
        .chars()
        .zip(str.chars().rev())
        .all(|(x, y)| x == y)
}

/// Converts given magnitude to a range of real numbers
///
/// to_range(2) -> 10..99
/// to_range(3) -> 100..999
fn to_range(magnitude: u8) -> Range<u32> {
    10u32.pow((magnitude - 1) as u32)..(10u32.pow(magnitude as u32) - 1)
}



