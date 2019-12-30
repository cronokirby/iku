use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum Token {
    CloseBrace,
    CloseParens,
    Func,
    Name(String),
    OpenBrace,
    OpenParens,
}

#[derive(Debug, PartialEq)]
pub enum LexError {
    UnexpectedChar(char),
}

pub type LexResult = Result<Token, LexError>;

struct Lexer<I: Iterator<Item = char>> {
    chars: Peekable<I>,
}

impl<I: Iterator<Item = char>> Lexer<I> {
    fn new(iterator: I) -> Self {
        Lexer {
            chars: iterator.peekable(),
        }
    }

    fn name(&mut self) -> String {
        let mut acc = String::new();
        while let Some(c) = self.chars.peek() {
            if !c.is_alphanumeric() {
                break;
            }
            // Safe since we peeked
            acc.push(self.chars.next().unwrap())
        }
        acc
    }

    fn line_comment(&mut self) {
        while let Some(c) = self.chars.next() {
            if c == '\n' {
                break;
            }
        }
    }
}

impl<I: Iterator<Item = char>> Iterator for Lexer<I> {
    type Item = LexResult;

    fn next(&mut self) -> Option<LexResult> {
        while let Some(c) = self.chars.peek() {
            match c {
                '{' => {
                    self.chars.next();
                    return Some(Ok(Token::OpenBrace));
                }
                '}' => {
                    self.chars.next();
                    return Some(Ok(Token::CloseBrace));
                }
                '(' => {
                    self.chars.next();
                    return Some(Ok(Token::OpenParens));
                }
                ')' => {
                    self.chars.next();
                    return Some(Ok(Token::CloseParens));
                }
                '/' => {
                    self.chars.next();
                    if let Some('/') = self.chars.peek() {
                        self.line_comment();
                    } else {
                        return Some(Err(LexError::UnexpectedChar('/')));
                    }
                }
                w if w.is_whitespace() => {
                    self.chars.next();
                }
                x if !x.is_uppercase() => {
                    let name = self.name();
                    let out = match name.as_ref() {
                        "func" => Token::Func,
                        _ => Token::Name(name),
                    };
                    return Some(Ok(out));
                }
                u => return Some(Err(LexError::UnexpectedChar(*u))),
            }
        }
        None
    }
}

pub fn lex<I: IntoIterator<Item = char>>(chars: I) -> impl Iterator<Item = LexResult> {
    Lexer::new(chars.into_iter())
}
