use std::io::Read;

use crate::{casing::Casing, record::Record, token::Token};

#[derive(Debug)]
pub struct Task {
    candidate: Token,
    rename_to: Token,
    buf: String,
}

impl Task {
    pub fn new(mut input: impl Read, candidate: &str, rename_to: &str) -> Self {
        let mut buf = String::new();
        let _ = input.read_to_string(&mut buf);

        Self {
            candidate: candidate.parse().unwrap(),
            rename_to: rename_to.parse().unwrap(),
            buf,
        }
    }

    pub fn generate_records(&self) -> Vec<Record> {
        let mut records = vec![];

        let casings: Vec<_> = vec![
            Casing::LowerCase,
            Casing::PascalCase,
            Casing::SnakeCase,
            Casing::CamelCase,
        ];

        let matches: Vec<_> = casings
            .iter()
            .map(|casing| self.candidate.to_case(casing))
            .collect();

        for (casing, token_to_match) in casings.iter().zip(matches.iter()) {
            if token_to_match.is_none() {
                continue;
            }

            let token_to_match = token_to_match.as_ref().unwrap();

            let res: Vec<_> = self.buf.match_indices(token_to_match).collect();
            for item in res.iter() {
                records.push(Record {
                    pos: item.0,
                    len: token_to_match.len(),
                    case: casing.clone(),
                })
            }
        }

        records
    }

    pub fn process_records(&mut self, records: &mut [Record]) {
        // sort to make the offset work
        records.sort_by(|a, b| a.pos.cmp(&b.pos));

        let mut offset = 0;

        for record in records.iter() {
            let rename_to = self.rename_to.to_case(&record.case);
            if rename_to.is_none() {
                continue;
            }

            let rename_to = rename_to.unwrap();

            let start = record.pos + offset;
            let end = record.pos + record.len + offset;

            self.buf.replace_range(start..end, &rename_to);

            offset += rename_to.len() - record.len;
        }

        println!("{}", &self.buf);
    }
}
