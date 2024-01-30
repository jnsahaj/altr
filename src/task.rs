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

    pub fn process_records(
        &mut self,
        records: &mut Records,
        mut reader: impl BufRead,
        mut writer: impl Write,
    ) {
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
                });

            let start = offset.apply(record.pos);
            let end = offset.apply(record.pos + record.len);

            buf.replace_range(start..end, &rename_to);

            offset = Offset::add(offset, Offset::from_diff(rename_to.len(), record.len));
        }

        println!("{}", &buf);

        writer.write_all(buf.as_bytes()).expect("Failed to write");
    }
}
