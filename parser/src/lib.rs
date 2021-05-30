#![allow(unused_parens)]

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
    Array(String),
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
    let mut index = 0;
    let curr = data.chars().next().unwrap();
    Ok((TOKEN::Number(5), 1))
}

fn parse_word(data: &str) -> Result<(TOKEN, usize), ASTError> {
    let mut index = 0;
    for c in data.chars() {
        match c {
            c if c.is_alphabetic() => index += 1,
            c if c.is_digit(10) => index += 1,
            ' ' => {
                break;
            }
            '-' => index += 1,
            '_' => index += 1,
            _ => return Err(ASTError(format!("error illicit character found {}", c))),
        }
    }
    return Ok((TOKEN::Words(data[0..index].to_string()), index));
}
// ha -> 0,
fn ensure_word_to_end(data: &str) -> Result<usize, ASTError> {
    let mut index = 0;
    let mut found = false;
    for c in data.chars() {
        match c {
            ' ' => {
                found = true
            }
            '\n' => {
                found = true
            }
            '\t' => {
                found = true
            }
            ')' => {
                found = true
            }
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
                return Err((ASTError("errorbreak".to_string())));
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
                return Err(ASTError("expected".to_string()));
            }
            return Ok((TOKEN::from_u32(ident, &data[0..val]), val));
        }
        Err(val) => return Err(val),
    }
}

//pub fn parse_to_ast(data: &str, pos: u32, line: u32) -> (Vec<AST>, Vec<ASTError) {
//
//}

pub fn tokenize(data: &str) -> Result<(TOKEN, usize), ASTError> {
    let curr = data.chars().next().unwrap();
    match curr {
        ';' => Ok((TOKEN::Comment, seek_past_newline(data))),
        ':' => make_ident_or_error(&data[1..], 2),
        ',' => make_ident_or_error(&data[1..], 5),
        ')' => Ok((TOKEN::CParen, 1)),
        '(' => Ok((TOKEN::OParen, 1)),
        '\'' => make_ident_or_error(&data[1..], 0),
        '"' => Ok((TOKEN::Quoted("quoted".to_string()), 6)),
        '@' => make_ident_or_error(&data[1..], 1),
        '%' => make_ident_or_error(&data[1..], 3),
        '#' => make_ident_or_error(&data[1..], 4),
        '[' => Ok((TOKEN::Array("array".to_string()), 5)),
        '\t' => Ok((TOKEN::Comment, seek_past_whitespace(&data[1..]))),
        '\n' => Ok((TOKEN::Comment, seek_past_whitespace(&data[1..]))),
        ' ' => Ok((TOKEN::Comment, seek_past_whitespace(&data[1..]))),
        _ => {
            if (curr.is_alphabetic()) {
                return parse_word(&data[..]);
            } else if (curr.is_digit(10)) {
                return parse_number(&data[..]);
            }
            Err(ASTError("invalid token".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{tokenize, AST, TOKEN};
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
        assert_eq!(tokenize(":").unwrap_err().0, "expected".to_string());
    }
    #[test]
    fn tokenizes_types() {
        assert_eq!(
            tokenize("'string \"hello\"").unwrap().0,
            TOKEN::Type("string".to_string())
        );
        assert_eq!(tokenize("'string").unwrap().1, 6);
        assert_eq!(tokenize("'").unwrap_err().0, "expected".to_string());
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
    fn tokenizes_arrays() {}
    #[test]
    fn tokenizes_strings() {}

    #[test]
    fn tokenizes_numbers() {}
    #[test]
    fn tokenizes_pres() {}
    #[test]
    fn tokenizes_words() {
        assert_eq!(tokenize("word").unwrap().1, 4);
        assert_eq!(
            tokenize("works ()").unwrap().0,
            TOKEN::Words("works".to_string())
        );
    }
    #[test]
    fn tokenizes_interfaces() {}
}
