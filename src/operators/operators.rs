#[derive(PartialEq, Debug)]
pub enum Operators {
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    IDENTITY // TODO je pense qu'en fait identity y a pas besoin
}

pub fn find_operator_from(symbol: &char) -> Option<Operators> {
    match symbol {
        &'+' => Some(Operators::PLUS),
        &'-' => Some(Operators::MINUS),
        &'*' => Some(Operators::MULTIPLY),
        &'/' => Some(Operators::DIVIDE),
        _ => None
    }
}

// TODO to be tested et changer en floatant
pub fn apply_operation_i64(operator: Operators, first_operand: i64, second_operand: i64) -> i64 {
    match operator {
        Operators::PLUS => { first_operand + second_operand }
        Operators::MINUS => { first_operand - second_operand }
        Operators::MULTIPLY => { first_operand * second_operand }
        Operators::DIVIDE => { first_operand / second_operand }
        Operators::IDENTITY => { first_operand }
    }
}

pub fn apply_operation(operator: Operators, first_operand: f32, second_operand: f32) -> f32 {
    match operator {
        Operators::PLUS => { first_operand + second_operand }
        Operators::MINUS => { first_operand - second_operand }
        Operators::MULTIPLY => { first_operand * second_operand }
        Operators::DIVIDE => { first_operand / second_operand }
        Operators::IDENTITY => { first_operand }
    }
}

#[cfg(test)]
mod find_operator_from_tests {
    mod find_operator {
        use crate::operators::find_operator_from;
        use crate::operators::Operators::{DIVIDE, MINUS, MULTIPLY, PLUS};

        #[test]
        fn should_return_unknown_when_char_is_not_an_operator() {
            let symbol = '2';

            let result = find_operator_from(&symbol);

            assert_eq!(result, None);
        }

        #[test]
        fn should_return_plus_when_char_is_plus_operator() {
            let symbol = '+';

            let result = find_operator_from(&symbol);

            assert_eq!(result, Some(PLUS));
        }

        #[test]
        fn should_return_minus_when_char_is_minus_operator() {
            let symbol = '-';

            let result = find_operator_from(&symbol);

            assert_eq!(result, Some(MINUS));
        }

        #[test]
        fn should_return_multiply_when_char_is_multiply_operator() {
            let symbol = '*';

            let result = find_operator_from(&symbol);

            assert_eq!(result, Some(MULTIPLY));
        }

        #[test]
        fn should_return_divide_when_char_is_divide_operator() {
            let symbol = '/';

            let result = find_operator_from(&symbol);

            assert_eq!(result, Some(DIVIDE));
        }
    }

    mod apply_operation {
        use crate::operators::Operators;
        use crate::operators::operators::apply_operation;

        #[test]
        fn should_apply_addition() {
            let res1 = apply_operation(Operators::PLUS, 1.0, 2.0);
            let res2 = apply_operation(Operators::PLUS, 1.3, 4.3);

            assert_eq!(res1, 3.0);
            assert_eq!(res2, 5.6);
        }

        #[test]
        fn should_apply_substraction() {
            let res1 = apply_operation(Operators::MINUS, 1.0, 2.0);
            let res2 = apply_operation(Operators::MINUS, 9.3, 4.3);

            assert_eq!(res1, -1.0);
            assert_eq!(res2, 5.0);
        }
    }
}