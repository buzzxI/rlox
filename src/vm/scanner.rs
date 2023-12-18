use std::collections::HashMap;

pub mod token;

pub struct Scanner<'a> {
    source: &'a str, 
    start: usize,
    current: usize,
    line: u32,
    column: u32,    
    current_column: u32,
    keywords: HashMap<&'static str, token::TokenType>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &str) -> Scanner {
        let keywords:HashMap<&'static str, token::TokenType> = [
            ("false", token::TokenType::False),
            ("true", token::TokenType::True),

            ("and", token::TokenType::And),
            ("or", token::TokenType::Or),
            ("xor", token::TokenType::Xor),

            ("if", token::TokenType::If),
            ("else", token::TokenType::Else),
            ("while", token::TokenType::While),
            ("for", token::TokenType::For),

            ("var", token::TokenType::Var),
            ("fun", token::TokenType::Fun),
            ("class", token::TokenType::Class),
            
            ("print", token::TokenType::Print),
            ("return", token::TokenType::Return),
            ("super", token::TokenType::Super),
            ("this", token::TokenType::This),
            ("nil", token::TokenType::Nil),
        ].iter().cloned().collect();

        Scanner {
            source, 
            start: 0, 
            current: 0,
            line: 1,
            column: 1,
            current_column: 1,
            keywords,
        }
    }

    fn scan_token(&mut self) -> token::Token<'a> {
        if let Err(token) = self.fast_forward() {
            return token;
        }
        self.start = self.current;
        self.column = self.current_column;
        
        self.advance()
            .map(|c| self.handle_char(c))
            .unwrap_or_else(|| self.generate_token(token::TokenType::Eof))
    }

    fn handle_char(&mut self,  c: char) -> token::Token<'a> {
        match c {
            '(' => self.generate_token(token::TokenType::LeftParen),
            ')' => self.generate_token(token::TokenType::RightParen),
            '{' => self.generate_token(token::TokenType::LeftBrace),
            '}' => self.generate_token(token::TokenType::RightBrace),
            ',' => self.generate_token(token::TokenType::Comma),
            '.' => self.generate_token(token::TokenType::Dot),
            ';' => self.generate_token(token::TokenType::Semicolon),
            '-' | '+' | '*' | '/' | '%' | '!' | '=' | '<' | '>' => self.handle_operator(c),
            '"' => self.handle_string(),
            _ => self.handle_number_and_identifier(c)
        }
    }

    fn handle_operator(&mut self, c: char) -> token::Token<'a> {
        let token_type = match c {
            '-' => {
                if self.match_char('-') {
                    token::TokenType::MinusMinus
                } else if self.match_char('=') {
                    token::TokenType::MinusEqual
                } else {
                    token::TokenType::Minus
                }
            },
            '+' => {
                if self.match_char('+') {
                    token::TokenType::PlusPlus
                } else if self.match_char('=') {
                    token::TokenType::PlusEqual
                } else {
                    token::TokenType::Plus
                }
            },
            '*' => {
                if self.match_char('*') {
                    token::TokenType::StarStar
                } else if self.match_char('=') {
                    token::TokenType::StarEqual
                } else {
                    token::TokenType::Star
                }
            },
            '/' => {
                if self.match_char('=') {
                    token::TokenType::SlashEqual
                } else {
                    token::TokenType::Slash
                }
            },
            '%' => {
                if self.match_char('=') {
                    token::TokenType::PercentEqual
                } else {
                    token::TokenType::Percent
                }
            },
            '!' => {
                if self.match_char('=') {
                    token::TokenType::BangEqual
                } else {
                    token::TokenType::Bang
                }
            },
            '=' => {
                if self.match_char('=') {
                    token::TokenType::EqualEqual
                } else {
                    token::TokenType::Equal
                }
            },
            '<' => {
                if self.match_char('=') {
                    token::TokenType::LessEqual
                } else {
                    token::TokenType::Less
                }
            },
            '>' => {
                if self.match_char('=') {
                    token::TokenType::GreaterEqual
                } else {
                    token::TokenType::Greater
                }
            },
            _ => unreachable!(),
        };
        self.generate_token(token_type)
    }

    fn handle_string(&mut self) -> token::Token<'a> {
        while let Some(c) = self.peek(0) {
            match c {
                '"' => {
                    self.advance();
                    return self.generate_token(token::TokenType::String);
                }
                '\n' => self.next_line(),
                _ => {}
            }
            self.advance();
        }
        self.error_token("unterminated string")
    }

    fn handle_number_and_identifier(&mut self, c: char) -> token::Token<'a> {
        if !c.is_ascii_alphanumeric() && c != '_' {
            return self.error_token("invalid character")
        }

        if c.is_ascii_digit() {
            return self.number();
        }
        self.identifier()

    }

    fn number(&mut self) -> token::Token<'a> {
        let mut has_decimal = false;

        while let Some(c) = self.peek(0) {
            if !c.is_ascii_digit() {
                if c == '.' && !has_decimal && self.peek(0).map_or(true, |c| !c.is_ascii_digit()) {
                    return self.error_token("invalid number with tailing '.'");
                }
                break;
            }
            self.advance();
            if c == '.' {
                has_decimal = true;
            }
        }

        self.generate_token(token::TokenType::Number)
    }

    fn identifier(&mut self) -> token::Token<'a> {
        while let Some(c) = self.peek(0) {
            if !c.is_ascii_alphanumeric() && c != '_' {
                break;
            }
            self.advance();
        }
        let identifier = &self.source[self.start..self.current];
        let token_type = self.keywords.get(identifier).cloned().unwrap_or(token::TokenType::Identifier);
        self.generate_token(token_type)
    }

    fn fast_forward(&mut self) -> Result<(), token::Token<'a>> {
        while let Some(c) = self.peek(0) {
            match c {
                '/' => {
                    let rst = self.jump_comment()?;
                    if !rst {
                        break;
                    }
                }
                _ if c.is_whitespace() => {
                    self.advance();
                    if c == '\n' {
                        self.next_line();
                    }
                }
                _ => break,
            }
        }
        Ok(())
    }

    /// return the current char without advancing the scanner
    fn peek(&self, distance: usize) -> Option<char> {
        self.source.chars().nth(self.current + distance)
    }

    /// return the current char and advance the scanner
    fn advance(&mut self) -> Option<char> {
        self.peek(0).map(|c| {
            self.current += 1;
            self.current_column += 1;
            c 
        })
    }
    
    /// Forward the scanner if the current char is the expected char 
    fn match_char(&mut self, expected: char) -> bool {
        if let Some(c) = self.peek(0) {
            if c == expected {
                self.advance();
                return true;
            }
        }
        false
    }

    fn next_line(&mut self) {
        self.line = self.line + 1;
        self.current_column = 1;
    }

    /// return the next char without advancing the scanner
    fn generate_token(&self, token_type: token::TokenType) -> token::Token<'a> {
        token::Token::new(token_type, &self.source[self.start..self.current] ,token::LocationInfo::new(self.line, self.column))
    }

    fn jump_comment(&mut self) -> Result<bool, token::Token<'a>> {
        if let Some(c) = self.peek(1) {
            match c {
                '/' => { self.jump_line_comment(); }
                '*' => { self.jump_block_comment()?; }
                _ => return Ok(false),
            }   
            return Ok(true);
        }
        Ok(false)
    }

    fn jump_line_comment(&mut self) {
        self.advance();
        self.advance();

        while let Some(c) = self.advance() {
            if c == '\n' {
                self.next_line();
                break;
            }
        }
    }

    fn jump_block_comment(&mut self) -> Result<(), token::Token<'a>> {
        self.advance();
        self.advance();

        while let Some(c) = self.advance() {
            if c == '\n' {
                self.next_line();
            } else if c == '*' && self.peek(0) == Some('/') {
                self.advance();
                return Ok(());
            }
        }
        Err(self.error_token("Unterminated block comment"))
    }

    fn error_token(&self, message: &str) -> token::Token<'a> {
        eprintln!("{}", message);
        self.generate_token(token::TokenType::Error)
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = token::Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.scan_token();
        if token.token_type == token::TokenType::Eof {
            None
        } else {
            Some(token)
        }
    }
}