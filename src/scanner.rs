use crate::token;

use std::fmt;
use std::fmt::*;
extern crate phf;
use phf::*;

use token::TokenType::*;
use token::Literal::*;

use self::token::TokenType;

static Keywords : phf::Map<&'static str,TokenType> = phf_map! {
    "and"  => AND,
    "class"    => CLASS,
    "else" => ELSE,
    "false"    => FALSE,
    "for"  => FOR,
    "fun"  => FUN,
    "if"   => IF,
    "nil"  => NIL,
    "or"   => OR,
    "print"    => PRINT,
    "return"   => RETURN,
    "super"    => SUPER,
    "this" => THIS,
    "true" => TRUE,
    "var"  => VAR,
    "while"    => WHILE
};

#[derive(Debug)]
pub struct Scanner{
    source : String,
    tokens : Vec<token::Token>,
    start : usize,
    current : usize,
    _line : usize
}

impl Display for Scanner{
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.tokens)
    }
}

impl Scanner {
    
    pub fn new(source: String) -> Self { Self { source, tokens : Vec::<token::Token>::new(), start : 0, current : 0, _line : 1 } }
    
    pub fn scanTokens(&mut self) -> &Vec<token::Token> {
        while !self.isAtEnd(){
            self.start = self.current;
            self.scanToken()   
        }
        self.tokens.push(token::Token::new(EOF,String::from(""),NONE,self._line as i32));
        &self.tokens
    }
    
    fn isAtEnd(&self) -> bool{
        return self.current >= self.source.len();
    }

    fn scanToken(&mut self) {
        let c : char = self.advance();
        match c {
            '(' => self.addToken(LEFT_PAREN) ,
            ')' => self.addToken(RIGHT_PAREN) ,
            '{' => self.addToken(LEFT_BRACE) ,
            '}' => self.addToken(RIGHT_BRACE) ,
            ',' => self.addToken(COMMA) ,
            '.' => self.addToken(DOT) ,
            '-' => self.addToken(MINUS) ,
            '+' => self.addToken(PLUS) ,
            ';' => self.addToken(SEMICOLON) ,
            '*' => self.addToken(STAR),
            '!' => { if self.omatch('=') {  
                        self.addToken(BANG_EQUAL);
                    } 
                    else {
                        self.addToken(BANG)
                    }  
                    self.current += 1 ; 
                },
            '<' => { if self.omatch('=') {  
                        self.addToken(LESS_EQUAL)
                    }
                    else {
                        self.addToken(LESS)
                    }  
                    self.current += 1 ; 
                },
            '>' => { if self.omatch('='){
                        self.addToken(GREATER_EQUAL); 
                    }   
                    else {
                        self.addToken(GREATER)
                    }  
                    self.current += 1 ; 
                },
            '=' => { if self.omatch('=') {
                        self.addToken(EQUAL_EQUAL) 
                    }
                    else {
                        self.addToken(EQUAL)
                    }  
                    self.current += 1 ; 
                },
            '/' => { if self.omatch('/'){
                        while self.peek() != '\n' && !self.isAtEnd() {self.advance();}
                    }
                    else {
                        self.addToken(SLASH)
                    }
                },
            ' ' => {},
            '\r' => {},
            '\t' => {},
            '\n' => {
                self._line += 1;
            }
            '"' => {self.string()},
            'o' => {
                if self.peek() == 'r'{
                    self.addToken(OR)
                }
            },
            _   =>  {if self.isDigit(c) {
                        self.number();
                    } 
                    else if self.isAlpha(c){
                       self.identifier(); 
                    }
                    else {
                        crate::error(self._line as i32, "Unexpected character.".to_string())
                    }
                }         
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.isAtEnd(){
            if self.peek() == '\n' {
                self._line += 1;
            }
            let c = self.advance();
        }
        if self.isAtEnd(){
            crate::error(self._line as i32, ("Unterminated string.").to_string());
            return;
        }
        let c = self.advance();

        let value: String = self.source[self.start + 1..self.current - 1].to_string();
        self.add_Token(STR, STRING(value));
    }

    fn omatch(&self, expected : char) -> bool {
        let c = self.source.chars().nth(self.current).unwrap();
        if self.isAtEnd(){return false;};
        if c != expected { return false;};
        return true;
    }

    fn peek(&self) -> char{
        if self.isAtEnd() {return '\0';}
        return self.source.chars().nth(self.current).unwrap();
    }

    fn addToken(&mut self, __type : token::TokenType){
        self.add_Token(__type, NONE)
    }

    fn add_Token(&mut self, __type : token::TokenType, _literal : token::Literal){
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(token::Token::new(__type, text, _literal, self._line as i32))
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn isDigit(&self, c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    fn number(&mut self) {
        while self.isDigit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.isDigit(self.peekNext()){
            self.advance();
            while self.isDigit(self.peek()){
                self.advance();
            }
        }

        self.add_Token(NUM, NUMBER(self.source[self.start..self.current].parse::<f32>().unwrap()))
    }

    fn peekNext(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        return self.source.chars().nth(self.current + 1).unwrap();
    }

    fn isAlpha(&self, c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }

    fn identifier(&mut self) {
        while self.isAlphaNumeric(self.peek()) {
            self.advance();
        }
        let text = &self.source[self.start..self.current];
        let _type = match Keywords.get(text.to_ascii_lowercase().as_str()){
            Some(__type) => __type,
            None => &IDENTIFIER 
        };
        self.addToken(*_type);
    }

    fn isAlphaNumeric(&self, peek: char) -> bool {
        return self.isAlpha(peek) || self.isDigit(peek);
    } 
}
