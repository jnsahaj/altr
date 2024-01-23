use crate::SEPARATOR;

pub struct Token(String);

impl Token {
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
}
