use std::io::{BufRead, Write};

use crate::{
    casing::Casing,
    record::Records,
    task::offset::Offset,
    token::{Token, TokenError},
};

mod offset;

#[derive(Debug)]
pub struct Task {
    candidate: Token,
    rename_to: Token,
}

impl Task {
    pub fn new(candidate: &str, rename_to: &str) -> Self {
        Self {
            candidate: candidate.parse().unwrap(),
            rename_to: rename_to.parse().unwrap(),
        }
    }

    pub fn generate_records(&mut self, mut reader: impl BufRead) -> Records {
        let mut records = Records::new();

        let mut buf = String::new();
        let _ = reader.read_to_string(&mut buf);

        let casings: Vec<_> = vec![
            Casing::LowerCase,
            Casing::PascalCase,
            Casing::CamelCase,
            Casing::SnakeCase,
            Casing::UpperCase,
            Casing::UpperSnakeCase,
        ];

        let matches: Vec<_> = casings
            .iter()
            .map(|casing| self.candidate.try_to_casing(casing))
            .collect();

        for (casing, token_to_match) in casings.iter().zip(matches.iter()) {
            if token_to_match.is_err() {
                continue;
            }

            let token_to_match = token_to_match.as_ref().unwrap();

            let res: Vec<_> = buf.match_indices(token_to_match).collect();
            for item in res.iter() {
                let _ = records.try_insert(item.0, token_to_match.len(), casing.clone());
            }
        }

        records
    }

    pub fn process_records(&mut self, records: &mut Records, reader: &mut impl BufRead) -> String {
        let mut buf = String::new();
        let _ = reader.read_to_string(&mut buf);

        dbg!(&records);

        let mut offset = Offset::Pos(0);

        for (_, record) in records.iter() {
            let rename_to = self
                .rename_to
                .try_to_casing(&record.casing)
                .unwrap_or_else(|err| match err {
                    TokenError::AmbiguousToLowerCase => self.rename_to.to_camel_case(),
                    TokenError::AmbiguousToUpperCase => self.rename_to.to_upper_snake_case(),
                });

            let start = offset.apply(record.pos);
            let end = offset.apply(record.pos + record.len);

            buf.replace_range(start..end, &rename_to);

            offset = Offset::add(offset, Offset::from_diff(rename_to.len(), record.len));
        }

        buf
    }

    pub fn write(writer: &mut impl Write, value: &str) {
        writer.write_all(value.as_bytes()).expect("Failed to write");
    }
}

#[cfg(test)]
mod test_task {
    use std::io::{BufReader, Cursor, Read};

    use super::*;

    fn get_reader(input: &str) -> impl BufRead {
        let cursor: Cursor<Vec<u8>> = Cursor::new(input.into());
        BufReader::new(cursor)
    }

    fn get_writer(capacity: usize) -> Cursor<Vec<u8>> {
        let cursor = Cursor::new("".into());
        cursor
    }

    fn task_test_creator<'a>(
        candidate: &'a str,
        rename_to: &'a str,
    ) -> impl FnOnce(&'a str, &'a str) {
        let mut task = Task::new(candidate, rename_to);

        let f = move |input: &str, expected: &str| {
            let mut records = task.generate_records(get_reader(input));
            let result = task.process_records(&mut records, &mut get_reader(input));

            let mut writer = get_writer(result.len());
            let _ = Task::write(&mut writer, &result);

            let s = writer
                .get_ref()
                .bytes()
                .map(|v| v.unwrap())
                .collect::<Vec<u8>>();

            assert_eq!(
                s,
                expected.as_bytes(),
                "Result: {}",
                String::from_utf8_lossy(&s)
            );
        };

        f
    }

    #[test]
    fn test_1() {
        let candidate = "user";
        let rename_to = "supplyUser";

        let input = r"user User USER";
        let expected = r"supplyUser SupplyUser SUPPLY_USER";

        let assert = task_test_creator(candidate, rename_to);
        assert(input, expected);
    }

    #[test]
    fn test_2() {
        let candidate = "user";
        let rename_to = "dayTrader";

        let input = r#"
                const [user, setUser] = useState("");

                function getUserName() {
                    return user.name;
                }
                
                function setUserName(name: IUserName) {
                    // could also have been user
                    user.name = name;
                }
        "#;

        let expected = r#"
                const [dayTrader, setDayTrader] = useState("");

                function getDayTraderName() {
                    return dayTrader.name;
                }
                
                function setDayTraderName(name: IDayTraderName) {
                    // could also have been dayTrader
                    dayTrader.name = name;
                }
        "#;

        let assert = task_test_creator(candidate, rename_to);
        assert(&input, &expected);

        let reverse_assert = task_test_creator(rename_to, candidate);
        reverse_assert(&expected, &input);
    }
}
