#[derive(PartialEq, Debug)]
enum Operators {
    PLUS,
    MINUS,
    MULTIPLY,
    SUBTRACT
}

fn find_operator_from(symbol: &char) -> Option<Operators> {
    None
}

#[cfg(test)]
mod find_operator_from_tests {
    use crate::operators::operators::{find_operator_from, Operators};

    #[test]
    fn should_return_unknown_when_char_is_not_an_operator() {
        let symbol = '2';

        let result = find_operator_from(&symbol);

        assert_eq!(result, None);
    }
}