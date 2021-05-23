
pub fn insertion<T>(input: &mut Vec<T>)
where
    T: Ord,
{
    for i in 0..input.len() {
        for j in (0..i).rev() {
            if input[i] <= input[j] {}
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

pub fn bubble<T>(input: &mut Vec<T>) {}

pub fn merge<T>(input: &mut Vec<T>) {}

#[cfg(test)]
mod tests{
    use super::*;
    use assert::equal;
    
    static input_vec: Vec<i32> = vec![89, 3, 5, 34, 8, 1, 13, 21, 55, 2, 1];
    static output_vec: Vec<i32> = vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89];

    #[test]
    fn test_selection(){
        let mut input: Vec<i32> = input_vec.clone();
        selection(&mut input);
        assert::equal(input, output_vec.clone());
    }
}
