use crate::operators::{find_operator_from, Operators};

#[derive(Debug, PartialEq)]
pub struct Operation {
    operands: Vec<Operand>,
    operator: Operators,
}

#[derive(Debug, PartialEq)]
pub struct Operand {
    value: String,
    operation: Option<Operation>,
}

pub fn build_operations_from(instructions: &Vec<String>) -> Operation {
    let mut operation = Operation {
        operands: vec![],
        operator: Operators::IDENTITY,
    };

    for i in instructions {
        let i_as_char = i.chars().nth(0).unwrap();
        let op = find_operator_from(&i_as_char);

        match op {
            None => {
                operation.operands.push(Operand {
                    value: i.clone(),
                    operation: None,
                })
            }
            Some(o) => { operation.operator = o }
        }
    }

    operation
}

#[cfg(test)]
mod build_operations_from_tests {
    use crate::operators::operations::{build_operations_from, Operand, Operation};
    use crate::operators::Operators;

    #[test]
    fn should_build_an_operation_from_simple_instructions() {
        // 5 3 + => 5+3 => 8
        let instructions = vec!["5".to_string(), "3".to_string(), "+".to_string()];

        let result = build_operations_from(&instructions);

        assert_eq!(result, Operation {
            operands: vec![
                Operand {
                    value: "5".to_string(),
                    operation: None,
                },
                Operand {
                    value: "3".to_string(),
                    operation: None,
                }
            ],
            operator: Operators::PLUS,
        })
    }

    #[test]
    fn should_build_an_operation_from_simple_instructions_when_divide() {
        // 6 2 / => 6/2 => 3
        let instructions = vec!["6".to_string(), "2".to_string(), "/".to_string()];

        let result = build_operations_from(&instructions);

        assert_eq!(result, Operation {
            operands: vec![
                Operand {
                    value: "6".to_string(),
                    operation: None,
                },
                Operand {
                    value: "2".to_string(),
                    operation: None,
                }
            ],
            operator: Operators::DIVIDE,
        })
    }

    #[test]
    fn should_build_an_operation_when_two_operations() {
        // 5 2 - 7 + => (5 2 -) 7 + => (5 - 2) + 7 => 10
        /*
        struct Operation {
            left_operand: Operation,
            right_operand: Option<Operation>,
            operator: Operators
        }

        Il faudrait un Operators::Identity
        et il faudrait walk through vec

         */
    }
}