use super::{evaluator, lexer, parser};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Lexer(#[from] lexer::Error),

    #[error(transparent)]
    Parser(#[from] parser::Error),

    #[error(transparent)]
    Eval(#[from] evaluator::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
