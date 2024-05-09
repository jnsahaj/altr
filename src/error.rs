use derive_more::{Display, From};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From, Display)]
pub enum Error {
    #[display(
        fmt = "{}",
        r#"if _0.is_empty() { "Failed to detect casing for candidate term" } else { _0 } "#
    )]
    CandidateCasing(String),

    #[display(
        fmt = "{}",
        r#"if _0.is_empty() { "Failed to detect casing for rename term" } else { _0 } "#
    )]
    RenameCasing(String),

    #[from]
    #[display(
        fmt = "{}",
        r#"if _0.is_empty() { "An unexpectd error occurred" } else { _0 } "#
    )]
    Custom(String),

    #[from]
    #[display(fmt = "{}", "_0")]
    Io(std::io::Error),
}
