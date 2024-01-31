#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Casing {
    Camel,
    Pascal,
    Lower,
    Kebab,
    Snake,
    Upper,
    UpperSnake,
    UpperKebab,
}

impl Casing {
    pub fn detect_casing(s: &str) -> Result<Casing, String> {
        if s.chars().all(|c| c.is_lowercase()) {
            return Ok(Casing::Lower);
        }

        if s.chars().all(|c| c.is_uppercase()) {
            return Ok(Casing::Upper);
        }

        if s.contains('_') && s.chars().all(|c| c == '_' || c.is_lowercase()) {
            return Ok(Casing::Snake);
        }

        if s.contains('-') && s.chars().all(|c| c == '-' || c.is_lowercase()) {
            return Ok(Casing::Kebab);
        }

        if s.chars().all(|c| c.is_uppercase() || c == '_') {
            return Ok(Casing::UpperSnake);
        }

        if s.chars().all(|c| c.is_uppercase() || c == '-') {
            return Ok(Casing::UpperKebab);
        }

        // NOTE: PascalCase and CamelCase checks depend on position
        // which means the above checks are necessary and these cannot be moved arbitrarily

        if s.chars().next().is_some_and(|c| c.is_lowercase())
            && !s.contains('-')
            && !s.contains('_')
        {
            return Ok(Casing::Camel);
        }

        if s.chars().next().is_some_and(|c| c.is_uppercase())
            && !s.contains('-')
            && !s.contains('_')
        {
            return Ok(Casing::Pascal);
        }

        Err(format!("Failed to detect casing for {}", s))
    }
}

#[cfg(test)]
mod test_casing {
    use super::Casing;

    fn assert_inputs_casing(inputs: &[&str], casing: Casing) {
        assert!(inputs
            .iter()
            .all(|i| Casing::detect_casing(i).unwrap() == casing));
    }

    #[test]
    fn lower_case() {
        let inputs = vec!["lowercase", "keys", "inputs"];

        assert_inputs_casing(&inputs, Casing::Lower);
    }

    #[test]
    fn upper_case() {
        let inputs = vec!["UPPERCASE", "KEYS", "INPUTS"];

        assert_inputs_casing(&inputs, Casing::Upper);
    }

    #[test]
    fn snake_case() {
        let inputs = vec!["snake_case", "with_underscore", "multiple_words"];

        assert_inputs_casing(&inputs, Casing::Snake);
    }

    #[test]
    fn kebab_case() {
        let inputs = vec!["kebab-case", "with-hyphen", "multiple-words"];

        assert_inputs_casing(&inputs, Casing::Kebab);
    }

    #[test]
    fn camel_case() {
        let inputs = vec!["camelCase", "withMixedCase", "multipleWords"];

        assert_inputs_casing(&inputs, Casing::Camel);
    }

    #[test]
    fn pascal_case() {
        let inputs = vec!["PascalCase", "WithMixedCase", "MultipleWords"];

        assert_inputs_casing(&inputs, Casing::Pascal);
    }

    #[test]
    fn upper_snake_case() {
        let inputs = vec!["UPPER_SNAKE_CASE", "WITH_UNDERSCORE", "MULTIPLE_WORDS"];

        assert_inputs_casing(&inputs, Casing::UpperSnake);
    }

    #[test]
    fn upper_kebab_case() {
        let inputs = vec!["UPPER-KEBAB-CASE", "WITH-HYPHEN", "MULTIPLE-WORDS"];

        assert_inputs_casing(&inputs, Casing::UpperKebab);
    }

    #[test]
    fn invalid_case() {
        let inputs = vec!["InVA-lid", "INVa_lid", "in-Va_lid", "in-va_lid"];

        assert!(inputs.iter().all(|i| Casing::detect_casing(i).is_err()));
    }
}

#[derive(Debug)]
pub enum CasingSeparator {
    None,
    Underscore,
    Hyphen,
}

impl From<Casing> for CasingSeparator {
    fn from(value: Casing) -> Self {
        match value {
            Casing::Camel => CasingSeparator::None,
            Casing::Pascal => CasingSeparator::None,
            Casing::Lower => CasingSeparator::None,
            Casing::Kebab => CasingSeparator::Hyphen,
            Casing::Snake => CasingSeparator::Underscore,
            Casing::Upper => CasingSeparator::None,
            Casing::UpperSnake => CasingSeparator::Underscore,
            Casing::UpperKebab => CasingSeparator::Hyphen,
        }
    }
}

impl From<Option<Casing>> for CasingSeparator {
    fn from(value: Option<Casing>) -> Self {
        if let Some(v) = value {
            v.into()
        } else {
            CasingSeparator::None
        }
    }
}
