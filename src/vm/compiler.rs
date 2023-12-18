use crate::object;
use super::scanner::{self, token};

// struct Parser<'a> {
//     previous: Option<&'a token::Token<'a>>,
//     current: Option<token::Token<'a>>,
//     had_error: bool,
//     panic_mode: bool,
//     scanner: scanner::Scanner<'a>,
// }

// impl<'a> Parser<'a> {
//     pub fn new(source: &'a str) -> Self {
//         Self {
//             previous: None,
//             current: None,
//             had_error: false,
//             panic_mode: false,
//             scanner: scanner::Scanner::new(source),
//         }
//     }

//     pub fn compile(&self) -> Option<object::FunctionObj> {
//         None
//     }
// }

pub fn compile<'a>(source: &'a str) -> Option<object::FunctionObj> {
    println!("{}", source);
    let mut scanner = scanner::Scanner::new(source);

    while let Some(token) = scanner.next() {
        println!("{}", token);
        if token.token_type == token::TokenType::Error {
            return None;
        }
    }
    
    Some(object::FunctionObj::new(String::from("script")))
}