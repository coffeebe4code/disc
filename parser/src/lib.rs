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
pub fn tokenize(data: &str) -> Result<(TOKEN, usize), ASTError> {
    let curr = data.chars().next().unwrap(); 
    match curr {
        ';' => { Ok((TOKEN::Comment, 1)) }, // get the length until the \n 
        ':' => { Ok((TOKEN::Property("prop".to_string()), 4)) },
        ',' => { Ok((TOKEN::Interface("int".to_string()), 3)) },
        ')' => { Ok((TOKEN::CParen, 1)) },
        '(' => { Ok((TOKEN::OParen, 1)) },
        '\'' => { Ok((TOKEN::Type("type".to_string()), 4)) },
        '"' => { Ok((TOKEN::Quoted("quoted".to_string()), 6)) },
        '@' => { Ok((TOKEN::Declaration("dec".to_string()), 3)) },
        '%' => { Ok((TOKEN::Builtin("built".to_string()), 5)) },
        '#' => { Ok((TOKEN::Pre("prev".to_string()), 4)) },
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
    use crate::{ parse_from_str, AST};
    #[test]
    fn parses() {
        assert_eq!(parse_from_str("/",0,0).unwrap_err().0, "invalid token: /".to_string());
        assert_eq!(parse_from_str("()",0,0).unwrap().first().unwrap(), 0);
    }
}
