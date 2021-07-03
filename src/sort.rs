use std::fmt::*;
use std::result::Result;

#[allow(dead_code)]
pub fn insertion<T>(input: &mut Vec<T>)
where
    T: Ord,
{
    for i in 0..input.len() {
        for j in (0..i).rev() {
            if input[j] >= input[j + 1] {
                input.swap(j, j + 1)
            } else {
                break;
            }
        }
    }
}

#[allow(dead_code)]
pub fn selection<T>(input: &mut Vec<T>)
where
    T: Ord + Copy,
{
    let mut output: Vec<T> = Vec::new();

    while input.len() > 0 {
        let mut min_index: usize = 0;
        let mut min_value: T = input[min_index];
        for i in 1..input.len() {
            if input[i] < min_value {
                min_value = input[i];
                min_index = i;
            }
        }

        output.push(min_value);
        input.remove(min_index);
    }

    input.append(&mut output);
}

#[allow(dead_code)]
pub fn bubble<T>(input: &mut Vec<T>)
where
    T: Ord,
{
    if input.len() < 2 {
        return;
    }

    loop {
        let mut swap_count: usize = 0;
        for i in 1..input.len() {
            if input[i] < input[i - 1] {
                input.swap(i, i - 1);
                swap_count = swap_count + 1;
            }
        }

        if swap_count == 0 {
            break;
        }
    }
}

#[allow(dead_code)]
pub fn merge<T>(input: &mut Vec<T>)
where
    T: Ord + Copy + Debug,
{
    fn merge_sort<T>(input1: Vec<T>, input2: Vec<T>) -> Vec<T>
    where
        T: Ord + Copy,
    {
        if input1.is_empty() {
            return input2;
        }

        if input2.is_empty() {
            return input1;
        }

        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut output: Vec<T> = Vec::new();
        while i + j < input1.len() + input2.len() {
            if i < input1.len() && (j == input2.len() || input1[i] < input2[j]) {
                output.push(input1[i]);
                i = i + 1;
            } else if j < input2.len() {
                output.push(input2[j]);
                j = j + 1;
            }
        }

        return output;
    }

    if input.len() < 2 {
        return;
    }

    let mut half1 = (&input[0..input.len() / 2]).to_vec();
    let mut half2 = (&input[input.len() / 2..input.len()]).to_vec();
    merge(&mut half1);
    merge(&mut half2);
    let mut output = merge_sort(half1, half2);

    input.clear();
    input.append(&mut output);
}

// Identify the second largest number in the array
// with at most n + log(n) - 2 comparisons
#[allow(dead_code)]
pub fn second_largest<T>(input: &Vec<T>) -> T
where
    T: Ord + Copy + Debug,
{
    fn merge_second_largest<T>(input1: Vec<T>, input2: Vec<T>) -> T
    where
        T: Ord + Copy,
    {
        if input1.is_empty() {
            assert!(input2.len() == 1);
            return input2[0];
        }

        if input2.is_empty() {
            assert!(input1.len() == 1);
            return input1[0];
        }

        let max_1 = input1.iter().max();
        let max_2 = input2.iter().max();
        if max_1 > max_2 {
            return *max_2.unwrap();
        } else {
            return *max_1.unwrap();
        };
    }

    assert!(!input.is_empty());

    if input.len() < 2 {
        return input[0];
    }

    let half1 = (&input[0..input.len() / 2]).to_vec();
    let half2 = (&input[input.len() / 2..input.len()]).to_vec();
    second_largest(&half1);
    second_largest(&half2);
    return merge_second_largest(half1, half2);
}

