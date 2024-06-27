#[derive(PartialEq, Debug)]
pub enum Operators {
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
}

pub fn find_operator_from(symbol: &char) -> Option<Operators> {
    match symbol {
        &'+' => Some(Operators::PLUS),
        &'-' => Some(Operators::MINUS),
        &'*' | &'x' => Some(Operators::MULTIPLY),
        &'/' => Some(Operators::DIVIDE),
        _ => None
    }
}

pub fn apply_operation(left_operand: f32, right_operand: f32, operator: Operators) -> f32 {
    let result = match operator {
        Operators::PLUS => { left_operand + right_operand }
        Operators::MINUS => { left_operand - right_operand }
        Operators::MULTIPLY => { left_operand * right_operand }
        Operators::DIVIDE => { left_operand / right_operand }
    };

    (result  * 100.0).round() / 100.0
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
            let res1 = apply_operation(1.0, 2.0, Operators::PLUS);
            let res2 = apply_operation(1.3, 4.3, Operators::PLUS);

            assert_eq!(res1, 3.00);
            assert_eq!(res2, 5.60);
        }

        #[test]
        fn should_apply_substraction() {
            let res1 = apply_operation(1.0, 2.0, Operators::MINUS);
            let res2 = apply_operation(9.3, 4.3, Operators::MINUS);

            assert_eq!(res1, -1.00);
            assert_eq!(res2, 5.00);
        }

        #[test]
        fn should_apply_multiplication() {
            let res1 = apply_operation(1.0, 2.0, Operators::MULTIPLY);
            let res2 = apply_operation(9.3, 4.3, Operators::MULTIPLY);

            assert_eq!(res1, 2.00);
            assert_eq!(res2, 39.99);
        }

        #[test]
        fn should_apply_division() {
            let res1 = apply_operation(1.0, 2.0, Operators::DIVIDE);
            let res2 = apply_operation(9.3, 4.3, Operators::DIVIDE);

            assert_eq!(res1, 0.50);
            assert_eq!(res2, 2.16);
        }
    }
}