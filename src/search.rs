use nalgebra::base::*;
use std::{option::Option, vec::Vec};

// Find maximum of a unimodal vector;
// a vector with elements ascending till a peak and then decreasing.
#[allow(dead_code)]
pub fn unimodal_maximum<T>(input: &Vec<T>, start: usize, end: usize) -> T
where
    T: Ord + Copy,
{
    let index = (end + start) / 2;
    if end == start {
        input[start]
    } else if input[index + 1] > input[index] {
        unimodal_maximum(input, index + 1, end)
    } else {
        unimodal_maximum(input, start, index)
    }
}

#[allow(dead_code)]
pub fn element_equals_index(input: &Vec<i32>, start: usize, end: usize) -> Option<usize> {
    let index: usize = (end + start) / 2;
    if input[index] == index as i32 {
        Some(index)
    } else if end == start {
        None
    } else if input[index] < index as i32 {
        element_equals_index(input, index + 1, end)
    } else {
        element_equals_index(input, start, index)
    }
}

// TODO: test
#[allow(dead_code)]
pub fn local_min_2d_grid(
    input: &Matrix2<i32>,
    depth: usize,
    candidate: (i32, Option<(usize, usize)>),
) -> (usize, usize) {
    fn is_local_min(input: &Matrix2<i32>, col: usize, _row: usize) -> bool {
        let start_row = std::cmp::max(0, _row as i32 - 1) as usize;
        let end_row = std::cmp::min(_row + 1, input.nrows() - 1);
        let start_col = std::cmp::max(0, col as i32 - 1) as usize;
        let end_col = std::cmp::min(col + 1, input.ncols() - 1);

        for i in start_col..end_col {
            for j in start_row..end_row {
                if *input.get(i + j * input.ncols()).unwrap()
                    < *input.get(col + _row * input.ncols()).unwrap()
                {
                    return false;
                }
            }
        }

        return true;
    }

    fn check_col(
        input: &Matrix2<i32>,
        col: usize,
        start_row: usize,
        end_row: usize,
        candidate: i32,
    ) -> (Option<(usize, usize)>, Option<(usize, usize)>) {
        let mut current_candidate_pos: Option<(usize, usize)> = None;
        for j in start_row..end_row {
            let success = is_local_min(input, col, j);

            if success {
                return (Some((col, j)), None);
            }

            if *input.get(col + j * input.ncols()).unwrap() < candidate {
                current_candidate_pos = Some((col, j));
            }
        }

        (None, current_candidate_pos)
    }

    fn check_row(
        input: &Matrix2<i32>,
        row: usize,
        start_col: usize,
        end_col: usize,
        candidate: i32,
    ) -> (Option<(usize, usize)>, Option<(usize, usize)>) {
        let mut current_candidate_pos: Option<(usize, usize)> = None;
        for i in start_col..end_col {
            let success = is_local_min(input, i, row);

            if success {
                return (Some((row, i)), None);
            }

            if *input.get(i + row * input.ncols()).unwrap() < candidate {
                current_candidate_pos = Some((row, i));
            }
        }

        (None, current_candidate_pos)
    }

    fn get_quadrant(
        input: &Matrix2<i32>,
        depth: usize,
        candidate: (i32, Option<(usize, usize)>),
    ) -> ((usize, usize), (usize, usize)) {
        if candidate.1.is_none() {
            return ((0, input.ncols() - 1), (0, input.nrows() - 1));
        } else {
            let row_offset = input.nrows() / depth + 1 / 2;
            let col_offset = input.ncols() / depth + 1 / 2;
            let col = candidate.1.unwrap().0;
            let row = candidate.1.unwrap().1;
            let start = (
                std::cmp::max(0, col - col_offset),
                std::cmp::max(0, row - row_offset),
            );
            let end = (
                std::cmp::min(input.ncols(), col + col_offset),
                std::cmp::min(input.nrows(), row + row_offset),
            );
            return (start, end);
        }
    }

    let (start, end) = get_quadrant(&input, depth, candidate);

    let mut candidate_copy = candidate;
    let col_output = check_col(&input, (start.0 + end.0) / 2, start.1, end.1, candidate.0);
    if !col_output.0.is_none() {
        return col_output.0.unwrap();
    }
    if !col_output.1.is_none() {
        let candidate_value = input
            .get(col_output.0.unwrap().0 + col_output.0.unwrap().1 * input.ncols())
            .unwrap();
        candidate_copy = (*candidate_value, col_output.1);
    }
    let row_output = check_row(&input, (start.1 + end.1) / 2, start.0, end.1, candidate.0);
    if !row_output.0.is_none() {
        return row_output.0.unwrap();
    }
    if !row_output.1.is_none() {
        let candidate_value = input
            .get(row_output.0.unwrap().0 + row_output.0.unwrap().1 * input.ncols())
            .unwrap();
        candidate_copy = (*candidate_value, row_output.1);
    }

    local_min_2d_grid(&input, depth + 1, candidate_copy)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unimodal_maximum() {
        let input: Vec<i32> = vec![1, 2, 3, 4, 5, 3, 1, -1];
        let target = 5;
        let output = unimodal_maximum(&input, 0, input.len() - 1);
        assert::equal(output, target)
    }
    #[test]
    fn test_element_equals_index() {
        let input = vec![-12, 0, 0, 3, 46, 57, 73, 101];
        let target: Option<usize> = Some(3);
        let output = element_equals_index(&input, 0, input.len() - 1);
        assert::equal(output, target)
    }
}
