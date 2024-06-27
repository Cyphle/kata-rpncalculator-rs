use crate::operators::{find_operator_from, Operators};
use crate::operators::operators::apply_operation;

#[derive(Debug, PartialEq)]
pub struct Operation {
    operands: Vec<Operand>,
    operator: Operators,
}

#[derive(Debug, PartialEq)]
pub struct Operand {
    value: Option<String>,
    operation: Option<Operation>,
}

pub fn build_operations_from_old(instructions: &Vec<String>) -> Operation {
    let mut operation = Operation {
        operands: vec![],
        operator: Operators::IDENTITY,
    };

    let mut raw_operands = vec![];
    for i in instructions {
        let i_as_char = i.chars().nth(0).unwrap();
        let op = find_operator_from(&i_as_char);

        match op {
            None => {
                raw_operands.push(i.clone())
            }
            Some(o) => {
                let mut operands: Vec<Operand> = vec![
                    Operand {
                        value: None,
                        operation: Some(operation),
                    }
                ];
                raw_operands
                    .iter()
                    .for_each(|x| operands.push(Operand {
                        value: Some(x.clone()),
                        operation: None,
                    }));

                operation = Operation {
                    operands,
                    operator: o,
                };
                raw_operands.clear()
            }
        }
    }

    operation
}

pub fn apply_calculation(instructions: &Vec<String>) -> Vec<String> {
    let mut new_instructions: Vec<String> = vec![];
    for (index, instruction) in instructions.iter().enumerate() {
        // TODO faudrait virer le unwrap c'est not safe
        let instruction_first_char = instruction.chars().nth(0).unwrap();
        let operator = find_operator_from(&instruction_first_char);
        match operator {
            None => {}
            Some(ope) => {
                // TODO faudrait virer les unwrap c'est not safe
                let first_operand = instructions.iter().nth(index - 2).unwrap().parse::<f32>().unwrap();
                let second_operand = instructions.iter().nth(index - 1).unwrap().parse::<f32>().unwrap();

                let result = apply_operation(ope, first_operand, second_operand);

                let _ = &instructions[0..(index-2)].iter().for_each(|x| new_instructions.push(x.clone()));
                new_instructions.push(result.to_string());
                let _ = &instructions[(index + 1)..instructions.len()].iter().for_each(|x| new_instructions.push(x.clone()));

                break;
            }
        };
    }

    if new_instructions.len() > 1 {
        apply_calculation(&new_instructions)
    } else {
        new_instructions
    }
}

#[cfg(test)]
mod build_operations_from_tests {
    mod build_operations {
        use crate::operators::operations::apply_calculation;

        #[test]
        fn should_calculation_from_simple_instructions() {
            // 5 3 + => 5+3 => 8
            let instructions = vec!["5".to_string(), "3".to_string(), "+".to_string()];

            let result = apply_calculation(&instructions);

            assert_eq!(result, vec!["8".to_string()])
        }

        #[test]
        fn should_calculation_from_simple_instructions_when_divide() {
            // 6 2 / => 6/2 => 3
            let instructions = vec!["6".to_string(), "2".to_string(), "/".to_string()];

            let result = apply_calculation(&instructions);

            assert_eq!(result, vec!["3".to_string()])
        }

        #[test]
        fn should_calculation_when_two_operations() {
            // 5 2 - 7 + => (5 2 -) 7 + => (5 - 2) + 7 => 10
            let instructions = vec![
                "5".to_string(),
                "2".to_string(),
                "-".to_string(),
                "7".to_string(),
                "+".to_string(),
            ];

            let result = apply_calculation(&instructions);

            assert_eq!(result, vec!["10".to_string()]);
        }

        #[test]
        fn should_calculation_when_multiple_operations() {
            // 3 4 2 1 + x + 2 / => (3 + (4 x (2+1)))/2 => 7.5
            let instructions = vec![
                "3".to_string(),
                "4".to_string(),
                "2".to_string(),
                "1".to_string(),
                "+".to_string(),
                "x".to_string(),
                "+".to_string(),
                "2".to_string(),
                "/".to_string()
            ];

            let result = apply_calculation(&instructions);

            assert_eq!(result, vec!["7.5".to_string()]);
        }

