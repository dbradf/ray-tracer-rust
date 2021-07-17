const EPSILON: f64 = 0.00001;

pub fn equal_f64(x: f64, y: f64) -> bool {
    if (x - y).abs() < EPSILON {
        true
    } else {
        false
    }
}

#[cfg(tests)]
mod tests {
    #[test]
    fn test_equal_f64_should_return_true_for_eq() {
        assert!(equal_f64(3.0, 3.0));
        assert!(equal_f64(3.14, 3.14));
    }

    #[test]
    fn test_equal_f64_should_return_false_for_non_eq() {
        assert!(!equal_f64(3.0, 2.9));
        assert!(!equal_f64(3.14, 3.13));
    }
}

