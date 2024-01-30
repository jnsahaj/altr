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
    rename: Token,
}

impl Task {
    pub fn new(candidate: &str, rename: &str) -> Self {
        Self {
            candidate: candidate.parse().unwrap(),
            rename: rename.parse().unwrap(),
        }
    }

    pub fn generate_records(&mut self, reader: impl BufRead) -> Records {
        let mut records = Records::new();

        // collection of casings to operate on
        let casings: Vec<_> = vec![
            Casing::Lower,
            Casing::Pascal,
            Casing::Camel,
            Casing::Snake,
            Casing::Upper,
            Casing::UpperSnake,
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
            .filter(|(_, p)| p.is_ok())
            .map(|(c, p)| (c, p.as_ref().unwrap()))
            .collect();

        let mut line_offset: usize = 0;

        for line in reader.lines() {
            let line = line.unwrap();

            for (casing, pattern) in casing_with_candidates.iter() {
                let matches = line.match_indices(*pattern);

                for item in matches {
                    let _ =
                        records.try_insert(item.0 + line_offset, pattern.len(), (**casing).clone());
                }
            }

            line_offset += line.len() + 1; // + 1 accounts for the \n character
        }

        records
    }

    pub fn process_records(&mut self, records: &mut Records, reader: &mut impl BufRead) -> String {
        let mut buf = String::new();
        let _ = reader.read_to_string(&mut buf);

        dbg!(&records);

        let mut offset = Offset::Pos(0);

        for (_, record) in records.iter() {
            let rename =
                self.rename
                    .try_to_casing(&record.casing)
                    .unwrap_or_else(|err| match err {
                        TokenError::AmbiguousToLowerCase => self.rename.to_camel_case(),
                        TokenError::AmbiguousToUpperCase => self.rename.to_upper_snake_case(),
                    });

            let start = offset.apply(record.pos);
            let end = offset.apply(record.pos + record.len);

            buf.replace_range(start..end, &rename);

            offset = Offset::add(offset, Offset::from_diff(rename.len(), record.len));
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
        Cursor::new(Vec::with_capacity(capacity))
    }

    fn task_test_creator<'a>(candidate: &'a str, rename: &'a str) -> impl FnOnce(&'a str, &'a str) {
        let mut task = Task::new(candidate, rename);

        let f = move |input: &str, expected: &str| {
            let mut records = task.generate_records(get_reader(input));
            let result = task.process_records(&mut records, &mut get_reader(input));

            let mut writer = get_writer(result.len());
            Task::write(&mut writer, &result);

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
        let rename = "supplyUser";

        let input = r"user User USER";
        let expected = r"supplyUser SupplyUser SUPPLY_USER";

        let assert = task_test_creator(candidate, rename);
        assert(input, expected);
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

        let assert = task_test_creator(candidate, rename);
        assert(input, expected);

        let reverse_assert = task_test_creator(rename, candidate);
        reverse_assert(expected, input);
    }
}
