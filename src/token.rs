#[derive(Debug, PartialEq, Eq)]

pub enum Token {
    Semicolon,
    NotEq,
    While,
    Do,
    End,
    Zero,
    One,
    Plus,
    Minus,
    Var(u64),
    Assign,
    Eof,
}
