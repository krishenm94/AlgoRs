pub fn get_magnitude(x: i64) -> u32 {
    let mut mag: u32 = 0;
    let mut val = x;
    while val != 0 {
        val = val / 10;
        mag = mag + 1;
    }

    return mag;
}

pub fn mult(x: i64, y: i64) -> i64 {
    if x == 0 || y == 0 {
        return 0;
    }
    let mag_x = get_magnitude(x);
    let mag_y = get_magnitude(y);
    println!("Value & Mag x, y: {}, {}, {}, {}", x, mag_x, y, mag_y);
    if mag_x < 2 && mag_y < 2 {
        return x * y;
    }

    let a = x / (i64::pow(10, mag_x / 2));
    let b = x - (a * (i64::pow(10, mag_x / 2)));
    let c = y / (i64::pow(10, mag_y / 2));
    let d = y - (c * (i64::pow(10, mag_y / 2)));

    println!("{}", (10 ^ (mag_y / 2)));
    println!("a, b, c, d: {}, {}, {}, {}", a, b, c, d);

    let p = a + b;
    let q = c + d;
    let ac = mult(a, c);
    let bd = mult(b, d);
    let pq = mult(p, q);
    let adbc = pq - ac - bd;

    // TBC
    // shortcut: mag_x is used
    return (i64::pow(10, mag_x)) * ac + (i64::pow(10, mag_x / 2)) * adbc + bd;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_magnitude() {
        let x = 1234567890;
        let mag = get_magnitude(x);
        assert::equal(mag, 10);
    }

    #[test]
    fn test_mult() {
        let x: i64 = 12;
        let y: i64 = 13;
        let output = mult(x, y);
        assert::equal(output, 156);
    }
}
