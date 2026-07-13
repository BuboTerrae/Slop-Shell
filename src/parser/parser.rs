use crate::lexer::lexer::Token;

#[derive(Debug)]
pub enum Ast {
    Yap(String),
    NoOp,
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Ast {
        if self.pos >= self.tokens.len() {
            return Ast::NoOp;
        }

        match &self.tokens[self.pos] {
            Token::Command(cmd) if cmd == "yap" => {
                self.pos += 1;
                if let Some(Token::StringLiteral(s)) = self.tokens.get(self.pos) {
                    self.pos += 1;
                    Ast::Yap(s.clone())
                } else {
                    Ast::Yap("".to_string())
                }
            }
            _ => Ast::NoOp,
        }
    }
}