#[allow(dead_code)]
pub fn count_inversions<T>(input: &mut Vec<T>) -> usize
where
    T: Ord + Copy + Debug,
{
    pub fn merge_count_inv<T>(input1: Vec<T>, input2: Vec<T>) -> (usize, Vec<T>)
    where
        T: Ord + Copy,
    {
        if input1.is_empty() {
            return (0, input2);
        }

        if input2.is_empty() {
            return (0, input1);
        }

        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut inversions: usize = 0;
        let mut output: Vec<T> = Vec::new();
        while i + j < input1.len() + input2.len() {
            if i < input1.len() && (j == input2.len() || input1[i] < input2[j]) {
                output.push(input1[i]);
                i = i + 1;
            } else if j < input2.len() {
                output.push(input2[j]);
                j = j + 1;
                inversions = inversions + input1.len() - i;
            }
        }

        return (inversions, output);
    }

    if input.len() < 2 {
        return 0;
    }

    let mut half1 = (&input[0..input.len() / 2]).to_vec();
    let mut half2 = (&input[input.len() / 2..input.len()]).to_vec();
    let half1_inversions = count_inversions(&mut half1);
    let half2_inversions = count_inversions(&mut half2);
    let (inversions, mut output) = merge_count_inv(half1, half2);

    input.clear();
    input.append(&mut output);
    return inversions + half1_inversions + half2_inversions;
}

#[allow(dead_code)]
pub fn closest_pair_1d<T>(input: &mut Vec<T>) -> (T, T)
where
    T: Ord + Copy + Debug + num_traits::Num,
{
    assert!(input.len() > 1);

    if input.len() == 2 {
        return (input[0], input[1]);
    }

    let mut input_clone = input.clone();
    merge(&mut input_clone);

    input.clear();
    input.append(&mut input_clone);

    let mut index: usize = 0;
    let mut diff: Option<T> = None;
    for i in 1..input.len() {
        if !diff.is_none() && input[i] - input[i - 1] >= diff.unwrap() {
            continue;
        }

        diff = Some(input[i] - input[i - 1]);
        index = i;
    }
    return (input[index], input[index - 1]);
}

#[allow(dead_code)]
pub fn abs<T>(val: T) -> Result<T, String>
where
    T: Ord
        + num_traits::Num
        + num_traits::Signed
        + num_traits::Bounded
        + num_traits::Zero
        + Copy
        + Debug,
{
    match val {
        val if val == T::min_value() => Err("Overflow".to_string()),
        val if val < T::zero() => Ok(-val),
        _ => Ok(val),
    }
}

#[allow(dead_code)]
pub fn closest_pair_1d_v2<T>(input: &mut Vec<T>) -> Option<(T, T)>
where
    T: Ord + Copy + Debug + num_traits::Num + num_traits::Bounded,
{
    fn merge_find_pair<T>(input1: Vec<T>, input2: Vec<T>, diff: T) -> (Option<(T, T)>, Vec<T>)
    where
        T: Ord + Copy + num_traits::Num,
    {
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut diff_copy = diff;
        let mut pair: Option<(T, T)> = None;
        let mut output: Vec<T> = Vec::new();

        while i + j < input1.len() + input2.len() {
            if i < input1.len() && (j == input2.len() || input1[i] < input2[j]) {
                output.push(input1[i]);
                i = i + 1;
            } else if j < input2.len() {
                output.push(input2[j]);
                j = j + 1;

                if i + j > 1 && diff_copy > output[i + j - 1] - output[i + j - 2] {
                    diff_copy = output[i + j - 1] - output[i + j - 2];
                    pair = Some((output[i + j - 2], output[i + j - 1]));
                }
            }
        }

        return (pair, output);
    }

    let mut half1 = (&input[0..input.len() / 2]).to_vec();
    let mut half2 = (&input[input.len() / 2..input.len()]).to_vec();

    if input.len() <= 1 {
        return None;
    }

    let pair1 = closest_pair_1d_v2(&mut half1);
    let pair2 = closest_pair_1d_v2(&mut half2);
    let diff1 = if pair1.is_none() {
        None
    } else {
        Some(pair1.unwrap().1 - pair1.unwrap().0)
    };
    let diff2 = if pair2.is_none() {
        None
    } else {
        Some(pair2.unwrap().1 - pair2.unwrap().0)
    };
    let diff = if diff1.is_none() || diff2.is_none() {
        T::max_value()
    } else {
        std::cmp::min(diff1.unwrap(), diff2.unwrap())
    };

    let (pair, mut output) = merge_find_pair(half1, half2, diff);

    let mut temp_pair = if diff1.is_none() && diff2.is_none() {
        None
    } else if diff2.is_none() {
        pair1
    } else {
        pair2
    };

    if !pair.is_none() {
        let split_diff = pair.unwrap().1 - pair.unwrap().0;
        temp_pair = if split_diff < diff { pair } else { temp_pair };
    }

    input.clear();
    input.append(&mut output);
    return temp_pair;
}

