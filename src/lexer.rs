use regex::{Regex, RegexSet};
use std::str::FromStr;

/// Represents the type of token our lexer produces
///
/// For information on what they represent syntactically, look at the code
/// they generate.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    OpenParens,
    CloseParens,
    Semicolon,
    Func,
    StringLitteral { value: String },
    IntLitteral { value: i32 },
    Name { value: String },
}

/// Handle escape sequences when processing a litteral string.
pub fn process_string_litteral(input: &str) -> String {
    let mut acc = String::new();
    let mut escaping = false;
    for c in input.chars() {
        let was_escaping = escaping;
        match (escaping, c) {
            // This will only be the first and last
            (_, '"') => {}
            (true, 'n') => acc.push_str("\n"),
            (true, '\\') => acc.push('\\'),
            (true, 'r') => acc.push('\r'),
            (true, 't') => acc.push('\t'),
            (true, c) => {
                acc.push('\\');
                acc.push(c)
            }
            (false, '\\') => escaping = true,
            (false, c) => acc.push(c),
        }
        if was_escaping {
            escaping = false;
        }
    }
    acc
}

/// Represents the type of error that can happen while lexing.
///
/// Right now, this is empty.
#[derive(Debug, Clone, PartialEq)]
pub struct LexError {
    message: String,
}

/// Represents a location inside some piece of text
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Location(usize);

/// This is what our lexer produces
pub type Span = Result<(Location, Token, Location), LexError>;

const SIMPLE_MATCH_STRINGS: [&str; 6] = [r"^\{", r"^\}", r"^\(", r"^\)", r"^;", r"func"];
const SIMPLE_MATCH_LENGTHS: [usize; 6] = [1, 1, 1, 1, 1, 4];
const SIMPLE_MATCH_TOKENS: [Token; 6] = [
    Token::OpenBrace,
    Token::CloseBrace,
    Token::OpenParens,
    Token::CloseParens,
    Token::Semicolon,
    Token::Func,
];

pub struct Lexer<'d> {
    data: &'d str,
    pos: usize,
    simple_matchers: RegexSet,
    whitespace_matcher: Regex,
    string_litteral_matcher: Regex,
    int_litteral_matcher: Regex,
    name_matcher: Regex,
}

impl<'d> Lexer<'d> {
    pub fn new(data: &'d str) -> Lexer {
        let simple_matchers = RegexSet::new(&SIMPLE_MATCH_STRINGS).unwrap();
        let whitespace_matcher = Regex::new(r"^\s+").unwrap();
        let int_litteral_matcher = Regex::new(r"^-?[0-9]+").unwrap();
        let string_litteral_matcher = Regex::new(r#"^"([^"]*)""#).unwrap();
        let name_matcher = Regex::new(r"^[a-z]\w*").unwrap();
        Lexer {
            data,
            pos: 0,
            simple_matchers,
            whitespace_matcher,
            string_litteral_matcher,
            int_litteral_matcher,
            name_matcher,
        }
    }
}

impl<'d> Iterator for Lexer<'d> {
    type Item = Span;

    fn next(&mut self) -> Option<Span> {
        while let Some(mat) = self.whitespace_matcher.find(&self.data[self.pos..]) {
            self.pos += mat.end() - mat.start();
        }
        if self.pos >= self.data.len() {
            return None;
        }
        let current_data = &self.data[self.pos..];
        if let Some(first) = self.simple_matchers.matches(current_data).iter().next() {
            let matched_token = SIMPLE_MATCH_TOKENS[first].clone();
            let start = Location(self.pos);
            self.pos += SIMPLE_MATCH_LENGTHS[first];
            let end = Location(self.pos);
            return Some(Ok((start, matched_token, end)));
        }
        if let Some(mat) = self.name_matcher.find(current_data) {
            let matched_string = mat.as_str();
            let matched_token = Token::Name {
                value: String::from(matched_string),
            };
            let start = Location(self.pos);
            self.pos += matched_string.len();
            let end = Location(self.pos);
            return Some(Ok((start, matched_token, end)));
        }
        if let Some(caps) = self.string_litteral_matcher.captures(current_data) {
            let matched_string = caps.get(1).unwrap().as_str();
            let matched_token = Token::StringLitteral {
                value: process_string_litteral(matched_string),
            };
            let start = Location(self.pos);
            let total_match = caps.get(0).unwrap();
            self.pos += total_match.end() - total_match.start();
            let end = Location(self.pos);
            return Some(Ok((start, matched_token, end)));
        }
        if let Some(mat) = self.int_litteral_matcher.find(current_data) {
            let matched_string = mat.as_str();
            let value = i32::from_str(matched_string).unwrap();
            let matched_token = Token::IntLitteral { value };
            let start = Location(self.pos);
            self.pos += matched_string.len();
            let end = Location(self.pos);
            return Some(Ok((start, matched_token, end)));
        }
        let message = format!("Unrecognized characters at position {}", self.pos);
        // Since nothing matched, we have to skip to the end
        self.pos += current_data.len();
        Some(Err(LexError { message }))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn unicode_names_can_be_lexed() {
        let a_cat = "a猫";
        let mut lexer = Lexer::new(a_cat);
        let token = Token::Name {
            value: String::from(a_cat),
        };
        let span = (Location(0), token, Location(a_cat.len()));
        assert_eq!(lexer.next(), Some(Ok(span)));
    }

    #[test]
    fn spaces_are_skipped_when_lexing() {
        let input = "func main";
        let lexer = Lexer::new(input);
        let result: Vec<Span> = lexer.collect();
        let spans = vec![
            Ok((Location(0), Token::Func, Location(4))),
            Ok((
                Location(5),
                Token::Name {
                    value: String::from("main"),
                },
                Location(9),
            )),
        ];
        assert_eq!(result, spans);
    }

    #[test]
    fn string_litterals_lex() {
        let input = r#""\n""#;
        let lexer = Lexer::new(input);
        let result: Vec<Span> = lexer.collect();
        let spans = vec![Ok((
            Location(0),
            Token::StringLitteral {
                value: String::from("\n"),
            },
            Location(4),
        ))];
        assert_eq!(result, spans);
    }
}
