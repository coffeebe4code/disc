#![allow(unused_parens)]
#![feature(iter_advance_by)]

use core::panic;

#[derive(Debug)]
pub struct ASTError(String);

#[derive(Debug, Clone)]
pub struct AST {}

#[derive(Debug, PartialEq)]
pub enum TOKEN {
    Type(String),
    Declaration(String),
    Property(String),
    Builtin(String),
    Pre(String),
    Interface(String),
    CParen,
    OParen,
    Comment,
    Quoted(String),
    Float(f64),
    Array(Vec<TOKEN>),
    Number(u64),
    Words(String),
}

impl TOKEN {
    fn from_u32(val: u32, data: &str) -> TOKEN {
        match val {
            0 => TOKEN::Type(data.to_string()),
            1 => TOKEN::Declaration(data.to_string()),
            2 => TOKEN::Property(data.to_string()),
            3 => TOKEN::Builtin(data.to_string()),
            4 => TOKEN::Pre(data.to_string()),
            5 => TOKEN::Interface(data.to_string()),
            6 => TOKEN::Words(data.to_string()),
            _ => panic!("invalid TOKEN supplied"),
        }
    }
}

fn seek_past_newline(data: &str) -> usize {
    let mut index = 0;
    let mut found = false;
    for c in data.chars() {
        match c {
            ' ' => {}
            '\n' => found = true,
            '\t' => {}
            _ => {
                if (found == true) {
                    break;
                }
            }
        }
        index += 1;
    }
    return index;
}

fn parse_number(data: &str) -> Result<(TOKEN, usize), ASTError> {
    let index = 0;
    let invalid = false;
    for c in data.chars() {
        match c {
            ' ' => {}
            '\n' => {}
            '\t' => {}
            'x' => {}
            'b' => {}
            'f' => {}
            'l' => {}
            _ => {}
        }
    }
    Ok((TOKEN::Number(5), 1))
}

fn ensure_word_to_end(data: &str) -> Result<usize, ASTError> {
    let mut index = 0;
    let mut found = false;
    for c in data.chars() {
        match c {
            ' ' => found = true,
            '\n' => found = true,
            '\t' => found = true,
            ')' => found = true,
            c if c.is_alphabetic() => {
                index += 1;
            }
            c if c.is_digit(10) => {
                index += 1;
            }
            '-' => {
                index += 1;
            }
            '_' => index += 1,
            _ => {
                return Err((ASTError(format!("invalid character in word {}", c))));
            }
        }
        if (found) {
            break;
        }
    }
    return Ok(index);
}

fn seek_past_whitespace(data: &str) -> usize {
    let mut index = 0;
    let mut found = false;
    for c in data.chars() {
        match c {
            ' ' => {
                index -= 1;
                found = true
            }
            '\n' => {
                index -= 1;
                found = true
            }
            '\t' => {
                index -= 1;
                found = true
            }
            _ => {
                if (found == true) {
                    break;
                }
                index += 1;
            }
        }
    }
    return index;
}

fn make_ident_or_error(data: &str, ident: u32) -> Result<(TOKEN, usize), ASTError> {
    let skip = ensure_word_to_end(&data[0..]);
    match skip {
        Ok(val) => {
            if (val == 0) {
                return Err(ASTError("expected a word".to_string()));
            }
            return Ok((TOKEN::from_u32(ident, &data[0..val]), val));
        }
        Err(val) => return Err(val),
    }
}

pub fn parse_quoted(data: &str) -> Result<(TOKEN, usize), ASTError> {
    let mut escape = false;
    let mut closed = false;
    let mut new_data: String = "".to_string();
    let mut index = 0;
    for c in data.chars() {
        match c {
            '\\' => {
                escape = true;
                index += 1
            }
            '"' => {
                if (!escape) {
                    closed = true;
                    index += 1;
                    break;
                }
                new_data.push('\"');
                index += 1;
            }
            _ => {
                if (escape) {
                    match c {
                        'n' => {
                            new_data.push('\n');
                        }
                        't' => {
                            new_data.push('\t');
                        }
                        '\\' => {
                            new_data.push('\\');
                        }
                        'r' => {
                            new_data.push('\r');
                        }
                        '0' => {
                            new_data.push('\0');
                        }
                        'x' => {
                            new_data.push('\x10');
                        }
                        'u' => {
                            new_data.push('\u{0010}');
                        }
                        _ => {
                            return Err(ASTError(format!("invalid escape character {}", c)));
                        }
                    }
                } else {
                    new_data.push(c);
                }
                index += 1;
            }
        }
    }
    if (!closed) {
        return Err(ASTError("expected closing \"".to_string()));
    }
    return Ok((TOKEN::Quoted(new_data), index - 1));
}

