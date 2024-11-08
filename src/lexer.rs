use std::fmt;

#[derive(Debug)]
pub enum TokenTypes {
    Add,    // +
    Sub,    // -
    Div,    // /
    Mult,   // *
    Mod,    // %

    IntDiv, // //
    Pow,    // **
    
    Inc, // ++
    Dec, // --
    
    Assign,     // =
    AddAssign,  // +=
    SubAssign,  // -=
    DivAssign,  // /=
    MultAssign, // *=
    ModAssign,  // %=
    
    //Comparison Operators

    Equal,        // ==
    NotEqual,     // !=
    LessThan,     // <
    Greater,      // >
    LessEqual,    // <=
    GreaterEqual, // >=

    And, // &&
    Or,  // ||
    Not, // !

    //Bitwise

    BitAnd, // &
    BitOr,  // |
    BitNot, // ¬

    Int(Option<i64>),
    Float(Option<f64>),
    String(Option<String>),
    Char(Option<char>),
    Bool(Option<bool>),
    None,

    Var, // initialise variable
    Const, // initialise constant
    
    Print,

    Indentifier {name: String, value:Box<TokenTypes>},

    LeftParen, // (
    RightParen, // )

    LeftCurly, // {
    RightCurly, // }

    LeftSquare, // [
    RightSquare, // ],

    Colon, // :


    EndStatement, // \n
    
}
impl fmt::Display for TokenTypes {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenTypes,
}

impl Token {
    pub fn new(token_type: TokenTypes) -> Self {
        return Self {
            token_type,
        }
    }
}

pub struct Lexer {
    tokens: Vec<Token>,
    contents: Vec<char>,
    current_pos: usize,
}

impl Lexer {
    pub fn new(contents:Vec<char>) -> Self {
        return Self {
            tokens: Vec::new(),
            current_pos: 0,
            contents
        }
    }

    fn scry(&self, offset:usize) -> Option<char> {
        if self.current_pos + offset + 1 <= self.contents.len() {
            return Some(*self.contents.get(&self.current_pos + offset).unwrap())
        } else {
            return None
        }
    }

