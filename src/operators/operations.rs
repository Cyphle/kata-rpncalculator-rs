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

pub fn build_operations_from(instructions: &Vec<String>) -> Vec<String> {
    /*
En fait, chaque operateur suit ses operandes donc l'algo n'est pas bon.
Il faut walk through les instructions et déterminer s'il s'agit d'une operande ou d'un operateur
Si on trouve un operateur, on applique au deux précédents
Puis on a une nouvelle série avec les n-3-k et les n+m
 */
    let mut new_instructions: Vec<String> = vec![];
    for (index, instruction) in instructions.iter().enumerate() {
        let instruction_first_char = instruction.chars().nth(0).unwrap();
        let operator = find_operator_from(&instruction_first_char);
        match operator {
            None => {}
            Some(ope) => {
                // TODO faudrait virer les unwrap c'est not safe
                let first_operand = instructions.iter().nth(index - 2).unwrap().parse::<i64>().unwrap();
                let second_operand = instructions.iter().nth(index - 1).unwrap().parse::<i64>().unwrap();

                let result = apply_operation(ope, first_operand, second_operand);

                &instructions[0..(index-2)].iter().for_each(|x| new_instructions.push(x.clone()));
                new_instructions.push(result.to_string());
                &instructions[(index + 1)..instructions.len()].iter().for_each(|x| new_instructions.push(x.clone()));

                break;
            }
        };
    }

    return new_instructions;
}

#[cfg(test)]
mod build_operations_from_tests {
    use crate::operators::operations::{build_operations_from, Operand, Operation};
    use crate::operators::Operators;

    mod build_operations {
        use crate::operators::operations::{build_operations_from, Operand, Operation};
        use crate::operators::Operators;

        #[test]
        fn should_build_an_operation_from_simple_instructions() {
            // 5 3 + => 5+3 => 8
            let instructions = vec!["5".to_string(), "3".to_string(), "+".to_string()];

            let result = build_operations_from(&instructions);

            assert_eq!(result, vec!["8".to_string()])
        }

        #[test]
        fn should_build_an_operation_from_simple_instructions_when_divide() {
            // 6 2 / => 6/2 => 3
            let instructions = vec!["6".to_string(), "2".to_string(), "/".to_string()];

            let result = build_operations_from(&instructions);

            assert_eq!(result, vec!["3".to_string()])
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

            let result = build_operations_from(&instructions);

            assert_eq!(result, vec!["3".to_string(), "7".to_string(), "+".to_string()])
        }
    }

    mod build_operations_old {
        use crate::operators::operations::{build_operations_from, build_operations_from_old, Operand, Operation};
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