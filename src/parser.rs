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
    FinalKeyword,
    AbstractKeyword,
    ProtectedKeyword,
    StaticKeyword,
    
    OpenCurly,
    CloseCurly,
    OpenParen,
    CloseParen,
    
    SemiColon,
    Comma,
    Equal,
    
    Eof,
}

impl TokenKind {
    fn keyword_from_str(ident: &str) -> TokenKind {
        match ident {
            "class" => TokenKind::ClassKeyword,
            "private" => TokenKind::PrivateKeyword,
            "public" => TokenKind::PublicKeyword,
            "static" => TokenKind::StaticKeyword,
            "protected" => TokenKind::ProtectedKeyword,
            "abstract" => TokenKind::AbstractKeyword,
            "final" => TokenKind::FinalKeyword,

            _ => TokenKind::Identifier,
        }
    }

    fn is_keyword(&self) -> bool {
        match self {
            TokenKind::ClassKeyword | 
            TokenKind::PrivateKeyword |
            TokenKind::PublicKeyword |
            TokenKind::StaticKeyword |
            TokenKind::ProtectedKeyword |
            TokenKind::AbstractKeyword |
            TokenKind::FinalKeyword => true,
            _ => false
        }
    }
}

#[derive(Clone)]
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
    fn nth_chr(&self, offset: usize) -> char {
        match self.source.chars().nth(self.pos + offset) {
            Some(c) => c,
            None => '\0'
        }
    }

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
            
            '=' => {
                self.pos += 1;
                Token::new(TokenKind::Equal, "=".to_string(), self.pos - 1)
            },

            c if c.is_alphabetic() => {
                let start = self.pos;
                let ident = self.chr_take_while(|c| c.is_alphabetic() || c == '_' || c.is_digit(10) || c == '[' || c == ']');
                Token::new(TokenKind::keyword_from_str(&ident), ident, start)
            }

            '/' => {
                self.pos += 1;
                if self.curr_chr() == '/' {
                    while self.curr_chr() != '\n' && self.curr_chr() != '\0' {
                        self.pos += 1;
                    }
                } else if self.curr_chr() == '*' {
                    self.pos += 1;
                    while self.curr_chr() != '*' || self.nth_chr(1) != '/' {
                        self.pos += 1;
                    }
                    self.pos += 2;
                }
                
                self.consume_token()
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

    fn consume_expected(&mut self, kind: TokenKind) -> Option<Token> {
        let token = self.consume_token();

        if token.kind != kind {
            self.report_diagnostic(
                format!("Unexpected Token of kind {:?}, expected {:?}", 
                        token.kind, kind),
                token.pos);
            None
        } else {
            Some(token)
        }

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

    fn parse_keywords(&mut self) -> Vec<Token> {
        let mut keywords = Vec::new();
        
        while self.curr_token().kind.is_keyword() {
            keywords.push(self.consume_token());
        }

        keywords
    }


    fn parse_def(&mut self) -> Option<Declaration> {
        let keywords = self.parse_keywords();
        let capsulation = {
            let caps_keyword = keywords
                               .iter()
                               .find(|k| k.kind == TokenKind::PrivateKeyword
                                      || k.kind == TokenKind::PublicKeyword
                                      || k.kind == TokenKind::ProtectedKeyword);
            if caps_keyword.is_some() {
                match caps_keyword.unwrap().kind {
                    TokenKind::PrivateKeyword => Capsulation::Private,
                    TokenKind::PublicKeyword => Capsulation::Public,
                    TokenKind::ProtectedKeyword => Capsulation::Protected,
                    _ => unreachable!()
                }
            } else {
                Capsulation::Public
            }
        };

        let field_type = self.consume_expected(TokenKind::Identifier)?;
        let (name, is_constructor) = if self.curr_token().kind == TokenKind::OpenParen {
            (field_type.clone(), true)
        } else {
            (self.consume_expected(TokenKind::Identifier)?, false)
        };

        if self.consume_optional(TokenKind::OpenParen) {
            let mut start_pos = self.pos;
            let mut params = Vec::new();

            while self.curr_token().kind != TokenKind::CloseParen {
                let param_type = self.consume_expected(TokenKind::Identifier)?;
                let name = self.consume_expected(TokenKind::Identifier)?;
                let param = Parameter(Type(param_type.text), name.text);

                params.push(param);

                if self.pos == start_pos {
                    break;
                }

                start_pos = self.pos;

                if self.curr_token().kind != TokenKind::Comma {
                    break;
                }

                self.consume_expected(TokenKind::Comma)?;
            }

            self.consume_expected(TokenKind::CloseParen)?;

            if !self.consume_optional(TokenKind::SemiColon) {
                self.consume_expected(TokenKind::OpenCurly)?;
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

            Some(Declaration::Method(
                if is_constructor {
                    Method::new_constructor(name.text, params, capsulation)
                } else {
                    Method::new(name.text, Type(field_type.text), params, capsulation)
                }))
        } else {
            if self.curr_token().kind == TokenKind::Equal {
                self.consume_token();
                while self.curr_chr() != ';' && self.curr_chr() != '\0' {
                    self.pos += 1;
                }
            }

            self.consume_expected(TokenKind::SemiColon)?;
            Some(Declaration::Field(Field::new(Type(field_type.text), name.text, capsulation)))
        }
    }

    pub fn parse_class_def(&mut self) -> Option<Class> {
        self.consume_expected(TokenKind::ClassKeyword)?;
        let name = self.consume_expected(TokenKind::Identifier)?;
        self.consume_expected(TokenKind::OpenCurly)?;
        let mut start_pos = self.pos;
        let mut fields = Vec::new();
        let mut methods = Vec::new();

        while self.curr_token().kind != TokenKind::CloseCurly {
            let def = self.parse_def()?;

            match def {
                Declaration::Field(field) => fields.push(field),
                Declaration::Method(method) => methods.push(method),
            }

            if self.pos == start_pos {
                break;
            }

            start_pos = self.pos;
        }

        self.consume_expected(TokenKind::CloseCurly)?;

        Some(Class::new(name.text, fields, methods))
    }
}