pub fn parse_array(data: &str) -> Result<(TOKEN, usize), ASTError> {
    let mut vec: Vec<TOKEN> = vec![];
    let mut closed = false;
    let mut index = 0;
    let mut iter = data.chars();
    loop {
        match iter.next() {
            None => {
                return Err(ASTError(format!("expected closing ]").to_string()));
            }
            Some(']') => {
                closed = true;
                index += 1;
                break;
            }
            Some('"') => {
                let quote_or = parse_quoted(&data[index..]);
                match quote_or {
                    Err(val) => return Err(val),
                    Ok(val) => {
                        vec.push(val.0);
                        iter.advance_by(val.1);
                        index += val.1;
                    }
                }
            }
            Some(val) => {}
        }
    }
    if (!closed) {
        return Err(ASTError(format!("expected closing ]").to_string()));
    }
    return Ok((TOKEN::Array(vec), index - 1));
}

pub fn tokenize(data: &str) -> Result<(TOKEN, usize), ASTError> {
    let curr = data.chars().next().unwrap();
    match curr {
        ';' => Ok((TOKEN::Comment, seek_past_newline(data))),
        ':' => make_ident_or_error(&data[1..], 2),
        ',' => make_ident_or_error(&data[1..], 5),
        ')' => Ok((TOKEN::CParen, 1)),
        '(' => Ok((TOKEN::OParen, 1)),
        '\'' => make_ident_or_error(&data[1..], 0),
        '"' => parse_quoted(&data[1..]),
        '@' => make_ident_or_error(&data[1..], 1),
        '%' => make_ident_or_error(&data[1..], 3),
        '#' => make_ident_or_error(&data[1..], 4),
        '[' => parse_array(&data[1..]),
        '\t' => Ok((TOKEN::Comment, seek_past_whitespace(&data[1..]))),
        '\n' => Ok((TOKEN::Comment, seek_past_whitespace(&data[1..]))),
        ' ' => Ok((TOKEN::Comment, seek_past_whitespace(&data[1..]))),
        c if c.is_alphabetic() => make_ident_or_error(&data[..], 6),
        c if c.is_digit(10) => parse_number(&data[..]),
        _ => Err(ASTError(format!("invalid token: {}", curr))),
    }
}

#[cfg(test)]
mod tests {
    use crate::{tokenize, TOKEN};
    #[test]
    fn tokenizes_comments() {
        assert_eq!(tokenize("; hello\n  \t(").unwrap().1, 11);
        assert_eq!(tokenize(";").unwrap().1, 1);
    }
    #[test]
    fn tokenizes_properties() {
        assert_eq!(tokenize(":prop ()").unwrap().1, 4);
        assert_eq!(
            tokenize(":prop ()").unwrap().0,
            TOKEN::Property("prop".to_string())
        );
        assert_eq!(tokenize(":").unwrap_err().0, "expected a word".to_string());
        assert_eq!(
            tokenize(":prop)").unwrap().0,
            TOKEN::Property("prop".to_string())
        );
    }
    #[test]
    fn tokenizes_types() {
        assert_eq!(
            tokenize("'string \"hello\"").unwrap().0,
            TOKEN::Type("string".to_string())
        );
        assert_eq!(tokenize("'string").unwrap().1, 6);
        assert_eq!(tokenize("'").unwrap_err().0, "expected a word".to_string());
    }
    #[test]
    fn tokenizes_scopes() {
        assert_eq!(tokenize("()").unwrap().0, TOKEN::OParen);
        assert_eq!(tokenize(")").unwrap().0, TOKEN::CParen);
    }
    #[test]
    fn tokenizes_builtins() {
        assert_eq!(tokenize("%ffi )").unwrap().1, 3);
        assert_eq!(
            tokenize("%main ").unwrap().0,
            TOKEN::Builtin("main".to_string())
        );
    }
    #[test]
    fn tokenizes_arrays() {
        // assert_eq!(tokenize("[5 6 7 9]"));
    }
    #[test]
    fn tokenizes_strings() {
        assert_eq!(tokenize("\"string\"").unwrap().1, 6);
        assert_eq!(
            tokenize("\"string\"").unwrap().0,
            TOKEN::Quoted("string".to_string())
        );
        assert_eq!(
            tokenize("\"string").unwrap_err().0,
            "expected closing \"".to_string()
        );
    }
    #[test]
    fn tokenizes_numbers() {}
    #[test]
    fn tokenizes_pres() {
        assert_eq!(tokenize("#static").unwrap().1, 6);
        assert_eq!(
            tokenize("#inline ").unwrap().0,
            TOKEN::Pre("inline".to_string())
        );
    }
    #[test]
    fn tokenizes_words() {
        assert_eq!(tokenize("word").unwrap().1, 4);
        assert_eq!(
            tokenize("works ()").unwrap().0,
            TOKEN::Words("works".to_string())
        );
        assert_eq!(
            tokenize("works)").unwrap().0,
            TOKEN::Words("works".to_string())
        );
    }
    #[test]
    fn tokenizes_interfaces() {
        assert_eq!(
            tokenize(",debug-it").unwrap().0,
            TOKEN::Interface("debug-it".to_string())
        );
        assert_eq!(tokenize(",d").unwrap().1, 1);
    }
}
