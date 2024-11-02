#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("ran out of characters while lexing")]
    NoCharactersToEat,

    #[error("Invalid character: '{0}'")]
    InvalidCharacter(char),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum TokenKind {
    Asterisk,
    Eof,
    LeftParen,
    Minus,
    Number,
    Plus,
    RightParen,
    Slash,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Token {
    pub kind: TokenKind,
    pub spelling: String,
}

pub struct Lexer {
    s: String,
    curr_char: Option<char>,
    curr_idx: usize,
    curr_spelling: String,
}

impl Lexer {
    pub fn new(s: String) -> Self {
        let curr_idx = 0;
        let curr_char = s.chars().nth(0);

        Self {
            s,
            curr_char,
            curr_idx,
            curr_spelling: String::new(),
        }
    }

    fn skip_it(&mut self) {
        self.curr_idx += 1;
        self.curr_char = self.s.chars().nth(self.curr_idx);
    }

    fn eat_it(&mut self) -> Result<()> {
        self.curr_spelling
            .push(self.curr_char.ok_or_else(|| Error::NoCharactersToEat)?);
        self.curr_idx += 1;
        self.curr_char = self.s.chars().nth(self.curr_idx);

        Ok(())
    }

    fn lex_it(&mut self) -> Result<TokenKind> {
        let mut tok_kind = TokenKind::Eof;

        match self.curr_char {
            Some(c) if c.is_numeric() => {
                while self.curr_char.is_some_and(|c| c.is_numeric() || c == '.') {
                    self.eat_it()?;
                }
                tok_kind = TokenKind::Number;
            }

            Some('(') => {
                self.eat_it()?;
                tok_kind = TokenKind::LeftParen;
            }

            Some(')') => {
                self.eat_it()?;
                tok_kind = TokenKind::RightParen;
            }

            Some('+') => {
                self.eat_it()?;
                tok_kind = TokenKind::Plus;
            }

            Some('-') => {
                self.eat_it()?;
                tok_kind = TokenKind::Minus;
            }

            Some('*') => {
                self.eat_it()?;
                tok_kind = TokenKind::Asterisk;
            }

            Some('/') => {
                self.eat_it()?;
                tok_kind = TokenKind::Slash;
            }

            Some(c) => Err(Error::InvalidCharacter(c))?,
            None => tok_kind = TokenKind::Eof,
        }

        Ok(tok_kind)
    }

    pub fn lex(&mut self) -> Result<Token> {
        while self.curr_char.is_some_and(|c| c.is_ascii_whitespace()) {
            self.skip_it();
        }

        self.curr_spelling.clear();
        let tok_kind = self.lex_it()?;

        Ok(Token {
            kind: tok_kind,
            spelling: self.curr_spelling.to_owned(),
        })
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lex() {
            Ok(tok) => {
                if tok.kind == TokenKind::Eof {
                    None
                } else {
                    Some(tok)
                }
            }

            Err(err) => {
                eprintln!("{err}");
                None
            }
        }
    }
}
