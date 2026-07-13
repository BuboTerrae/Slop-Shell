#[derive(Debug, PartialEq)]
pub enum Token {
    Command(String),
    StringLiteral(String),
    Whitespace,
    Eof,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        if self.pos >= self.input.len() {
            return Token::Eof;
        }

        let ch = self.input[self.pos];

        if ch.is_whitespace() {
            self.pos += 1;
            return Token::Whitespace;
        }

        if ch == '"' {
            return self.read_string();
        }

        self.read_command()
    }

    pub fn read_string(&mut self) -> Token {
        self.pos += 1;
        let mut s = String::new();
        while self.pos < self.input.len() {
            let ch = self.input[self.pos];
            if ch == '"' {
                self.pos += 1;
                return Token::StringLiteral(s);
            }
            s.push(ch);
            self.pos += 1;
        }
        Token::StringLiteral(s)
    }

    pub fn read_command(&mut self) -> Token {
        let mut cmd = String::new();
        while self.pos < self.input.len() {
            let ch = self.input[self.pos];
            if ch.is_whitespace() || ch == '"' {
                break;
            }
            cmd.push(ch);
            self.pos += 1;
        }
        Token::Command(cmd)
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        loop {
            let tok = self.next_token();
            if matches!(tok, Token::Eof) {
                break;
            }
            if !matches!(tok, Token::Whitespace) {
                tokens.push(tok);
            }
        }
        tokens
    }
}
