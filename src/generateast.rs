use crate::token::*;

pub enum LiteralValue {
    NUMBER(f32),
    STRING(String),
    True,
    False,
    Nil
}

pub enum Expr{
    Binary { left: Box<Expr>, operator : Token, right: Box<Expr>},
    Grouping {expression : Box<Expr>},
    Literal {value : LiteralValue},
    Unary {operator : Token, right : Box<Expr>}
}

impl LiteralValue {
    fn to_string(&self) -> String{
        match self {
            LiteralValue::NUMBER(x) => x.to_string(),
            LiteralValue::STRING(x) => x.clone(),
            LiteralValue::False => String::from("false"),
            LiteralValue::True => String::from("true"),
            LiteralValue::Nil => String::from("nil")

        }
    }

    pub fn from_token(token : Token) -> LiteralValue{
        match token._type{
            TokenType::NUM =>   LiteralValue::NUMBER(Literal::unwrap_as_f32(token.literal)),
            TokenType::STR =>   LiteralValue::STRING(Literal::unwrap_as_String(token.literal)),
            TokenType::TRUE =>  LiteralValue::True,
            TokenType::FALSE => LiteralValue::False,
            TokenType::NIL =>   LiteralValue::Nil,
            _ => panic!("Could not create LiteralValue from {:?}", token)
        }
    }
}

impl Expr{
    pub fn to_string(&self) -> String{
        match self{
            Expr::Binary {left, operator, right } => {
                format!("({} {} {})", operator.lexeme, left.to_string(), right.to_string())
            },
            Expr::Grouping { expression } => {
                format!("(group {})", expression.to_string())
            },
            Expr::Literal { value } => {
                format!("{}", value.to_string())
            },
            Expr::Unary { operator, right } => {
                let operator_str = operator.lexeme.clone();
                let right_str = (*right).to_string();
                format!("({} {})", operator.lexeme, right_str)
            }
        }
    }

    pub fn print(&self){
        println!("{:?}",self.to_string());
    }
}

#[cfg(test)]
mod tests{
    use crate::token::Literal;
    use super::*;
    #[test]
    fn pretty_print_est(){
        let minus_token = Token::new(TokenType::MINUS,String::from("-"), Literal::NONE, -1);
        let onetwothree = Box::from(Expr::Literal {value : LiteralValue::NUMBER(123.00)});
        let group = Expr::Grouping { expression : Box::from(Expr::Literal { value: LiteralValue::NUMBER(45.67)})}; 
        let multi = Token::new(TokenType::STAR, String::from("*"), Literal::NONE, -1);
        let ast = Expr::Binary{left : Box::from(Expr::Unary{operator : minus_token, right : onetwothree}), operator : multi, right : Box::from(group)};
        ast.print();
    }

}