use std::{
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
};

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
    file: File,
}

impl Task {
    pub fn new(file: File, candidate: &str, rename_to: &str) -> Self {
        Self {
            candidate: candidate.parse().unwrap(),
            rename_to: rename_to.parse().unwrap(),
            file,
        }
    }

    pub fn generate_records(&mut self) -> Records {
        let mut records = Records::new();

        let mut buf = String::new();
        let _ = self.file.read_to_string(&mut buf);
        let _ = self.file.seek(SeekFrom::Start(0));

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

    pub fn process_records(&mut self, records: &mut Records) {
        let mut buf = String::new();
        let _ = self.file.read_to_string(&mut buf);

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

        self.file.set_len(0).unwrap();
        self.file.rewind().unwrap();
        self.file
            .write_all(buf.as_bytes())
            .expect("Failed to write");
    }
}
