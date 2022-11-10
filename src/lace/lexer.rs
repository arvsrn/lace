use crate::lace::{io, token};


pub struct Lexer {
    pub source:   io::Source,
    pub position: (usize, usize),
    index:        isize,
    current:      Option<char>,
}

impl Lexer {
    pub fn new(source: io::Source) -> Self {
        Self {
            source,
            position: (1, 0),
            index:    -1,
            current:  None,
        }
    }

    pub fn next_token(&mut self) -> token::Token {
        let mut current = token::Token::new(token::Kind::EOF, "EndOfFile".to_owned(), None,
                                           (self.position.0, self.position.1));
        match self.current {
            None => {
                if self.source.content.len() != self.index as usize {
                    self.advance();
                    return self.next_token();
                }
            },

            Some(' ') | Some('\t') => {
                self.advance();
                return self.next_token();
            },

            Some('=') => {
                self.advance();
                if self.current == Some('=') {
                    current = token::Token::new(token::Kind::Equal, "==".to_owned(), None,
                                        (self.position.0, self.position.1 - 1));
                } else {
                    self.index -= 1;
                    current = token::Token::new(token::Kind::Assign, "=".to_owned(), None,
                                        (self.position.0, self.position.1 - 1));
                }
            },

            Some(';') => {
                current = token::Token::new(token::Kind::Semicolon, ";".to_owned(), None,
                                        (self.position.0, self.position.1));
            },

            Some(':') => {
                current = token::Token::new(token::Kind::Colon, ":".to_owned(), None,
                                        (self.position.0, self.position.1));
            },

            Some('(') => {
                current = token::Token::new(token::Kind::LeftParen, "(".to_owned(), None,
                                        (self.position.0, self.position.1));
            },

            Some(')') => {
                current = token::Token::new(token::Kind::RightParen, ")".to_owned(), None,
                                        (self.position.0, self.position.1));
            },

            Some('{') => {
                current = token::Token::new(token::Kind::LeftCurlyBracket, "{".to_owned(), None,
                                        (self.position.0, self.position.1));
            },

            Some('}') => {
                current = token::Token::new(token::Kind::RightCurlyBracket, "}".to_owned(), None,
                                        (self.position.0, self.position.1));
            },

            Some(',') => {
                current = token::Token::new(token::Kind::Comma, ",".to_owned(), None,
                                        (self.position.0, self.position.1));
            },

            Some('+') => {
                current = token::Token::new(token::Kind::Plus, "+".to_owned(), None,
                                        (self.position.0, self.position.1));
            },

            Some('-') => {
                current = token::Token::new(token::Kind::Minus, "-".to_owned(), None,
                                        (self.position.0, self.position.1));
            },

            Some('*') => {
                current = token::Token::new(token::Kind::Multiply, "*".to_owned(), None,
                                        (self.position.0, self.position.1));
            },

            Some('/') => {
                current = token::Token::new(token::Kind::Divide, "/".to_owned(), None,
                                        (self.position.0, self.position.1));
            },

            Some('!') => {
                self.advance();
                if self.current == Some('=') {
                    current = token::Token::new(token::Kind::NotEqual, "!=".to_owned(), None,
                                        (self.position.0, self.position.1 - 1));
                }
            },

            Some('>') => {
                self.advance();
                if self.current == Some('=') {
                    current = token::Token::new(token::Kind::GreaterThanOrEqual, ">=".to_owned(), None,
                                        (self.position.0, self.position.1));
                } else {
                    current = token::Token::new(token::Kind::GreaterThan, ">".to_owned(), None,
                                        (self.position.0, self.position.1));
                    self.index -= 1;
                }
            },

            Some('<') => {
                self.advance();
                if self.current == Some('=') {
                    current = token::Token::new(token::Kind::LessThanOrEqual, "<=".to_owned(), None,
                                        (self.position.0, self.position.1));
                } else {
                    current = token::Token::new(token::Kind::LessThan, "<".to_owned(), None,
                                        (self.position.0, self.position.1));
                    self.index -= 1;
                }
            },


            Some('\"') | Some('\'') => current = self.lex_string(),

            _ if self.current.unwrap().is_alphabetic() => current = self.lex_identifier(),

            _ if self.current.unwrap().is_numeric() => current = self.lex_number(),

            Some('\n') => current = self.skip_lines(),

            _ => {},
        }

        self.advance();
        current
    }

    fn lex_string(&mut self) -> token::Token {
            let quote = self.current;
            let mut value = String::new();
            let start = self.position.1;
            self.advance();
            while self.current.is_some() && self.current != quote {
                value.push(self.current.unwrap());
                self.advance();
            }

            token::Token::new(token::Kind::String, value, None,
                             (self.position.0, start))
    }

    fn lex_identifier(&mut self) -> token::Token {
            let mut value = String::new();
            let start = self.position.1;
            while self.current.is_some() && (self.current.unwrap().is_alphanumeric() || self.current.unwrap() == '_') {
                value.push(self.current.unwrap());
                self.advance();
            }

            self.index -= 1;
            token::Token::new(token::Kind::Identifier, value, None,
                             (self.position.0, start))
    }

    fn lex_number(&mut self) -> token::Token {
            let mut value = String::new();
            let start = self.position.1;
            while self.current.is_some() && self.current.unwrap().is_numeric() {
                value.push(self.current.unwrap());
                self.advance();
            }

            self.index -= 1;
            let int_value = value.clone().parse();
            token::Token::new(token::Kind::Integer, value, Some(int_value.unwrap()),
                             (self.position.0, start))

    }

    fn skip_lines(&mut self) -> token::Token {
            let (sy, sx) = (self.position.0, self.position.1);
            while self.current == Some('\n') {
                self.position.0 += 1;
                self.advance();
            }

            self.position.1 = 1;
            self.index -= 1;
            token::Token::new(token::Kind::Newline, "\\n".to_owned(), None,
                             (sy, sx))

    }

    fn advance(&mut self) {
        self.position.1 += 1;
        self.index += 1;
        self.current = self.source.content.chars().nth(self.index as usize);
    }
}
