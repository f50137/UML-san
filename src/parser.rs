use crate::java::*;

pub struct Parser<'s> {
    source: &'s str,
    pos: usize,
    pub diagnostics: Vec<Diagnostic>,
}

impl<'s> Parser<'s> {
    pub fn new(source: &'s str) -> Self {
        Self {
            source,
            pos: 0,
            diagnostics: Vec::new(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum TokenKind {
    Identifier,

    ClassKeyword,
    PublicKeyword,
    PrivateKeyword,
    
    OpenCurly,
    CloseCurly,
    OpenParen,
    CloseParen,
    
    SemiColon,
    Comma,
    
    Eof,
}

impl TokenKind {
    fn keyword_from_str(ident: &str) -> TokenKind {
        match ident {
            "class" => TokenKind::ClassKeyword,
            "private" => TokenKind::PrivateKeyword,
            "public" => TokenKind::PublicKeyword,
            _ => TokenKind::Identifier,
        }
    }
}

struct Token {
    kind: TokenKind,
    text: String,
    pos: usize,
}

impl Token {
    fn new(kind: TokenKind, text: String, pos: usize) -> Self {
        Self {
            kind, text, pos
        }
    }
}

pub struct Diagnostic {
    pub message: String,
    pub pos: usize,
}


impl Parser<'_> {
    //fn nth_chr(&self, offset: usize) -> char {
    //    match self.source.chars().nth(self.pos + offset) {
    //        Some(c) => c,
    //        None => '\0'
    //    }
    //}

    fn curr_chr(&self) -> char {
        match self.source.chars().nth(self.pos) {
            Some(c) => c,
            None => '\0',
        }
    }

    fn chr_drop_while(&mut self, pred: fn (char) -> bool) {
        while (pred)(self.curr_chr()) {
            self.pos += 1;
        }
    }
    
    fn chr_take_while(&mut self, pred: fn (char) -> bool) -> String {
        let mut out = String::new();
        let mut curr: char;

        while {
            curr = self.curr_chr();
            (pred)(curr)
        } {
            self.pos += 1;
            out.push(curr);
        }

        out
    }

    fn report_diagnostic(&mut self, message: String, pos: usize) {
        self.diagnostics.push(Diagnostic { message, pos })
    }

    fn consume_token(&mut self) -> Token {
        self.chr_drop_while(|c| c.is_whitespace());
        match self.curr_chr() {
            '\0' => {
                Token::new(TokenKind::Eof, "\0".to_string(), self.pos)
            },
            
            '{' => {
                self.pos += 1;
                Token::new(TokenKind::OpenCurly, "{".to_string(), self.pos - 1)
            },
            
            '}' => {
                self.pos += 1;
                Token::new(TokenKind::CloseCurly, "}".to_string(), self.pos - 1)
            },

            '(' => {
                self.pos += 1;
                Token::new(TokenKind::OpenParen, "(".to_string(), self.pos - 1)
            },

            ')' => {
                self.pos += 1;
                Token::new(TokenKind::CloseParen, ")".to_string(), self.pos - 1)
            },
            
            ';' => {
                self.pos += 1;
                Token::new(TokenKind::SemiColon, ";".to_string(), self.pos - 1)
            },

            ',' => {
                self.pos += 1;
                Token::new(TokenKind::Comma, ",".to_string(), self.pos - 1)
            },

            c if c.is_alphabetic() => {
                let start = self.pos;
                let ident = self.chr_take_while(|c| c.is_alphabetic());
                Token::new(TokenKind::keyword_from_str(&ident), ident, start)
            }

            c @ _ => {
                self.report_diagnostic(format!("Bad character input '{}'", c), self.pos);
                self.pos += 1;
                self.consume_token()
            }
        }
    }

    fn curr_token(&mut self) -> Token {
        let pos = self.pos;
        let tok = self.consume_token();
        self.pos = pos;
        tok
    }

    fn consume_expected(&mut self, kind: TokenKind) -> Token {
        let token = self.consume_token();

        if token.kind != kind {
            self.report_diagnostic(
                format!("Unexpected Token of kind {:?}, expected {:?}", 
                        token.kind, kind),
                token.pos);
        }

        token
    }

    fn consume_optional(&mut self, kind: TokenKind) -> bool {
        let pos = self.pos;
        let token = self.consume_token();

        if token.kind == kind {
            true
        } else {
            self.pos = pos;
            false
        }
    }

    fn parse_def(&mut self) -> Declaration {
        let capsulation = if self.consume_optional(TokenKind::PrivateKeyword) {
            Capsulation::Private
        } else {
            self.consume_optional(TokenKind::PublicKeyword);
            Capsulation::Public
        };

        let field_type = self.consume_expected(TokenKind::Identifier);
        let name = self.consume_expected(TokenKind::Identifier);

        if self.consume_optional(TokenKind::OpenParen) {
            let mut start_pos = self.pos;
            let mut params = Vec::new();

            while self.curr_token().kind != TokenKind::CloseParen {
                let param_type = self.consume_expected(TokenKind::Identifier);
                let name = self.consume_expected(TokenKind::Identifier);
                let param = Parameter(Type(param_type.text), name.text);

                params.push(param);

                if self.pos == start_pos {
                    break;
                }

                start_pos = self.pos;

                if self.curr_token().kind != TokenKind::Comma {
                    break;
                }

                self.consume_expected(TokenKind::Comma);
            }

            self.consume_expected(TokenKind::CloseParen);

            if !self.consume_optional(TokenKind::SemiColon) {
                self.consume_expected(TokenKind::OpenCurly);
                let mut curly_stack = 1;
                let mut curr;

                while {
                    curr = self.curr_chr();
                    curly_stack != 0 && curr != '\0'
                }{
                    match curr {
                        '}' => curly_stack -= 1,
                        '{' => curly_stack += 1,
                        _ => ()
                    }
                    self.pos += 1;
                }
            }

            Declaration::Method(Method::new(name.text, Type(field_type.text), params, capsulation))
        } else {
            self.consume_expected(TokenKind::SemiColon);
            Declaration::Field(Field::new(Type(field_type.text), name.text, capsulation))
        }
    }

    pub fn parse_class_def(&mut self) -> Class {
        self.consume_expected(TokenKind::ClassKeyword);
        let name = self.consume_expected(TokenKind::Identifier);
        self.consume_expected(TokenKind::OpenCurly);
        let mut start_pos = self.pos;
        let mut fields = Vec::new();
        let mut methods = Vec::new();

        while self.curr_token().kind != TokenKind::CloseCurly {
            let def = self.parse_def();
            match def {
                Declaration::Field(field) => fields.push(field),
                Declaration::Method(method) => methods.push(method),
            }

            if self.pos == start_pos {
                break;
            }

            start_pos = self.pos;
        }

        self.consume_expected(TokenKind::CloseCurly);

        Class::new(name.text, fields, methods)
    }
}
