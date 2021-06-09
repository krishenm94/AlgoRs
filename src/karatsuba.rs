// TBC
fn mult(x: i64, y: i64, n: i64) -> i64 {
    if n == 1 {
        return x * y;
    }

    let a = x / (10 ^ (n / 2));
    let b = x - a;
    let c = y / (10 ^ (n / 2));
    let d = y - c;

    let p = a + b;
    let q = c + d;
    let ac = mult(a, c, n / 2);
    let bd = mult(b, d, n / 2);
    let pq = mult(p, q, n / 2);
    let adbc = pq - ac - bd;

    return 10 ^ n * ac + 10 ^ (n - 2) * adbc + bd;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mult() {
        let x: i64 = 12;
        let y: i64 = 13;
        let n = 4;
        let output = mult(x, y, n);
        assert::equal(output, 156);
    }
}
