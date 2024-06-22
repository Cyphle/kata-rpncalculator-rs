#[derive(PartialEq, Debug)]
pub enum Operators {
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    IDENTITY
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