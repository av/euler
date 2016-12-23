mod utils;

fn main() {
    println!("{}", find_pythagorean_triplet(
        utils::read_arg()
    ));
}

/// Finds the pythagorean triplet for a given target,
/// returning -1 if it doesn't exist and a product of a triplet
/// if it was found
fn find_pythagorean_triplet(target: i64) -> i64 {
    for i in 1..target {
        for j in 1..target {
            for k in 1..target {
                if is_pythagorean_triplet(i, j, k) && (i + j + k == target) {
                    return i * j * k;
                }
            }
        }
    }

    -1
}

/// Checks if given triple is pythagorean
fn is_pythagorean_triplet(i: i64, j: i64, k: i64) -> bool {
    let i2 = i.pow(2);
    let j2 = j.pow(2);
    let k2 = k.pow(2);

    i2 + j2 == k2 || i2 + k2 == j2 || j2 + k2 == i2
}