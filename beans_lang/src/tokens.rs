use std::string::String;
use std::collections::HashMap;
#[derive(PartialEq, PartialOrd, Copy, Clone, Debug)]
pub enum TokenType { 
Colon,
Function,
Less,
Elif,
Do,
Semicolon,
Return,
Star,
False,
Dot,
LessEquals,
Ampersand,
MoreEquals,
Eof,
Break,
Then,
LessLess,
RightParen,
Comma,
Struct,
Plus,
And,
EqualsEquals,
Str,
Import,
More,
MoreMore,
Nil,
Var,
If,
Slash,
In,
Minus,
Pass,
LeftParen,
Lambda,
RightBrace,
Or,
End,
Continue,
RightSquare,
Num,
Enum,
LeftSquare,
BangEquals,
Pipe,
While,
Else,
For,
Not,
Identifier,
LeftBrace,
Equals,
True,

}

lazy_static! {
        pub static ref TOKENS_MAP : HashMap<String, TokenType> = {
        let mut m = HashMap::new();
        m.insert(String::from("function"), TokenType::Function);
        m.insert(String::from("elif"), TokenType::Elif);
        m.insert(String::from("do"), TokenType::Do);
        m.insert(String::from("return"), TokenType::Return);
        m.insert(String::from("false"), TokenType::False);
        m.insert(String::from("break"), TokenType::Break);
        m.insert(String::from("then"), TokenType::Then);
        m.insert(String::from("struct"), TokenType::Struct);
        m.insert(String::from("and"), TokenType::And);
        m.insert(String::from("import"), TokenType::Import);
        m.insert(String::from("nil"), TokenType::Nil);
        m.insert(String::from("var"), TokenType::Var);
        m.insert(String::from("if"), TokenType::If);
        m.insert(String::from("in"), TokenType::In);
        m.insert(String::from("pass"), TokenType::Pass);
        m.insert(String::from("lambda"), TokenType::Lambda);
        m.insert(String::from("or"), TokenType::Or);
        m.insert(String::from("end"), TokenType::End);
        m.insert(String::from("continue"), TokenType::Continue);
        m.insert(String::from("enum"), TokenType::Enum);
        m.insert(String::from("while"), TokenType::While);
        m.insert(String::from("else"), TokenType::Else);
        m.insert(String::from("for"), TokenType::For);
        m.insert(String::from("not"), TokenType::Not);
        m.insert(String::from("true"), TokenType::True);

        m
    };
}

#[derive(Debug)]
pub enum Value {
    Empty,
    Num(f64),
    Str(String),
    Id(String),
}

#[derive(Debug)]
pub struct Token {
    token_type : TokenType,
    line: u16,
    val: Value,
}

impl Token{
    pub fn new(token_type : TokenType, line : u16, val : Value) -> Token {
        Token {
            token_type : token_type,
            line : line,
            val : val,
        }
    }

    pub fn get_val(&self) -> &Value {
        &self.val
    }

    pub fn get_type(&self) -> TokenType {
        self.token_type
    }

    pub fn get_line(&self) -> u16 {
        self.line
    }

    pub fn as_f64(&self) -> f64 {
        match self.val {
            Value::Num(n) => n,
            _ => panic!("Falied converting Value to f64!")
        }
    }

    pub fn as_String(&self) -> String {
        match &self.val {
            Value::Str(s) => s.clone(),
            _ => panic!("Falied converting Value to String!")
        }
    }

    pub fn as_Id(&self) -> String {
        match &self.val {
            Value::Id(s) => s.clone(),
            _ => panic!("Falied converting Value to Id!")
        }
    }
}