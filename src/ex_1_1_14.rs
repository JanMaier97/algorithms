//! # Exercise 1.1.14, p. 56
//!
//! Write a static method lg() that takes an `int` value N as argument and returns the largest
//! `int` not larger than the base-2 logarithm of N. Do *not* use math.
//!
//! ## Example
//!
//! ```
//! use algorithms_lib::ex_1_1_14::*;
//!
//! assert_eq!(lg(100), 6);
//! assert_eq!(lg(6), 2);
//! ```

pub fn lg(n: u32) -> u32 {
    if n == 0 {
        panic!("Cannot compute the logarith of 0");
    }

    // java's pow mehtod is part of the Math library
    // so I cannot use it in rust as well
    let mut current_val = 1;
    for i in 1..n {
        current_val *= 2;

        if current_val > n {
            return i - 1;
        }
    }

    unreachable!("The for loop completed for n={}", n);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn panic_for_0() {
        lg(0);
    }
}
