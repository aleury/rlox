#[derive(Debug)]
pub enum ScanError {
    IllegalCharacter { line: usize, character: char },
    UnterminatedString { line: usize },
    ParseNumberError { lexeme: String, line: usize },
}

pub struct Scanner {
    source: Vec<char>,
    line: usize,
    start: usize,
    current: usize,
    tokens: Vec<Token>,
    pub errors: Vec<ScanError>,
}

impl Scanner {
    #[must_use]
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            line: 1,
            start: 0,
            current: 0,
            tokens: vec![],
            errors: vec![],
        }
    }

    #[must_use]
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {
            line: self.line,
            lexeme: String::new(),
            kind: TokenKind::Eof,
        });
        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        match self.advance() {
            '(' => self.add_token(TokenKind::LeftParen),
            ')' => self.add_token(TokenKind::RightParen),
            '{' => self.add_token(TokenKind::LeftBrace),
            '}' => self.add_token(TokenKind::RightBrace),
            ',' => self.add_token(TokenKind::Comma),
            '.' => self.add_token(TokenKind::Dot),
            '-' => self.add_token(TokenKind::Minus),
            '+' => self.add_token(TokenKind::Plus),
            ';' => self.add_token(TokenKind::Semicolon),
            '*' => self.add_token(TokenKind::Star),
            '!' => {
                let token_type = if self.matches('=') {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                };
                self.add_token(token_type);
            }
            '=' => {
                let token_type = if self.matches('=') {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                };
                self.add_token(token_type);
            }
            '<' => {
                let token_type = if self.matches('=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                };
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.matches('=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                };
                self.add_token(token_type);
            }
            '/' => {
                if self.matches('/') {
                    // A comment goes until the end of the line.
                    while self.peek() != '\n' && !self.at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenKind::Slash);
                }
            }
            ' ' | '\r' | '\t' => {} // ignore whitespace
            '\n' => {
                self.line += 1;
            }
            '"' => self.string(),
            c => {
                if c.is_ascii_digit() {
                    self.number();
                } else if c.is_ascii_alphabetic() || c == '_' {
                    self.identifier();
                } else {
                    self.errors.push(ScanError::IllegalCharacter {
                        line: self.line,
                        character: c,
                    });
                }
            }
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        let token_type = match lexeme.as_str() {
            "and" => TokenKind::And,
            "class" => TokenKind::Class,
            "else" => TokenKind::Else,
            "false" => TokenKind::False,
            "for" => TokenKind::For,
            "fun" => TokenKind::Fun,
            "if" => TokenKind::If,
            "nil" => TokenKind::Nil,
            "or" => TokenKind::Or,
            "print" => TokenKind::Print,
            "return" => TokenKind::Return,
            "super" => TokenKind::Super,
            "this" => TokenKind::This,
            "true" => TokenKind::True,
            "var" => TokenKind::Var,
            "while" => TokenKind::While,
            _ => TokenKind::Identifier,
        };
        self.add_token(token_type);
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // look for the fractional part
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance(); // consume the '.'
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let lexeme: String = self.source[self.start..self.current].iter().collect();

        match lexeme.parse::<f64>() {
            Ok(number) => self.add_token(TokenKind::Number(number)),
            Err(_) => {
                self.errors.push(ScanError::ParseNumberError {
                    lexeme,
                    line: self.line,
                });
            }
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.at_end() {
            self.errors
                .push(ScanError::UnterminatedString { line: self.line });
            return;
        }

        // The closing "
        self.advance();

        // Trim the surrounding quotes
        let string: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token(TokenKind::String(string));
    }

    fn peek(&self) -> char {
        if self.at_end() {
            return '\0';
        }
        self.source[self.current]
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source[self.current + 1]
    }

    fn matches(&mut self, c: char) -> bool {
        if self.at_end() {
            return false;
        }
        if self.source[self.current] != c {
            return false;
        }
        self.current += 1;
        true
    }

    fn add_token(&mut self, kind: TokenKind) {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token {
            line: self.line,
            lexeme,
            kind,
        });
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    Number(f64),
    String(String),

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub line: usize,
    pub lexeme: String,
    pub kind: TokenKind,
}
