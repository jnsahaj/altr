use std::io::{Cursor, Read};

use casing::Casing;
use record::Record;

use crate::token::Token;

mod casing;
mod record;
mod token;

const SEPARATOR: char = ',';

#[derive(Debug)]
pub struct Program<T: Read> {
    candidate: String,
    rename_to: String,
    input: T,
    records: Vec<Record>,
}

impl<T: Read> Program<T> {
    fn new(input: T) -> Self {
        Self {
            candidate: "user".into(),
            rename_to: "supplyUser".into(),
            input,
            records: vec![],
        }
    }

    fn process_records(&mut self) {
        let mut buf = String::new();
        self.input.read_to_string(&mut buf);

        let mut tok = Token::from_camel_case(&self.candidate).unwrap();

        let matches = vec![
            (Casing::CamelCase, tok.to_camel_case()),
            (Casing::PascalCase, tok.to_pascal_case()),
            (Casing::SnakeCase, tok.to_snake_case()),
        ];

        for (case, m) in matches {
            let res: Vec<_> = buf.match_indices(&m).collect();
            for item in res.iter() {
                self.records.push(Record {
                    pos: item.0,
                    len: self.candidate.len(),
                    case: case.clone(),
                })
            }
        }

        dbg!(&self.records);
    }
}

fn main() {
    let candidate = "user";
    let rename_to = "supplyUser";

    let check = r#"
        const [user, setUser] = useState(undefined);
        
        function getUserName(user: IUser) {
            return user.name;
        }
    "#;

    let input_token = Token::from_camel_case(candidate);
    let rename_token = Token::from_snake_case(rename_to);

    let mut program = Program::new(Cursor::new(check));
    program.process_records();

    // println!("{}", token.unwrap().to_camel_case());

    // println!(
    //     "{}",
    //     compute(&input_token.unwrap(), &rename_token.unwrap(), check)
    // );
}

fn compute(input_token: &Token, rename_token: &Token, check: &str) -> String {
    dbg!(&input_token.to_pascal_case());
    check
        .replace(&input_token.to_camel_case(), &rename_token.to_camel_case())
        // .replace(&input_token.to_snake_case(), &rename_token.to_snake_case())
        .replace(
            &input_token.to_pascal_case(),
            &rename_token.to_pascal_case(),
        )
}