    pub fn scan(&mut self) -> &Vec<Token> {
        while self.current_pos < self.contents.len() {
            let mut current_char = self.contents.get(self.current_pos).unwrap();
            match current_char {
                ' ' | '\t' => {}

                '\n' => self.tokens.push(Token::new(TokenTypes::EndStatement)),

                ':' => self.tokens.push(Token::new(TokenTypes::Colon)),

                '(' => self.tokens.push(Token::new(TokenTypes::LeftParen)),
                ')' => self.tokens.push(Token::new(TokenTypes::RightParen)),

                '[' => self.tokens.push(Token::new(TokenTypes::LeftSquare)),
                ']' => self.tokens.push(Token::new(TokenTypes::RightSquare)),

                '{' => self.tokens.push(Token::new(TokenTypes::LeftCurly)),
                '}' => self.tokens.push(Token::new(TokenTypes::RightCurly)),

                '"' => {
                    let mut buffer: Vec<char> = Vec::new();
                    let mut string_end: bool = false;
                    self.current_pos += 1;
                    while string_end && self.current_pos < self.contents.len() {
                        current_char = self.contents.get(self.current_pos).unwrap();
                        if *current_char != '"' {
                            buffer.push(*current_char);
                        } else {
                            string_end = true;
                        }
                        self.current_pos += 1;
                    }
                    self.current_pos -= 1;
                    self.tokens.push(Token::new( TokenTypes::String(Some(buffer.iter().collect()))))
                }

                '\'' => {todo!("Add Chars")}

                '0'..':' => {
                    let mut buffer: Vec<char> = Vec::new();
                    let mut is_float = false;
                    while ( (*current_char < ':' && *current_char >= '0') || *current_char == '.' ) && self.current_pos < self.contents.len()  {
                        buffer.push(*current_char);
                        self.current_pos += 1;
                        current_char = self.contents.get(self.current_pos).unwrap();

                        if *current_char == '.' {
                            is_float = true
                        }
                    }
                    self.current_pos -= 1;
                    let mut num: String = buffer.iter().collect();
                    num = num.trim().to_string();
                    if is_float {
                        self.tokens.push(Token::new(TokenTypes::Float(Some(num.parse::<f64>().unwrap()))))
                    } else {
                        self.tokens.push(Token::new(TokenTypes::Int(Some(num.parse::<i64>().unwrap()))))
                    }

                }

                '+' => {
                    let next_char = self.scry(1);
                    if next_char.is_some() {
                        match next_char.unwrap() {
                            '+' => {
                                self.tokens.push(Token::new(TokenTypes::Inc));
                                self.current_pos += 1;
                            }
                            '=' => {
                                self.tokens.push(Token::new(TokenTypes::AddAssign));
                                self.current_pos += 1;
                            }
                            _ => self.tokens.push(Token::new(TokenTypes::Add))
                        }
                    } else {
                        self.tokens.push(Token::new(TokenTypes::Add))
                    }
                }

                '-' => {
                    let next_char = self.scry(1);
                    if next_char.is_some() {
                        match next_char.unwrap() {
                            '-' => {
                                self.tokens.push(Token::new(TokenTypes::Dec));
                                self.current_pos += 1;
                            }
                            '=' => {
                                self.tokens.push(Token::new(TokenTypes::SubAssign));
                                self.current_pos += 1;
                            }
                            _ => self.tokens.push(Token::new(TokenTypes::Sub))
                        }
                    } else {
                        self.tokens.push(Token::new(TokenTypes::Sub))
                    }
                }

                '/' => {
                    let next_char = self.scry(1);
                    if next_char.is_some() {
                        match next_char.unwrap() {
                            '/' => {
                                self.tokens.push(Token::new(TokenTypes::IntDiv));
                                self.current_pos += 1;
                            }
                            '=' => {
                                self.tokens.push(Token::new(TokenTypes::DivAssign));
                                self.current_pos += 1;
                            }
                            _ => self.tokens.push(Token::new(TokenTypes::Div))
                        }
                    } else {
                        self.tokens.push(Token::new(TokenTypes::Div,))
                    }
                }

                '*' => {
                    let next_char = self.scry(1);
                    if next_char.is_some() {
                        match next_char.unwrap() {
                            '*' => {
                                self.tokens.push(Token::new(TokenTypes::Pow));
                                self.current_pos += 1;
                            }
                            '=' => {
                                self.tokens.push(Token::new(TokenTypes::MultAssign));
                                self.current_pos += 1;
                            }
                            _ => self.tokens.push(Token::new(TokenTypes::Mult))
                        }
                    } else {
                        self.tokens.push(Token::new(TokenTypes::Mult))
                    }
                }

                '%' => {
                    let next_char = self.scry(1);
                    if next_char.is_some() && next_char.unwrap() == '=' {
                        self.tokens.push(Token::new(TokenTypes::ModAssign));
                        self.current_pos += 1;
                    } else {
                        self.tokens.push(Token::new(TokenTypes::Mod));
                    }
                }

                 '>' => {
                    let next_char = self.scry(1);
                    if next_char.is_some() && next_char.unwrap() == '=' {
                        self.tokens.push(Token::new(TokenTypes::GreaterEqual));
                        self.current_pos += 1;
                    } else {
                        self.tokens.push(Token::new(TokenTypes::Greater));
                    }
                }

                '<' => {
                    let next_char = self.scry(1);
                    if next_char.is_some() && next_char.unwrap() == '=' {
                        self.tokens.push(Token::new(TokenTypes::LessEqual));
                        self.current_pos += 1;
                    } else {
                        self.tokens.push(Token::new(TokenTypes::LessThan));
                    }
                }

                '=' => {
                    let next_char = self.scry(1);
                    if next_char.is_some() && next_char.unwrap() == '=' {
                        self.tokens.push(Token::new(TokenTypes::Equal));
                        self.current_pos += 1;
                    } else {
                        self.tokens.push(Token::new(TokenTypes::Assign));
                    }
                }

                '!' => {
                    let next_char = self.scry(1);
                    if next_char.is_some() && next_char.unwrap() == '=' {
                        self.tokens.push(Token::new(TokenTypes::NotEqual));
                        self.current_pos += 1;
                    } else {
                        self.tokens.push(Token::new(TokenTypes::Not));
                    }
                }

                '&' => {
                    let next_char = self.scry(1);
                    if next_char.is_some() && next_char.unwrap() == '&' {
                        self.tokens.push(Token::new(TokenTypes::And));
                        self.current_pos += 1;
                    } else {
                        self.tokens.push(Token::new(TokenTypes::BitAnd));
                    }
                }

                '|' => {
                    let next_char = self.scry(1);
                    if next_char.is_some() && next_char.unwrap() == '|' {
                        self.tokens.push(Token::new(TokenTypes::Or));
                        self.current_pos += 1;
                    } else {
                        self.tokens.push(Token::new(TokenTypes::BitOr));
                    }
                }

                '¬' => {self.tokens.push(Token::new(TokenTypes::BitNot))}

                _ =>  {
                    if current_char.is_alphabetic() {
                        let mut buffer: Vec<char> = Vec::new();
                        while current_char.is_alphanumeric() {
                            buffer.push(*current_char);
                            self.current_pos += 1;
                            current_char = self.contents.get(self.current_pos).unwrap();
                        }
                        self.current_pos -= 1;
                        let mut keyword: String = buffer.iter().collect();
                        keyword = keyword.trim().to_string();
                        match &keyword as &str {
                            "true" => self.tokens.push(Token::new(TokenTypes::Bool(Some(true)))),
                            "false" => self.tokens.push(Token::new(TokenTypes::Bool(Some(false)))),
                            
                            "var" => self.tokens.push(Token::new(TokenTypes::Var)),
                            "const" => self.tokens.push(Token::new(TokenTypes::Const)),
                            
                            "print" => self.tokens.push(Token::new(TokenTypes::Print)),
                            
                            "int" => self.tokens.push(Token::new(TokenTypes::Int(None))),
                            "float" => self.tokens.push(Token::new(TokenTypes::Float(None))),
                            "string" => self.tokens.push(Token::new(TokenTypes::String(None))),
                            "bool" => self.tokens.push(Token::new(TokenTypes::Bool(None))),

                            _ => {self.tokens.push(Token::new(TokenTypes::Indentifier { name: keyword, value: Box::from(TokenTypes::None)}))}
                        };
                    }
                },
            }
            self.current_pos += 1;
        };
        return &self.tokens
    }
}
