use crate::operators::{find_operator_from, Operators};

#[derive(Debug, PartialEq)]
pub struct Operation {
    operands: Vec<String>,
    operator: Operators
}

pub fn build_operations_from(instructions: &Vec<String>) -> Operation {
    let mut operands: Vec<String> = vec![];
    let mut operator: Operators = Operators::PLUS;
    for i in instructions {
        if i.len() > 1 {
            operands.push(i.clone());
        } else {
            let i_as_char = i.chars().nth(0).unwrap();
            let op = find_operator_from(&i_as_char);
            match op {
                None => { operands.push(i.clone())}
                Some(o) => { operator = o }
            }
        }
    }

    Operation {
        operands,
        operator
    }
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
            operands: vec!["5".to_string(), "3".to_string()],
            operator: Operators::PLUS
        })
    }
}