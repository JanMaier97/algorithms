//! # Exercise 1.1.15, p. 56
//!
//! Write a static method `histogram()` that takes an array `a[]` of `int` values and an integer
//! `M` as arguments and returns an array of length `M` wiose `i`th entry is the number of times
//! the integer `i` appeared in the argument array. If the values in a[] are all between 0 and
//! `M-1`, the sum of values in the returned array should be equal to `a.length`.
//!
//! # Example
//! ```
//! use algorithms_lib::ex_1_1_15::*;
//!
//! assert!(
//!     are_equal(
//!         &histogram(&[1,1,0,2,4,5,3,4,5,5,3,4,5,5,3,2,0,10]),
//!         &[2,2,2,3,3,5,0,0,0,0,1]
//!     )
//! );
//! ```

// Use &[T] instead of Veck<T>
// &[T] is a slice type, which is automatically coerced
// &[T] also allows and arrays without allocating them on the stack
// Vec is an mutable list, but we will not mutate it anyways, so &[T] is fine
pub fn histogram(values: &[u32]) -> Vec<u32> {
    let iter = dbg!(values).iter();
    let largest_number = match iter.max() {
        Some(num) => num,
        None => return vec![],
    };

    let largest_number = *largest_number as usize;
    let mut histogram = vec![0; largest_number + 1];

    for value in values {
        let value = *value as usize;
        histogram[value] += 1;
    }

    dbg!(histogram)
}

pub fn are_equal(a: &[u32], b: &[u32]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    a.iter()
        .zip(b.iter())
        .all(|(item_a, item_b)| item_a == item_b)
}

#[cfg(test)]
mod tests {
    use super::are_equal;

    #[test]
    fn arrays_of_different_lengths() {
        assert!(!are_equal(&[0, 1, 2], &[0, 1]));
    }

    #[test]
    fn equal_arrays() {
        assert!(are_equal(&[0, 1, 2], &[0, 1, 2]));
    }

    #[test]
    fn arrays_with_different_items() {
        assert!(!are_equal(&[0, 1, 2], &[1, 2, 3]));
    }
}
