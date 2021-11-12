pub const EPSILON: f64 = 0.0001;

pub(crate) fn epsilon_equal(x: f64, y: f64) -> bool {
    (x - y).abs() < EPSILON
}

#[cfg(test)]
mod equal_tests {
    use crate::epsilon::epsilon_equal;

    #[test]
    fn test_epsilon_equal() {
        assert!(epsilon_equal(1.0f64, 1.00000000001f64));
        assert!(!epsilon_equal(1.0f64, 1.1f64));
    }

    #[test]
    fn test_epsilon_equal_nan() {
        assert!(!epsilon_equal(f64::NAN, f64::NAN));
        assert!(!epsilon_equal(f64::NAN, 1.1f64));
    }
}
