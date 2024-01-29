#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Casing {
    CamelCase,
    PascalCase,
    LowerCase,
    KebabCase,
    SnakeCase,
    UpperCase,
    UpperSnakeCase,
    UpperKebabCase,
}

impl Casing {
    pub fn detect_casing(s: &str) -> Option<Casing> {
        if s.chars().all(|c| c.is_lowercase()) {
            return Some(Casing::LowerCase);
        }

        if s.chars().all(|c| c.is_uppercase()) {
            return Some(Casing::UpperCase);
        }

        if s.contains('_') && s.chars().all(|c| c == '_' || c.is_lowercase()) {
            return Some(Casing::SnakeCase);
        }

        if s.contains('-') && s.chars().all(|c| c == '-' || c.is_lowercase()) {
            return Some(Casing::KebabCase);
        }

        if s.chars().all(|c| c.is_uppercase() || c == '_') {
            return Some(Casing::UpperSnakeCase);
        }

        if s.chars().all(|c| c.is_uppercase() || c == '-') {
            return Some(Casing::UpperKebabCase);
        }

        // NOTE: PascalCase and CamelCase checks depend on position
        // which means the above checks are necessary and these cannot be moved arbitrarily

        if s.chars().next().is_some_and(|c| c.is_lowercase())
            && !s.contains("-")
            && !s.contains("_")
        {
            return Some(Casing::CamelCase);
        }

        if s.chars().next().is_some_and(|c| c.is_uppercase())
            && !s.contains("-")
            && !s.contains("_")
        {
            return Some(Casing::PascalCase);
        }

        None
    }
}

#[cfg(test)]
mod test_casing {
    use super::Casing;

    fn assert_inputs_casing(inputs: &[&str], casing: Option<Casing>) {
        assert!(inputs.iter().all(|i| Casing::detect_casing(i) == casing));
    }

    #[test]
    fn lower_case() {
        let inputs = vec!["lowercase", "keys", "inputs"];

        assert_inputs_casing(&inputs, Some(Casing::LowerCase));
    }

    #[test]
    fn upper_case() {
        let inputs = vec!["UPPERCASE", "KEYS", "INPUTS"];

        assert_inputs_casing(&inputs, Some(Casing::UpperCase));
    }

    #[test]
    fn snake_case() {
        let inputs = vec!["snake_case", "with_underscore", "multiple_words"];

        assert_inputs_casing(&inputs, Some(Casing::SnakeCase));
    }

    #[test]
    fn kebab_case() {
        let inputs = vec!["kebab-case", "with-hyphen", "multiple-words"];

        assert_inputs_casing(&inputs, Some(Casing::KebabCase));
    }

    #[test]
    fn camel_case() {
        let inputs = vec!["camelCase", "withMixedCase", "multipleWords"];

        assert_inputs_casing(&inputs, Some(Casing::CamelCase));
    }

    #[test]
    fn pascal_case() {
        let inputs = vec!["PascalCase", "WithMixedCase", "MultipleWords"];

        assert_inputs_casing(&inputs, Some(Casing::PascalCase));
    }

    #[test]
    fn upper_snake_case() {
        let inputs = vec!["UPPER_SNAKE_CASE", "WITH_UNDERSCORE", "MULTIPLE_WORDS"];

        assert_inputs_casing(&inputs, Some(Casing::UpperSnakeCase));
    }

    #[test]
    fn upper_kebab_case() {
        let inputs = vec!["UPPER-KEBAB-CASE", "WITH-HYPHEN", "MULTIPLE-WORDS"];

        assert_inputs_casing(&inputs, Some(Casing::UpperKebabCase));
    }

    #[test]
    fn invalid_case() {
        let inputs = vec!["InVA-lid", "INVa_lid", "in-Va_lid", "in-va_lid"];

        assert_inputs_casing(&inputs, None);
    }
}
