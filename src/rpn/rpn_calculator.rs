


pub fn calculate_with_rpn(input: &str) -> f32 {
    let instructions = crate::transformers::from_string_to_char_vec(input);
    let result = crate::operators::apply_calculation(&instructions);
    match result.iter().nth(0) {
        Some(res) => crate::operators::string_to_f32(res),
        None => panic!("Cannot calculate operation with RPN")
    }
}

#[cfg(test)]
mod calculate_with_rpn_tests {
    use crate::rpn::rpn_calculator::calculate_with_rpn;

    #[test]
    fn should_calculate_operation_with_rpn() {
        let operation = "5 3 +";

        let result = calculate_with_rpn(&operation);

        assert_eq!(result, 8.0);
    }

    #[test]
    fn should_calculation_when_multiple_operations() {
        let operation = "5 4 1 2 + × +";

        let result = calculate_with_rpn(&operation);

        assert_eq!(result, 17.0);
    }

    #[test]
    fn should_calculation_when_multiple_operations_again() {
        let operation = "3 4 2 1 + x + 2 /";

        let result = calculate_with_rpn(&operation);

        assert_eq!(result, 7.5);
    }
}