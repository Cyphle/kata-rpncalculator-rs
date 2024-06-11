use crate::operators::Operators;

pub struct Operation {
    left_operand: String,
    right_operand: String,
    operator: Operators
}

pub fn build_operations_from(instructions: &Vec<String>) -> Operation {

}

#[cfg(test)]
mod build_operations_from_tests {
    use crate::operators::operations::{build_operations_from, Operation};
    use crate::operators::Operators;

    #[test]
    fn should_build_an_operation_from_simple_instructions() {
        let instructions = vec!["5".to_string(), "3".to_string(), "+".to_string()];

        let result = build_operations_from(&instructions);

        assert_eq!(result, Operation {
            left_operand: "5".to_string(),
            right_operand: "3".to_string(),
            operator: Operators::PLUS
        })
    }
}