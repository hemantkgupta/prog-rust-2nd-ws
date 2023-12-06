pub fn generic_const_structs_work(){
    use std::f64::consts::FRAC_PI_2;   // π/2

    // Approximate the `sin` function: sin x ≅ x - 1/6 x³ + 1/120 x⁵
    // Around zero, it's pretty accurate!
    let sine_poly = Polynomial::new([0.0, 1.0, 0.0, -1.0/6.0, 0.0,
                                    1.0/120.0]);
    assert_eq!(sine_poly.eval(0.0), 0.0);
    assert!((sine_poly.eval(FRAC_PI_2) - 1.).abs() < 0.005);
}

// A polynomial of degree N - 1.
struct Polynomial<const N: usize> {
    /// The coefficients of the polynomial.
    ///
    /// For a polynomial a + bx + cx² + ... + zxⁿ⁻¹,
    /// the `i`'th element is the coefficient of xⁱ.
    coefficients: [f64; N]
}

impl<const N: usize> Polynomial<N> {
    fn new(coefficients: [f64; N]) -> Polynomial<N> {
        Polynomial { coefficients }
    }

    /// Evaluate the polynomial at `x`.
    fn eval(&self, x: f64) -> f64 {
        // Horner's method is numerically stable, efficient, and simple:
        // c₀ + x(c₁ + x(c₂ + x(c₃ + ... x(c[n-1] + x c[n]))))
        let mut sum = 0.0;
        for i in (0..N).rev() {
            sum = self.coefficients[i] + x * sum;
        }

        sum
    }
}

struct LumpOfReferences<'a, T, const N: usize> {
    the_lump: [&'a T; N]
}