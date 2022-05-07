//! # Exercise 1.1.11, p.56
//!
//! Write a code fragment that prints the contents of a two-dimensional boolean array, using * to
//! represent `true` and a space to represent `false`. Include row and column numbers.
//!
//! ## Example
//! ```
//! use algorithms_lib::ex_1_1_11::*;
//!
//! let matrix = vec![vec![true, false, false], vec![false, true, true], vec![false, false,
//! false]];
//!
//! // creating formatted strings is weird
//! let output = "0 *  \n\
//!               1  * \n\
//!               2  * 
//!   012";
//!
//! assert_eq!(matrix_to_string(&matrix), output);
//! ```

pub fn matrix_to_string(matrix: &Vec<Vec<bool>>) -> String {
    if matrix.len() == 0 {
        return String::from("");
    }

    if matrix[0].len() == 0 {
        return String::from("");
    }

    let row_count = matrix[0].len();
    let mut result = String::new();
    let convert_value = |val| match val {
        true => String::from("*"),
        false => String::from(" ")
    };


    for row_num in 0..row_count {
        let line = format!(
            "{} {}\n",
            row_num,
            matrix
                .iter()
                .map(|col| convert_value(col[row_num]))
                .collect::<Vec<String>>()
                .join("")
        );

        result.push_str(&line);
    }

    let column_signs = (0..matrix.len())
        .map(|num| num.to_string())
        .collect::<Vec<String>>()
        .join("");

    result.push_str(&format!("  {}", column_signs));

    return result;
}

#[cfg(test)]
mod tests {}
