pub const EPSILON: f64 = 0.0001;

/// Represents epsilon equality.
pub trait EpsilonEqual {
    /// true, if self is not farther than EPSILON away from other.
    fn e_equals(self, other: Self) -> bool;
}

impl EpsilonEqual for f64 {
    fn e_equals(self, other: Self) -> bool {
        (self - other).abs() < EPSILON
    }
}


/// for swapping in a more performant int for a float calculation
impl EpsilonEqual for i32 {
    fn e_equals(self, other: Self) -> bool {
        self == other
    }
}

#[cfg(test)]
mod equal_tests {
    use crate::epsilon::EpsilonEqual;

    #[test]
    fn test_epsilon_equal() {
        assert!(1.0f64.e_equals(1.00000000001f64));
        assert!(!1.0f64.e_equals(1.1f64));
    }

    #[test]
    fn test_epsilon_equal_nan() {
        assert!(!f64::NAN.e_equals(f64::NAN));
        assert!(!f64::NAN.e_equals(1.1f64));
    }

    #[test]
    fn test_epsilon_equal_int() {
        assert!(1.e_equals(1));
        assert!(!1.e_equals(2));
    }

}
