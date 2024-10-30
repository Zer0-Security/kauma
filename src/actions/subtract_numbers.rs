pub fn execute(number1: i32, number2: i32) -> i32 {
    number1 - number2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subtract_two_positive_numbers() {
        let result = execute(7, 3);
        assert_eq!(result, 4);
    }

    #[test]
    fn subtract_two_negative_numbers() {
        let result = execute(-2, -8);
        assert_eq!(result, 6);
    }
    #[test]
    fn subtract_numbers_two_numbers() {
        let result = execute(100, -7);
        assert_eq!(result, 107);
    }
}