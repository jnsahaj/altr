use std::str::FromStr;

use crate::{casing::Casing, SEPARATOR};

#[derive(Debug)]
pub enum TokenError {
    AmbiguousToLowerCase,
    AmbiguousToUpperCase,
}

#[derive(Debug)]
pub struct Token(String);

impl Token {
    pub fn try_to_casing(&self, casing: &Casing) -> Result<String, TokenError> {
        match casing {
            Casing::CamelCase => Ok(self.to_camel_case()),
            Casing::PascalCase => Ok(self.to_pascal_case()),
            Casing::LowerCase => self.try_to_lower_case(),
            Casing::KebabCase => todo!(),
            Casing::SnakeCase => Ok(self.to_snake_case()),
            Casing::UpperCase => self.try_to_upper_case(),
            Casing::UpperSnakeCase => Ok(self.to_upper_snake_case()),
            Casing::UpperKebabCase => todo!(),
        }
    }

    pub fn from_casing(casing: &Casing, input: &str) -> Option<Self> {
        match casing {
            Casing::CamelCase => Token::from_camel_case(input),
            Casing::PascalCase => Token::from_pascal_case(input),
            Casing::LowerCase => Some(Token(input.into())),
            Casing::KebabCase => todo!(),
            Casing::SnakeCase => Token::from_snake_case(input),
            Casing::UpperCase => Some(Token(input.into())),
            Casing::UpperSnakeCase => todo!(),
            Casing::UpperKebabCase => todo!(),
        }
    }

    pub fn from_camel_case(input: &str) -> Option<Self> {
        let mut result = String::new();

        for (i, ch) in input.chars().enumerate() {
            if ch.is_uppercase() {
                if i == 0 {
                    return None;
                } else {
                    result.push(SEPARATOR);
                    result.push(ch.to_ascii_lowercase());
                }
            } else {
                result.push(ch);
            }
        }

        Some(Token(result))
    }

    pub fn from_snake_case(input: &str) -> Option<Self> {
        Some(Token(input.replace("_", &String::from(SEPARATOR))))
    }

    pub fn from_pascal_case(input: &str) -> Option<Self> {
        let mut result = String::new();

        for (i, ch) in input.chars().enumerate() {
            if ch.is_uppercase() {
                if i == 0 {
                    result.push(ch.to_ascii_lowercase());
                } else {
                    result.push(SEPARATOR);
                    result.push(ch.to_ascii_lowercase());
                }
            } else {
                if i == 0 {
                    return None;
                }
                result.push(ch);
            }
        }

        Some(Token(result))
    }

    pub fn to_camel_case(&self) -> String {
        self.0
            .split(SEPARATOR)
            .enumerate()
            .map(|(i, part)| {
                if i == 0 {
                    part.to_string()
                } else {
                    part.to_string()
                        .char_indices()
                        .map(|(i, ch)| if i == 0 { ch.to_ascii_uppercase() } else { ch })
                        .collect()
                }
            })
            .collect::<Vec<_>>()
            .join("")
    }

    pub fn to_snake_case(&self) -> String {
        self.0.replace(&String::from(SEPARATOR), "_")
    }

    pub fn to_upper_snake_case(&self) -> String {
        self.to_snake_case().to_ascii_uppercase()
    }

    pub fn to_pascal_case(&self) -> String {
        self.0
            .split(SEPARATOR)
            .map(|part| {
                part.to_string()
                    .char_indices()
                    .map(|(i, ch)| if i == 0 { ch.to_ascii_uppercase() } else { ch })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("")
    }

    pub fn try_to_lower_case(&self) -> Result<String, TokenError> {
        if self.0.contains(SEPARATOR) {
            return Err(TokenError::AmbiguousToLowerCase);
        } else {
            Ok(self.0.clone())
        }
    }

    pub fn try_to_upper_case(&self) -> Result<String, TokenError> {
        if self.0.contains(SEPARATOR) {
            return Err(TokenError::AmbiguousToUpperCase);
        } else {
            Ok(self.0.to_ascii_uppercase().clone())
        }
    }
}

impl FromStr for Token {
    type Err = u8;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Token::from_casing(
            &Casing::detect_casing(s).expect(&format!("Failed to detect casing for {}", s)),
            s,
        )
        .ok_or(0)
    }
}
