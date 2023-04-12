// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand for a hint.

use core::ops::Range;

fn main() {
    // Create an array of length 100
    let hundred_zeros: [u8; 100] = [0; 100];
    let zero_to_hundred: Range<i32> = 0..100;

    if hundred_zeros.len() >= 100 && zero_to_hundred.len() >= 100 {
        println!("{:?}", hundred_zeros);
        println!("{:?}", zero_to_hundred);
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
