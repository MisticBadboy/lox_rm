use std::any::TypeId;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN, RIGHT_PAREN, LEFT_BRACE, RIGHT_BRACE,
    COMMA, DOT, MINUS, PLUS, SEMICOLON, SLASH, STAR,
    // One or two character tokens.
    BANG, BANG_EQUAL,
    EQUAL, EQUAL_EQUAL,
    GREATER, GREATER_EQUAL,
    LESS, LESS_EQUAL,
    // Literals.
    IDENTIFIER, STR, NUM,
    // Keywords.
    AND, CLASS, ELSE, FALSE, FUN, FOR, IF, NIL, OR,
    PRINT, RETURN, SUPER, THIS, TRUE, VAR, WHILE,
    EOF
}
#[derive(Debug, Clone, PartialEq)]
pub enum Literal{
    STRING(String), NUMBER(f32), NONE
}

impl Literal{
    pub fn unwrap_as_f32(lit : Self) -> f32{
        match lit {
            Self::NUMBER(x) => x,
            _ => panic!("Could't convert {:?} to f32",lit )
        }
    }

    pub fn unwrap_as_String(lit : Self) -> String{
        match lit{
            Self::STRING(x) => x,
            _ => panic!("Couldn't convert {:?} to String", lit)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token{
    pub _type : TokenType,
    pub lexeme : String,
    pub literal : Literal,
    pub line : i32
}

impl Token {
    pub fn new(_type: TokenType, lexeme: String, literal: Literal, line: i32) -> Self { Self { _type, lexeme, literal, line } }
    
    pub fn toString(&self) -> String{
        format!("{:?} {:?} {:?}", self._type, self.lexeme, self.literal)
    }

}