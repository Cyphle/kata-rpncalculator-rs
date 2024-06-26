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

// TODO to be tested
pub fn apply_operation(operator: Operators, first_operand: i64, second_operand: i64) -> i64 {
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
    use crate::operators::operators::find_operator_from;
    use crate::operators::operators::Operators::{DIVIDE, MINUS, MULTIPLY, PLUS};

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