use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    Error,
    /**
     * single-character token
     * '(', ')', '{', '}'，',', '.', '-', '+', ';', '*', '/'
     */ 
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    /**
     * single or double characters token
     * '!', '!=', '=', '==', '>'u '>=', '<', '<='
     */
    Bang, BangEqual, Equal, EqualEqual,
    Greater, GreaterEqual, Less, LessEqual,

    /**
     * literals 
     */
    Identifier, String, Number,

    /**
     * keywords 
     */
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,
    Eof,

    /**
     * who knows
     * '%', '**', '++', '--', '+=', '-=', '*=', '/=', '%=', 'xor'
     */
    Percent, StarStar, PlusPlus, MinusMinus, 
    PlusEqual, MinusEqual, StarEqual, SlashEqual, PercentEqual,
    Xor,
}

pub struct LocationInfo {
    line: u32,
    column: u32,
}

impl LocationInfo {
    pub fn new(line: u32, column: u32) -> Self {
        Self {
            line,
            column,
        }
    }
}

pub struct Token<'a> {
    pub token_type: TokenType,
    pub lexeme: &'a str,
    pub localtion: LocationInfo,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, lexeme: &'a str, localtion: LocationInfo) -> Self {
        Self {
            token_type,
            lexeme,
            localtion,
        }
    }
}

impl fmt::Display for LocationInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line: {}, column: {}", self.line, self.column)
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "lox token: {:?}\nlexeme: {}\nlocaltion: {}\n", self.token_type, self.lexeme, self.localtion)    
    }
}