use super::{evaluator, lexer, parser};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    LexerError(#[from] lexer::Error),
    #[error(transparent)]
    ParserError(#[from] parser::Error),
    #[error(transparent)]
    EvalError(#[from] evaluator::Error),
}
