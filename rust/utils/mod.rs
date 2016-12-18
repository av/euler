use std::env;

pub mod primes;

/// Reads first argument from the users input
/// panic! s if anything looks wrong
pub fn read_arg() -> i64 {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            panic!("Empty args, terminating.");
        },
        2 => {
            match args[1].parse() {
                Ok(n) => n,
                Err(_) => {
                    panic!("Failed to parse num: {}", args[1]);
                }
            }
        },
        _ => {
            panic!("Only one arg allowed, terminating.")
        }
    }
}
