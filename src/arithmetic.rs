#[allow(dead_code)]
pub fn binary_exponentiation(base: usize, power: usize) -> usize {
    if power == 0 {
        return 1;
    }
    if power == 1 {
        return base;
    }
    let val = binary_exponentiation(base * base, power / 2);
    if power % 2 == 1 {
        return base * val;
    } else {
        return val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast() {
        let base: usize = 23;
        let power: u32 = 7;
        let output = binary_exponentiation(base, power as usize);
        assert::equal(output, base.pow(power));
    }
}
