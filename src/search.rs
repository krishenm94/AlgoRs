use std::vec::Vec;

// Find maximum of a unimodal vector;
// a vector with elements ascending till a peak and then decreasing.
pub fn unimodal_maximum<T>(input: &Vec<T>, start: usize, end: usize) -> T
where
    T: Ord + Copy,
{
    if end - start == 0 {
        return input[start];
    }

    if input[(end + start) / 2 + 1] > input[(end + start) / 2] {
        unimodal_maximum(input, (start + end) / 2 + 1, end)
    } else {
        unimodal_maximum(input, 0, (start + end) / 2)
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
}
