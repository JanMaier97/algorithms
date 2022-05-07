//! # Excercise 1.1.13
//! write a code fragment to print the transposition (rows and columns changed) of a
//! two-dimensional array with M rows and N columns
//!
//! ## Example
//!
//! ```
//! use algorithms_lib::ex_1_1_13::*;
//! let result = transpose(&vec![vec![true, false, false]]);
//! let expected = vec![vec![true], vec![false], vec![false]];
//!
//! assert!(matrix_eq(&result, &expected));
//! ```

pub fn transpose(matrix: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    if matrix.len() == 0 {
        return Vec::new();
    }

    if matrix.len() > 1 {
        let column_len = matrix[0].len();

        for column in matrix {
            if column.len() != column_len {
                panic!("Columns have different lenghts");
            }
        }
    }

    let trans_column_count = matrix[0].len();
    let trans_row_count = matrix.len();
    let mut transpose: Vec<Vec<bool>> = vec![vec![false; trans_row_count]; trans_column_count];

    for (x, column) in matrix.iter().enumerate() {
        for (y, cell) in column.iter().enumerate() {
            transpose[y][x] = cell.clone();
        }
    }

    transpose
}

pub fn matrix_eq(mat_a: &Vec<Vec<bool>>, mat_b: &Vec<Vec<bool>>) -> bool {
    if mat_a.len() != mat_b.len() {
        return false;
    }

    for (col_a, col_b) in mat_a.iter().zip(mat_b) {
        if col_a.len() != col_b.len() {
            return false;
        }

        for (cell_a, cell_b) in col_a.iter().zip(col_b) {
            if cell_a != cell_b {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_eq_emtpy_matrix() {
        assert!(matrix_eq(&(Vec::new()), &Vec::new()));
    }

    #[test]
    fn empty_matrix() {
        let result = transpose(&Vec::new());

        assert!(matrix_eq(&result, &Vec::new()));
    }

    #[test]
    fn column_only_matrix() {
        let result = transpose(&vec![vec![true, false, false]]);
        let expected = vec![vec![true], vec![false], vec![false]];
            
        assert!(matrix_eq(&result, &expected));
    }

    #[test]
    fn row_only_matrix() {
        let result = vec![vec![true], vec![false], vec![false]];
        let expected = transpose(&vec![vec![true, false, false]]);
            
        assert!(matrix_eq(&result, &expected));
    }
}
