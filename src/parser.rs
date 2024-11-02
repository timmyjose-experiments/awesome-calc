use std::num::ParseFloatError;

use crate::{
    ast::{Ast, Op},
    lexer::{Lexer, Token, TokenKind},
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("ran out of tokens while parsing")]
    NoMoreTokens,

    #[error("Parsing error: {0}")]
    Parse(String),

    #[error(transparent)]
    ParseFloatError(#[from] ParseFloatError),
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Parser {
    curr_tok: Option<Token>,
    lexer: Lexer,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Self {
            lexer,
            curr_tok: None,
        }
    }

    fn match_it(&mut self) {
        self.curr_tok = self.lexer.next();
    }

    fn match_kind(&mut self, expected_kind: TokenKind) -> Result<()> {
        if let Some(tok) = &self.curr_tok {
            if tok.kind != expected_kind {
                Err(Error::Parse(format!(
                    "Expected {:#?}, but found {:#?}",
                    expected_kind, tok.kind
                )))
            } else {
                Ok(())
            }
        } else {
            Err(Error::NoMoreTokens)
        }
    }

    fn parse_number(&mut self) -> Result<Ast> {
        if let Some(tok) = &self.curr_tok {
            let number = Ast::Number(tok.spelling.parse::<f64>()?);
            self.match_it();
            Ok(number)
        } else {
            Err(Error::NoMoreTokens)
        }
    }

    fn parse_operator(&mut self) -> Result<Op> {
        if let Some(tok) = &self.curr_tok {
            match tok.kind {
                TokenKind::Plus | TokenKind::Minus | TokenKind::Asterisk | TokenKind::Slash => {
                    let op = match tok.kind {
                        TokenKind::Plus => Op::Add,
                        TokenKind::Minus => Op::Sub,
                        TokenKind::Asterisk => Op::Mul,
                        TokenKind::Slash => Op::Div,
                        _ => unreachable!(),
                    };
                    self.match_it();

                    Ok(op)
                }
                _ => Err(Error::Parse(format!(
                    "Expected an operator, but found {:#?}",
                    tok.kind
                ))),
            }
        } else {
            Err(Error::NoMoreTokens)
        }
    }

    /// Primary <- Number | '(' E ')'
    fn parse_primary(&mut self) -> Result<Ast> {
        if let Some(tok) = &self.curr_tok {
            match tok.kind {
                TokenKind::Number => self.parse_number(),
                TokenKind::LeftParen => {
                    self.match_it();
                    let e = self.parse_expression()?;
                    self.match_kind(TokenKind::RightParen)?;
                    Ok(e)
                }
                _ => Err(Error::Parse(format!(
                    "Expected number or parenthesised expression. Found: {:#?}",
                    tok.kind
                ))),
            }
        } else {
            Err(Error::NoMoreTokens)
        }
    }

    /// F <- ('+' | '- ') Primary
    fn parse_factor(&mut self) -> Result<Ast> {
        if let Some(tok) = &self.curr_tok {
            match tok.kind {
                TokenKind::Plus | TokenKind::Minus => {
                    let op = self.parse_operator()?;
                    let t = self.parse_primary()?;

                    Ok(Ast::UnaryExpr {
                        op,
                        rhs: Box::new(t),
                    })
                }
                _ => self.parse_primary(),
            }
        } else {
            Err(Error::NoMoreTokens)
        }
    }

    /// T <- F (('*' / '/') F)*
    fn parse_term(&mut self) -> Result<Ast> {
        let mut f1 = self.parse_factor()?;

        while let Some(tok) = &self.curr_tok {
            match tok.kind {
                TokenKind::Asterisk | TokenKind::Slash => {
                    let op = self.parse_operator()?;
                    let f2 = self.parse_factor()?;

                    f1 = Ast::BinaryExpr {
                        lhs: Box::new(f1),
                        op,
                        rhs: Box::new(f2),
                    };
                }
                _ => break,
            }
        }
        Ok(f1)
    }

    /// E <- T (('+' | '-') T)*
    fn parse_expression(&mut self) -> Result<Ast> {
        let mut t1 = self.parse_term()?;

        while let Some(tok) = &self.curr_tok {
            match tok.kind {
                TokenKind::Plus | TokenKind::Minus => {
                    let op = self.parse_operator()?;
                    let t2 = self.parse_term()?;

                    t1 = Ast::BinaryExpr {
                        lhs: Box::new(t1),
                        op,
                        rhs: Box::new(t2),
                    };
                }
                _ => break,
            }
        }

        Ok(t1)
    }

    /// Grammar:
    ///
    /// E <- T (('+' | '-) T)*
    /// T <- F (('*' | '/') F)*
    /// F <- ('+' | '-') Primary
    /// Primary <- Number | '(' E ')'
    ///
    pub fn parse(&mut self) -> Result<Ast> {
        self.match_it();
        self.parse_expression()
    }
}
