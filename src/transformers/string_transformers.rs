pub fn from_string_to_char_vec(original: &str) -> Vec<char> {
    original.chars().into_iter().filter(|c| c != &' ').collect()
}

#[cfg(test)]
mod from_string_to_char_vec_tests {
    use crate::transformers::string_transformers::from_string_to_char_vec;

    #[test]
    fn should_split_string_into_vec_of_chars() {
        let original = "5 3 +";

        let result = from_string_to_char_vec(original);

        assert_eq!(result, vec!['5', '3', '+']);
    }
}