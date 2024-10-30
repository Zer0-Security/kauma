pub fn execute(number1: i32, number2: i32) -> i32 {
    number1 + number2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_positive_numbers() {
        let result = execute(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_two_negative_numbers() {
        let result = execute(-2, -8);
        assert_eq!(result, -10);
    }
    #[test]
    fn add_two_numbers() {
        let result = execute(100, -7);
        assert_eq!(result, 93);
    }
}