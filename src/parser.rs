use crate::token::{TokenType::{*, self},Token};
use crate::generateast::*;

pub struct Parser{
    tokens: Vec<Token>,
    current: usize,
}

// macro_rules! match_tokens {
//     ($parser:ident,$($token:ident),+) => {
//         {
//             let mut result = false;
//             {
//                 $(result |= $parser.match_token($token);)*
//             }
//             result
//         }
//     };
// }

impl Parser{
    pub fn new(tokens: Vec<Token>) -> Self{
        Self{
            tokens,
            current : 0,
        }
    }

    pub fn expression(&mut self) -> Expr{
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr : Expr = self.comparison(); 
        while self.match_tokens(&[BANG_EQUAL,EQUAL_EQUAL]) {
            let operator : Token = self.previous();
            let rhs = self.comparison();
            expr = Expr::Binary { left: Box::from(expr), operator: operator, right: Box::from(rhs) };
        }
        expr
    }

    fn comparison(&mut self) -> Expr{
        let mut expr = self.term();
        while self.match_tokens(&[GREATER, GREATER_EQUAL, LESS, LESS_EQUAL]){
            let op = self.previous();
            let rhs = self.term();
            expr = Expr::Binary { left: Box::from(expr), operator: op, right: Box::from(rhs) };
        }
        expr
    }

    fn match_token(&mut self, typ : TokenType) -> bool {
        if self.is_at_end(){
            return false;
        }
        else{
            if self.peek()._type == typ {
                self.advance();
                return true;
            }
        }
        false
    }

    fn match_tokens(&mut self, typs : &[TokenType]) -> bool{
        for typ in typs{
            if self.match_token(*typ){
                return true;
            }
        }
        false
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn advance(&mut self) -> Token{
        let token = self.peek();
        if !self.is_at_end(){
            self.current += 1
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek()._type == EOF
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.match_tokens(&[MINUS,PLUS]){
            let op = self.previous();
            let right = self.factor();
            expr = Expr::Binary { left: Box::from(expr), operator: op, right: Box::from(right) };
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.match_tokens(&[SLASH,STAR]){
            let op = self.previous();
            let right = self.unary();
            expr = Expr::Binary { left: Box::from(expr), operator: op, right: Box::from(right) };
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_tokens(&[BANG, MINUS]){
            let op = self.previous();
            let right = self.unary();
            return Expr::Unary { operator: op, right: Box::from(right) };
        }

        return self.primary();
    }

    fn primary(&mut self) -> Expr {
        if self.match_tokens(&[TokenType::LEFT_PAREN]){
            let expr = self.expression();
            self.consume(TokenType::RIGHT_PAREN, "Expect ')' after expression");
            return Expr::Grouping { expression: Box::from(expr) };
        }
        else{
            let token = self.peek();
            self.advance();
            return Expr::Literal { value: LiteralValue::from_token(token)};
        }
    }

    fn consume(&mut self, right_paren: TokenType, arg: &str) {
        let token = self.peek();
        if token._type == right_paren{
            self.advance();
        }
        else{
            panic!("{:?}",arg)
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_addition(){
        let tokens = vec![Token{_type : TokenType::NUM, lexeme : String::from("1"), literal : crate::token::Literal::NUMBER(1.0), line : 0},
        Token{_type : TokenType::PLUS, lexeme : String::from("+"), literal : crate::token::Literal::NONE, line : 0},
        Token{_type : TokenType::NUM, lexeme : String::from("2"), literal : crate::token::Literal::NUMBER(2.0), line : 0},
        Token{_type : TokenType::SEMICOLON, lexeme : String::from(";"), literal : crate::token::Literal::NONE, line : 0}];
        let mut parser = Parser::new(tokens);
        let parser_expr = parser.expression();
        let string_expr = parser_expr.to_string();
        println!("{:?}", string_expr);
    }
}