// TODO: TBC
#[allow(dead_code)]
pub fn closest_pair_2d<T>(input: Vec<(T, T)>) -> Option<((T, T), (T, T))>
where
    T: Ord + Copy + Debug + num_traits::Num + num_traits::Bounded,
{
    return Some((
        (T::max_value(), T::max_value()),
        (T::max_value(), T::max_value()),
    ));
}

#[allow(dead_code)]
pub fn quick_first<T>(input: &mut Vec<T>) -> usize
where
    T: Ord + Debug,
{
    // bootstrap
    // choose partition
    // sort
}

#[allow(dead_code)]
pub fn quick_last<T>(input: &mut Vec<T>)
where
    T: Ord + Debug,
{
    // bootstrap
    // choose partition
    // sort
}

#[allow(dead_code)]
pub fn quick_median_first_middle_last<T>(input: &mut Vec<T>)
where
    T: Ord + Debug,
{
    // bootstrap
    // choose partition
    // sort
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref INPUT: Vec<i32> = vec![89, 3, 5, 34, 8, 1, 13, 21, 55, 2, 1];
        static ref OUTPUT: Vec<i32> = vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89];
    }

    #[test]
    fn test_selection() {
        let mut input: Vec<i32> = INPUT.clone();
        selection(&mut input);
        assert::equal(input, OUTPUT.clone());
    }

    #[test]
    fn test_insertion() {
        let mut input: Vec<i32> = INPUT.clone();
        insertion(&mut input);
        assert::equal(input, OUTPUT.clone());
    }

    #[test]
    fn test_bubble() {
        let mut input: Vec<i32> = INPUT.clone();
        bubble(&mut input);
        assert::equal(input, OUTPUT.clone());
    }

    #[test]
    fn test_merge() {
        let mut input: Vec<i32> = INPUT.clone();
        merge(&mut input);
        assert::equal(input, OUTPUT.clone());
    }

    #[test]
    fn test_second_largest() {
        let input: Vec<i32> = INPUT.clone();
        let output = second_largest(&input);
        assert::equal(output, OUTPUT[OUTPUT.len() - 2]);
    }

    #[test]
    fn test_count_inversions() {
        let mut input: Vec<i32> = vec![6, 5, 4, 3, 2, 1];
        let target: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
        let output = count_inversions(&mut input);
        assert::equal(output, 15);
        assert::equal(target, input);
    }

    #[test]
    fn test_closest_pair_1d() {
        let mut input: Vec<i32> = INPUT.clone();
        let pair = closest_pair_1d(&mut input);
        assert::equal(pair.0, OUTPUT[0]);
        assert::equal(pair.1, OUTPUT[1]);
        assert::equal(input, OUTPUT.clone())
    }

    #[test]
    fn test_closest_pair_1d_v2() {
        let mut input: Vec<i32> = INPUT.clone();
        let pair = closest_pair_1d_v2(&mut input);
        assert::equal(input, OUTPUT.clone());
        assert!(!pair.is_none());
        assert::equal(pair.unwrap().0, OUTPUT[0]);
        assert::equal(pair.unwrap().1, OUTPUT[1]);
    }

    /* #[test]
    fn test_closest_pair_2d() {
        let input: Vec<(i32, i32)> = vec![(-2, 4), (3, 2), (5, 6), (0, 9), (6, -4)];
        let pair = closest_pair_2d(input);
        assert!(!pair.is_none());
        assert::equal(pair.unwrap().0, (3, 2));
        assert::equal(pair.unwrap().1, (5, 6));
    } */
}
