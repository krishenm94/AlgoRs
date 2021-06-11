/* use nalgebra::base::Matrix;

pub fn matrix_mult<T, const R: usize, const C: usize, S>(x: &Matrix<T, R, C, S>, y: &Matrix<T, R, C, S>) -> Matrix<T, R, C, S>
where
    T: Copy,
{
    if R == 1 && C == 1 {
        return x.clone() * y.clone();
    }
    // mat = [quad1, quad2; quad3, quad4]
    let p1 = matrix_mult(x_quad1, y_quad2 - y_quad4);
    let p2 = matrix_mult(x_quad1 + x_quad2, y_quad4);
    let p3 = matrix_mult(x_quad3 + x_quad4, y_quad1);
    let p4 = matrix_mult(x_quad4, y_quad3 - y_quad1);
    let p5 = matrix_mult(x_quad1 + x_quad4, y_quad1 + y_quad4);
    let p6 = matrix_mult(x_quad2 - x_quad4, y_quad3 + y_quad4);
    let p7 = matrix_mult(x_quad1 - x_quad3, y_quad1 + y_quad2);

    out_quad1 = p5 + p4 - p2 + p6;
    out_quad2 = p1 + p2;
    out_quad3 = p3 + p4;
    out_quad4 = p1 + p5 - p3 - p7;
    output = [out_quad1, out_quad2; out_quad3, out_quad];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_mult() {
        let x = Matrix::from_iter(2, 2, 1..);
        let y = Matrix::from_iter(2, 2, 5..);
        let target = x.clone() * y.clone();
        let output = matrix_mult(&x, &y);
        assert_eq!(output, target);
    }
}
 */
