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
