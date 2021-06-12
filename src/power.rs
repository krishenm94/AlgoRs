pub fn fast(base: usize, power: usize) -> usize {
    if power == 0 {
        return 1;
    }
    if power == 1 {
        return base;
    }
    let val = fast(base * base, power / 2);
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
        let output = fast(base, power as usize);
        assert::equal(output, base.pow(power));
    }
}
