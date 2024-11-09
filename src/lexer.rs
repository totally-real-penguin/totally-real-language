
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

    Int {value: Option<i64>},
    Float {value: Option<f64>},
    String {value: Option<String>},
    Bool {value: Option<bool>},
    None,

    Var, // initialise variable
    Const, // initialise constant
    
    Print,
    If,
    Else,
    ElseIf,

    While,
    For,

    Function,

    Indentifier {name: String, value:Box<TokenTypes>},

    LeftParen, // (
    RightParen, // )

    LeftCurly, // {
    RightCurly, // }

    LeftSquare, // [
    RightSquare, // ],

    Colon, // :
}

pub struct Lexer {
    tokens: Vec<TokenTypes>,
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

    fn is_file_end(&self) -> bool {
        return self.current_pos >= self.contents.len()
    }

    pub fn scan(&mut self) -> &Vec<TokenTypes> {
        while !self.is_file_end() {
            let mut current_token = self.get_next_token();
            match current_token {
                TokenTypes::None => {}
                _ => self.tokens.push(current_token),
            };
            self.current_pos += 1
        }
        return &self.tokens
    }

    pub fn get_next_token(&mut self) -> TokenTypes {
        let mut current_char = self.contents.get(self.current_pos).unwrap();
        match current_char {
            '#' => {
                while *current_char != '\n' || !self.is_file_end() {
                    self.current_pos += 1;
                    current_char = self.contents.get(self.current_pos).unwrap();
            }}

            ':' => return TokenTypes::Colon,

            '(' => return TokenTypes::LeftParen,
            ')' => return TokenTypes::RightParen,

            '[' => return TokenTypes::LeftSquare,
            ']' => return TokenTypes::RightSquare,

            '{' => return TokenTypes::LeftCurly,
            '}' => return TokenTypes::RightCurly,

            '"' => {
                let mut buffer: Vec<char> = Vec::new();
                let mut string_end: bool = false;
                self.current_pos += 1;
                while string_end && !self.is_file_end() {
                    current_char = self.contents.get(self.current_pos).unwrap();
                    if *current_char != '"' {
                        buffer.push(*current_char);
                    } else {
                        string_end = true;
                    }
                    self.current_pos += 1;
                }
                self.current_pos -= 1;
                return TokenTypes::String {value: Some(buffer.iter().collect()) }
            }

            '0'..':' => {
                    let mut buffer: Vec<char> = Vec::new();
                    let mut is_float = false;
                    while ( (*current_char < ':' && *current_char >= '0') || *current_char == '.' ) && !self.is_file_end()  {
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
                        return TokenTypes::Float {value: Some(num.parse::<f64>().unwrap()) }
                    } else {
                        return TokenTypes::Int {value: Some(num.parse::<i64>().unwrap()) }
                    }
                }

            '+' => {
                let next_char = self.scry(1);
                if next_char.is_some() {
                    match next_char.unwrap() {
                        '+' => {
                            self.current_pos += 1;
                            return TokenTypes::Inc;
                        }
                        '=' => {
                            self.current_pos += 1;
                            return TokenTypes::AddAssign;
                        }
                        _ => return TokenTypes::Add
                    }
                } else {
                    return TokenTypes::Add
                }
            }

            '-' => {
                let next_char = self.scry(1);
                if next_char.is_some() {
                    match next_char.unwrap() {
                        '-' => {
                            self.current_pos += 1;
                            return TokenTypes::Dec;
                        }
                        '=' => {
                            self.current_pos += 1;
                            return TokenTypes::SubAssign;
                        }
                        _ => return TokenTypes::Sub
                    }
                } else {
                    return TokenTypes::Sub
                }
            }

            '/' => {
                let next_char = self.scry(1);
                if next_char.is_some() {
                    match next_char.unwrap() {
                        '/' => {
                            self.current_pos += 1;
                            return TokenTypes::IntDiv;
                        }
                        '=' => {
                            self.current_pos += 1;
                            return TokenTypes::DivAssign;
                        }
                        _ => return TokenTypes::Div
                    }
                } else {
                    return TokenTypes::Div
                }
            }

            '*' => {
                let next_char = self.scry(1);
                if next_char.is_some() {
                    match next_char.unwrap() {
                        '*' => {
                            self.current_pos += 1;
                            return TokenTypes::Pow;
                        }
                        '=' => {
                            self.current_pos += 1;
                            return TokenTypes::MultAssign;
                        }
                        _ => return TokenTypes::Mult
                    }
                } else {
                    return TokenTypes::Mult;
                }
            }

            '%' => {
                let next_char = self.scry(1);
                if next_char.is_some() && next_char.unwrap() == '=' { 
                    self.current_pos += 1;
                    return TokenTypes::ModAssign;
                } else {
                    return TokenTypes::Mod;
                }
            }

            '>' => {
                let next_char = self.scry(1);
                if next_char.is_some() && next_char.unwrap() == '=' { 
                    self.current_pos += 1;
                    return TokenTypes::GreaterEqual;
                } else {
                    return TokenTypes::Greater;
                }
            }

            '<' => {
                let next_char = self.scry(1);
                if next_char.is_some() && next_char.unwrap() == '=' {
                    self.current_pos += 1;
                    return TokenTypes::LessEqual;
                } else {
                    return TokenTypes::LessThan;
                }
            }

            '=' => {
                let next_char = self.scry(1);
                if next_char.is_some() && next_char.unwrap() == '=' {
                    self.current_pos += 1;
                    return TokenTypes::Equal;
                } else {
                    return TokenTypes::Assign;
                }
            }

            '!' => {
                let next_char = self.scry(1);
                if next_char.is_some() && next_char.unwrap() == '=' {
                    self.current_pos += 1;
                    return TokenTypes::NotEqual;
                } else {
                    return TokenTypes::Not;
                }
            }

            '&' => {
                let next_char = self.scry(1);
                if next_char.is_some() && next_char.unwrap() == '&' {
                    self.current_pos += 1;
                    return TokenTypes::And;
                } else {
                    return TokenTypes::BitAnd;
                }
            }

            '|' => {
                let next_char = self.scry(1);
                if next_char.is_some() && next_char.unwrap() == '|' {
                    self.current_pos += 1;
                    return TokenTypes::Or;
                } else {
                    return TokenTypes::BitOr;
                }
            }

            '¬' => return TokenTypes::BitNot,

            _ =>  {
                if current_char.is_alphabetic() {
                    let mut buffer: Vec<char> = Vec::new();
                    while current_char.is_alphanumeric() || *current_char == '_' || *current_char == '-' {
                        buffer.push(*current_char);
                        self.current_pos += 1;
                        current_char = self.contents.get(self.current_pos).unwrap();
                    }
                    self.current_pos -= 1;
                    let mut keyword: String = buffer.iter().collect();
                    keyword = keyword.trim().to_string();
                    match &keyword as &str {
                        "true" => return TokenTypes::Bool { value: Some(true) },
                        "false" => return TokenTypes::Bool { value: Some(false) },

                        "if" => return TokenTypes::If,
                        "else" => return TokenTypes::Else,
                        "elif" => return TokenTypes::ElseIf,
                        
                        "fn" => return TokenTypes::Function,

                        "while" => return TokenTypes::While,
                        "for" => return TokenTypes::For,

                        "var" => return TokenTypes::Var,
                        "const" => return TokenTypes::Const,
                        
                        "print" => return TokenTypes::Print,
                        
                        "int" => return TokenTypes::Int { value: None },
                        "float" => return TokenTypes::Float { value: None },
                        "string" => return TokenTypes::String { value: None },
                        "bool" => return TokenTypes::Bool { value: None },

                        _ => return TokenTypes::Indentifier { name: keyword, value: Box::from(TokenTypes::None) },
                    }
                }
            }
        }
        return TokenTypes::None;
        
    }
}
