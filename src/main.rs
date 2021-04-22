#[derive(Debug, Clone)]
enum AST {
    Tsymbol(String),
    Tnum(f64),
    Tkey(String),
    Tlist(Vec<AST>),
}

#[derive(Debug, Clone)]
struct Terr(String);
#[derive(Debug, Clone)]
struct Perr(String);
#[derive(Debug, Clone)]
struct Scope(f64);

fn tokenize(expr: String) -> Vec<AST> {
    let chars = expr.as_str();
    let mut ast = vec![];
    let collection = chars.chars();
    let coll_len = chars.len();
    let loop_cont = true;
    while loop_cont {
        let output = parse(chars);
        match output {
            Ok(result) => {
                if (result.1 == '\0' || result.1 == '\n') {
                    loop_cont = false;
                } else {
                    ast.push(result.0)
                }
            }
            Err(_) => loop_cont = false,
        }
    }
    return ast;
}
fn parse<'a>(tokens: &'a str) -> Result<(AST, &'a str), Perr> {
    match item {
        '\n' => ast.push(Token::Tstring(item.to_string())),
        _ => ast.push(Token::Tstring("".to_string())),
    }
}
fn main() {}
