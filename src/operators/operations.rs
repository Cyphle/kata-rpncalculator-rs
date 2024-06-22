use crate::operators::{find_operator_from, Operators};

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

pub fn build_operations_from(instructions: &Vec<String>) -> Operation {
    let mut operation = Operation {
        operands: vec![],
        operator: Operators::IDENTITY,
    };

    /*
        En fait, il faut walk through instruction et collect les instructions jusqu'à ce qu'on tombe sur une operation
        des qu'on trouve un operateur, on construit une operation
        faut update l'operation à retourner

        let first = Operation {
            operands: vec![init, 5, 2],
            operator: Operators.MINUS
        }

        Operation {
            operands: vec![first, 7],
            operator: Operator.PLUS
        }
     */

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

        let result = build_operations_from(&instructions);

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
            "+".to_string()
        ];

        let result = build_operations_from(&instructions);

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
                                })
                            },
                            Operand {
                                value: Some("5".to_string()),
                                operation: None,
                            },
                            Operand {
                                value: Some("2".to_string()),
                                operation: None,
                            }
                        ],
                        operator: Operators::MINUS
                    })
                },
                Operand {
                    value: Some("7".to_string()),
                    operation: None,
                }
            ],
            operator: Operators::PLUS
        })
    }
}