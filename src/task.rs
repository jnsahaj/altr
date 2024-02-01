use std::io;

use crate::{
    casing::{Casing, CasingSeparator},
    record::Records,
    task::offset::Offset,
    token::{Token, TokenError},
};

mod offset;

#[derive(Debug)]
pub struct Task<'a> {
    candidate: Token,
    rename: Token,
    preferred_casing_separator: CasingSeparator,
    buf: &'a str,
}

impl<'a> Task<'a> {
    pub fn build(candidate: &str, rename: &str, buf: &'a str) -> Result<Self, String> {
        Ok(Self {
            candidate: candidate.parse()?,
            rename: rename.parse()?,
            preferred_casing_separator: Casing::detect_casing(rename)?.into(),
            buf,
        })
    }

    pub fn generate_records(&mut self) -> Result<Records, io::Error> {
        let mut records = Records::new();

        // collection of casings to operate on
        let casings: Vec<_> = vec![
            Casing::Lower,
            Casing::Pascal,
            Casing::Camel,
            Casing::Snake,
            Casing::Upper,
            Casing::UpperSnake,
            Casing::Kebab,
            Casing::UpperKebab,
        ];

        let casified_candidates: Vec<_> = casings
            .iter()
            .map(|casing| self.candidate.try_to_casing(casing))
            .collect();

        let casing_with_candidates: Vec<(&Casing, &String)> = casings
            .iter()
            .zip(casified_candidates.iter())
            // NOTE: Ambiguity errors are noop matching cases since those will be automatically
            // handled by token conversion to cases like camelCase or UpperSnakeCase
            // Example: "user" is the same in both camelCase and lowercase, hence we ignore the lowercase
            // ambiguity error here
            // As a side-effect, pure lowercase/uppercase matches will be ignored
            // Example: "myUser" candidate will not altr "myuser"
            .filter(|(_, p)| p.is_ok())
            .map(|(c, p)| (c, p.as_ref().expect("fitering Ok values")))
            .collect();

        let mut line_offset: usize = 0;

        for line in self.buf.lines() {
            for (casing, pattern) in casing_with_candidates.iter() {
                let matches = line.match_indices(*pattern);

                for item in matches {
                    let _ =
                        records.try_insert(item.0 + line_offset, pattern.len(), (**casing).clone());
                }
            }

            line_offset += line.len() + 1; // + 1 accounts for the \n character
        }

        Ok(records)
    }

    pub fn process_records(&mut self, records: &mut Records) -> String {
        let mut buf = self.buf.to_string();

        dbg!(&records);

        let mut offset = Offset::Pos(0);

        for (_, record) in records.iter() {
            let rename =
                self.rename
                    .try_to_casing(&record.casing)
                    .unwrap_or_else(|err| match err {
                        TokenError::AmbiguousToLowerCase => match self.preferred_casing_separator {
                            CasingSeparator::None => self.rename.to_camel_case(),
                            CasingSeparator::Underscore => self.rename.to_snake_case(),
                            CasingSeparator::Hyphen => self.rename.to_kebab_case(),
                        },
                        TokenError::AmbiguousToUpperCase => match self.preferred_casing_separator {
                            CasingSeparator::None => self.rename.to_upper_snake_case(),
                            CasingSeparator::Underscore => self.rename.to_upper_snake_case(),
                            CasingSeparator::Hyphen => self.rename.to_upper_kebab_case(),
                        },
                    });

            let start = offset.apply(record.pos);
            let end = offset.apply(record.pos + record.len);

            buf.replace_range(start..end, &rename);

            offset = Offset::add(offset, Offset::from_diff(rename.len(), record.len));
        }

        buf
    }
}

#[cfg(test)]
mod test_task {

    use super::*;

    fn assert_expected<'a>(candidate: &'a str, rename: &'a str, input: &'a str, expected: &'a str) {
        let mut task = Task::build(candidate, rename, input).unwrap();

        let mut records = task.generate_records().unwrap();
        let result = task.process_records(&mut records);

        assert_eq!(result, expected, "Result: {}", result);
    }

    #[test]
    fn test_1() {
        let candidate = "user";
        let rename = "supplyUser";

        let input = r"user User USER";
        let expected = r"supplyUser SupplyUser SUPPLY_USER";

        assert_expected(candidate, rename, input, expected);
    }

    #[test]
    fn test_2() {
        let candidate = "user";
        let rename = "dayTrader";

        let input = r#"
                const [user, setUser] = useState("");

                function getUserName() {
                    return user.name;
                }
                
                function setUserName(name: IUserName) {
                    // could also have been USER
                    user.name = name;
                }
        "#;

        let expected = r#"
                const [dayTrader, setDayTrader] = useState("");

                function getDayTraderName() {
                    return dayTrader.name;
                }
                
                function setDayTraderName(name: IDayTraderName) {
                    // could also have been DAY_TRADER
                    dayTrader.name = name;
                }
        "#;

        assert_expected(candidate, rename, input, expected);

        // testing reversal
        assert_expected(rename, candidate, expected, input);
    }

    #[test]
    fn test_3() {
        let candidate = "oldUser";
        let rename = "myNewUser";

        let input = r"
            camel: oldUser
            pascal: OldUser
            snake: old_user
            lower: olduser
            upper: OLDUSER
            noop: Olduser
            kebab: old-user
            upperkebab: OLD-USER
            uppersnake: OLD_USER
        ";
        let expected = r"
            camel: myNewUser
            pascal: MyNewUser
            snake: my_new_user
            lower: olduser
            upper: OLDUSER
            noop: Olduser
            kebab: my-new-user
            upperkebab: MY-NEW-USER
            uppersnake: MY_NEW_USER
        ";

        assert_expected(candidate, rename, input, expected);
    }

    #[test]
    fn test_preferred_casing() {
        assert_expected(
            "result",
            "parsedTransaction",
            "result | Result | RESULT | rEsult",
            "parsedTransaction | ParsedTransaction | PARSED_TRANSACTION | rEsult",
        );

        assert_expected(
            "result",
            "parsed-transaction",
            "result | Result | RESULT | rEsult",
            "parsed-transaction | ParsedTransaction | PARSED-TRANSACTION | rEsult",
        );

        assert_expected(
            "result",
            "parsed_transaction",
            "result | Result | RESULT | rEsult",
            "parsed_transaction | ParsedTransaction | PARSED_TRANSACTION | rEsult",
        );
    }
}