        #[test]
        fn should_calculation_when_multiple_operations_again() {
            // 1 2 + 4 × 5 + 3 − => ((1+2) x 4) + 5 - 3 => 14
            let instructions = vec![
                "1".to_string(),
                "2".to_string(),
                "+".to_string(),
                "4".to_string(),
                "x".to_string(),
                "5".to_string(),
                "+".to_string(),
                "3".to_string(),
                "-".to_string()
            ];

            let result = apply_calculation(&instructions);

            assert_eq!(result, vec!["14".to_string()]);
        }

        #[test]
        fn should_calculation_when_multiple_operations_and_again() {
            // 5 4 1 2 + × + => 5 + (4 x (1+2)) => 17
            let instructions = vec![
                "5".to_string(),
                "4".to_string(),
                "1".to_string(),
                "2".to_string(),
                "+".to_string(),
                "*".to_string(),
                "+".to_string()
            ];

            let result = apply_calculation(&instructions);

            assert_eq!(result, vec!["17".to_string()]);
        }
    }

    mod build_operations_old {
        use crate::operators::operations::{build_operations_from_old, Operand, Operation};
        use crate::operators::Operators;

        #[test]
        fn should_build_an_operation_from_simple_instructions() {
            // 5 3 + => 5+3 => 8
            let instructions = vec!["5".to_string(), "3".to_string(), "+".to_string()];

            let result = build_operations_from_old(&instructions);

            assert_eq!(result, Operation {
                operands: vec![
                    Operand {
                        value: None,
                        operation: Some(Operation {
                            operands: vec![],
                            operator: Operators::IDENTITY,
                        }),
                    },
                    Operand {
                        value: Some("5".to_string()),
                        operation: None,
                    },
                    Operand {
                        value: Some("3".to_string()),
                        operation: None,
                    },
                ],
                operator: Operators::PLUS,
            })
        }

        #[test]
        fn should_build_an_operation_from_simple_instructions_when_divide() {
            // 6 2 / => 6/2 => 3
            let instructions = vec!["6".to_string(), "2".to_string(), "/".to_string()];

            let result = build_operations_from_old(&instructions);

            assert_eq!(result, Operation {
                operands: vec![
                    Operand {
                        value: None,
                        operation: Some(Operation {
                            operands: vec![],
                            operator: Operators::IDENTITY,
                        }),
                    },
                    Operand {
                        value: Some("6".to_string()),
                        operation: None,
                    },
                    Operand {
                        value: Some("2".to_string()),
                        operation: None,
                    },
                ],
                operator: Operators::DIVIDE,
            })
        }

        #[test]
        fn should_build_an_operation_when_two_operations() {
            // 5 2 - 7 + => (5 2 -) 7 + => (5 - 2) + 7 => 10
            let instructions = vec![
                "5".to_string(),
                "2".to_string(),
                "-".to_string(),
                "7".to_string(),
                "+".to_string(),
            ];

            let result = build_operations_from_old(&instructions);

            assert_eq!(result, Operation {
                operands: vec![
                    Operand {
                        value: None,
                        operation: Some(Operation {
                            operands: vec![
                                Operand {
                                    value: None,
                                    operation: Some(Operation {
                                        operands: vec![],
                                        operator: Operators::IDENTITY,
                                    }),
                                },
                                Operand {
                                    value: Some("5".to_string()),
                                    operation: None,
                                },
                                Operand {
                                    value: Some("2".to_string()),
                                    operation: None,
                                },
                            ],
                            operator: Operators::MINUS,
                        }),
                    },
                    Operand {
                        value: Some("7".to_string()),
                        operation: None,
                    },
                ],
                operator: Operators::PLUS,
            })
        }
    }
}