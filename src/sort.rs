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

pub fn merge<T>(input: &mut Vec<T>)
where
    T: Ord,
{
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
}
