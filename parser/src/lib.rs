#![allow(unused_parens)]

#[derive(Debug)]
pub struct ASTError(String);

#[derive(Debug,Clone)]
pub struct AST {
    
}

pub enum TOKEN {
    OParen,
    CParen,
    Comment,
    Type(String),
    Declaration(String),
    Property(String),
    Quoted(String),
    Builtin(String),
    Pre(String),
    Number(String),
    Words(String),
    Interface(String),
}

pub fn seek_past_newline(data: &str) -> usize {
    let mut index = 0;
    let mut found = false;
    for c in data.chars() {
        match c {
            ' ' => { },
            '\n' => { found = true },
            '\t' => { },
            _ => { if(found == true) { break; } }
        }
        index += 1;
    }
    return index;
}

pub fn seek_past_whitespace(data: &str) -> usize {
    let mut index = 0;
    let mut found = false;
    for c in data.chars() {
        match c {
            ' ' => { found = true},
            '\n' => { found = true },
            '\t' => { found = true },
            _ => { if(found == true) { break; } }
        }
        index+=1;
    }
    return index;
}

pub fn tokenize(data: &str) -> Result<(TOKEN, usize), ASTError> {
    let curr = data.chars().next().unwrap(); 
    match curr {
        ';' => { Ok((TOKEN::Comment, seek_past_newline(data))) }, // get the length until the \n 
        ':' => { let skip = seek_past_whitespace(&data[1..]); Ok((TOKEN::Property(data[1..skip].to_string()), skip)) },
        ',' => { let skip = seek_past_whitespace(&data[1..]); Ok((TOKEN::Interface(data[1..skip].to_string()), skip)) },
        ')' => { Ok((TOKEN::CParen, 1)) },
        '(' => { Ok((TOKEN::OParen, 1)) },
        '\'' => { let skip = seek_past_whitespace(&data[1..]); Ok((TOKEN::Type("type".to_string()), 4)) },
        '"' => { Ok((TOKEN::Quoted("quoted".to_string()), 6)) },
        '@' => { let skip = seek_past_whitespace(&data[1..]); Ok((TOKEN::Declaration("dec".to_string()), 3)) },
        '%' => { let skip = seek_past_whitespace(&data[1..]); Ok((TOKEN::Builtin("built".to_string()), 5)) },
        '#' => { let skip = seek_past_whitespace(&data[1..]); Ok((TOKEN::Pre("prev".to_string()), 4)) },
        '\t' => { Ok((TOKEN::Comment, seek_past_whitespace(&data[1..]))) },
        '\n' => { Ok((TOKEN::Comment, seek_past_whitespace(&data[1..]))) },
        ' ' => { Ok((TOKEN::Comment, seek_past_whitespace(&data[1..]))) },
        _ => { 
            if(curr.is_alphabetic()) {
                return Ok((TOKEN::Words("words".to_string()),5))
            }
            else if(curr.is_digit(10)) {
                return Ok((TOKEN::Number("5".to_string()), 1))
            }
            Err(ASTError("invalid token".to_string())) } 
    }
}

pub fn parse_from_str(source: &str, column: u32, line: u32) -> Result<Vec<AST>,ASTError> {
    let mut ast: Vec<AST> = Vec::new();
    let curr = source.chars().next().unwrap();
    match curr {
        '(' => { 
            ast.push(AST {}); 
            Ok(ast) 
        },
        _ => { return Err(ASTError(format!("invalid token: {}", curr))) }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ parse_from_str, tokenize, AST};
    #[test]
    fn tokenizes() {
        assert_eq!(tokenize("; hello\n  \t").unwrap().1, 11);
        assert_eq!(tokenize(":prop ()").unwrap().1, 5);
    }
}
