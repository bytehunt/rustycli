pub fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_positive_numbers() {
        assert_eq!(max(2, 3), 3);
    }

    #[test]
    fn test_max_negative_numbers() {
        assert_eq!(max(-5, -7), -5);
    }

    #[test]
    fn test_max_positive_and_negative_numbers() {
        assert_eq!(max(-10, 8), 8);
    }
}
