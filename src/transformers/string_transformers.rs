pub fn from_string_to_char_vec(original: &str) -> Vec<String> {
    original.split(' ').filter(|&x| !x.is_empty()).map(|x| x.to_string()).collect()
}

#[cfg(test)]
mod from_string_to_char_vec_tests {
    use crate::transformers::string_transformers::from_string_to_char_vec;

    #[test]
    fn should_split_string_of_instructions_into_vec_of_chars() {
        let original = "5 3 +";

        let result = from_string_to_char_vec(original);

        assert_eq!(result, vec!["5", "3", "+"]);
    }

    #[test]
    fn should_split_string_of_instructions_into_vec_of_string_representing_instructions() {
        let original = "53 23 +";

        let result = from_string_to_char_vec(original);

        assert_eq!(result, vec!["53", "23", "+"]);
    }
}