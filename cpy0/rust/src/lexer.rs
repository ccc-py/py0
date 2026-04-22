#[derive(Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
    pub col: usize,
}

#[derive(Clone, PartialEq)]
pub enum TokenKind {
    Eof,
    Newline,
    Indent,
    Dedent,
    Name,
    Int,
    Float,
    String,
    Def,
    If,
    Else,
    While,
    Return,
    Pass,
    Lparen,
    Rparen,
    Lbracket,
    Rbracket,
    Comma,
    Colon,
    Dot,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqEq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

pub struct Lexer {
    source: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    col: usize,
    tokens: Vec<Token>,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
            col: 1,
            tokens: Vec::new(),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        self.col += 1;
        c
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        self.col += 1;
        true
    }

    fn make_token(&mut self, kind: TokenKind) {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token {
            kind,
            lexeme,
            line: self.line,
            col: self.col - (self.current - self.start),
        });
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '#' => {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                '\n' => {
                    self.advance();
                    self.line += 1;
                    self.col = 1;
                }
                _ => break,
            }
        }
    }

    fn scan_string(&mut self) {
        let quote = self.advance();
        let mut value = String::new();
        while self.peek() != quote && !self.is_at_end() {
            if self.peek() == '\\' {
                self.advance();
                match self.peek() {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    'r' => value.push('\r'),
                    '\\' => value.push('\\'),
                    '\'' => value.push('\''),
                    '"' => value.push('"'),
                    _ => value.push(self.peek()),
                }
            } else if self.peek() == '\n' {
                self.line += 1;
                self.col = 1;
                value.push(self.advance());
            } else {
                value.push(self.advance());
            }
        }
        if self.is_at_end() {
            crate::util::die("unterminated string");
        }
        self.advance();
        self.start = self.current;
        self.make_token(TokenKind::String);
        self.tokens.last_mut().unwrap().lexeme = value;
    }

    fn scan_number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance();
            while self.peek().is_ascii_digit() {
                self.advance();
            }
            self.make_token(TokenKind::Float);
        } else {
            self.make_token(TokenKind::Int);
        }
    }

    fn scan_name(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        let text: String = self.source[self.start..self.current].iter().collect();
        let kind = match text.as_str() {
            "def" => TokenKind::Def,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "return" => TokenKind::Return,
            "pass" => TokenKind::Pass,
            _ => TokenKind::Name,
        };
        self.make_token(kind);
    }

    pub fn scan_tokens(&mut self) {
        let mut indent_stack: Vec<usize> = vec![0];
        let mut at_line_start = true;

        while !self.is_at_end() {
            if at_line_start {
                let mut spaces = 0;
                while self.peek() == ' ' || self.peek() == '\t' {
                    spaces += 1;
                    self.advance();
                }
                if self.peek() == '#' || self.peek() == '\n' {
                } else {
                    if spaces > *indent_stack.last().unwrap() {
                        indent_stack.push(spaces);
                        self.make_token(TokenKind::Indent);
                    } else if spaces < *indent_stack.last().unwrap() {
                        while Some(&spaces) < indent_stack.last() {
                            indent_stack.pop();
                            self.make_token(TokenKind::Dedent);
                        }
                    }
                }
                at_line_start = false;
            }

            self.start = self.current;
            if self.is_at_end() {
                break;
            }

            let c = self.peek();
            if c == '\n' {
                self.advance();
                self.line += 1;
                self.col = 1;
                self.make_token(TokenKind::Newline);
                at_line_start = true;
                continue;
            }

            if c == ' ' || c == '\r' || c == '\t' {
                self.advance();
                continue;
            }

            if c == '#' {
                while self.peek() != '\n' && !self.is_at_end() {
                    self.advance();
                }
                continue;
            }

            if c == '"' || c == '\'' {
                self.scan_string();
                continue;
            }

            if c.is_ascii_digit() {
                self.scan_number();
                continue;
            }

            if c.is_ascii_alphabetic() || c == '_' {
                self.scan_name();
                continue;
            }

            self.advance();

            match c {
                '(' => self.make_token(TokenKind::Lparen),
                ')' => self.make_token(TokenKind::Rparen),
                '[' => self.make_token(TokenKind::Lbracket),
                ']' => self.make_token(TokenKind::Rbracket),
                ',' => self.make_token(TokenKind::Comma),
                ':' => self.make_token(TokenKind::Colon),
                '.' => self.make_token(TokenKind::Dot),
                '+' => self.make_token(TokenKind::Plus),
                '-' => self.make_token(TokenKind::Minus),
                '*' => self.make_token(TokenKind::Star),
                '/' => self.make_token(TokenKind::Slash),
                '%' => self.make_token(TokenKind::Percent),
                '=' => {
                    if self.match_char('=') {
                        self.make_token(TokenKind::EqEq);
                    } else {
                        self.make_token(TokenKind::Equal);
                    }
                }
                '!' => {
                    if self.match_char('=') {
                        self.make_token(TokenKind::Ne);
                    } else {
                        crate::util::die("unexpected character: !");
                    }
                }
                '<' => {
                    if self.match_char('=') {
                        self.make_token(TokenKind::Le);
                    } else {
                        self.make_token(TokenKind::Lt);
                    }
                }
                '>' => {
                    if self.match_char('=') {
                        self.make_token(TokenKind::Ge);
                    } else {
                        self.make_token(TokenKind::Gt);
                    }
                }
                _ => crate::util::die(&format!("unexpected character: {}", c)),
            }
        }

        if *indent_stack.last().unwrap() > 0 {
            indent_stack.pop();
            self.make_token(TokenKind::Dedent);
        }

        self.make_token(TokenKind::Eof);
    }

    pub fn get_tokens(self) -> Vec<Token> {
        self.tokens
    }
}