//! Exercise 1.1.9, p. 55
//!
//! Write a code fragment that puts the binary representation of a positive integer N into a String s.
//! ```
//! use algorithms_lib::ex_1_1_9::*;
//! assert_eq!(integer_to_binary_string(13), "1101");
//! ```

pub fn integer_to_binary_string(mut number: i32) -> String {
    let mut result = String::new();

    while number > 0 {
        result = (number % 2).to_string() + &result;
        number = number / 2;
    }

    result
}
