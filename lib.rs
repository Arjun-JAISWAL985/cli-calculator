#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Num(f64),
    Unary { op: Op, rhs: Box<Expr> },
    Bin { op: Op, lhs: Box<Expr>, rhs: Box<Expr> },
}

// Error types
#[derive(Debug, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq)]
pub enum LexError {
    InvalidChar { ch: char, span: Span },
    InvalidNumber { span: Span },
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    UnexpectedToken { expected: String, found: Token, span: Span },
    UnexpectedEof,
    MismatchedParen { span: Span },
}

#[derive(Debug, PartialEq)]
pub enum EvalError {
    DivisionByZero,
}

// Placeholder functions - we'll implement these step by step
pub fn tokenize(input: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();
    let mut chars = input.char_indices().peekable();
    
    while let Some((start, ch)) = chars.next() {
        match ch {
            ' ' | '\t' | '\n' | '\r' => continue, // Skip whitespace
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Star),
            '/' => tokens.push(Token::Slash),
            '^' => tokens.push(Token::Caret),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '0'..='9' => {
                let mut number = String::new();
                number.push(ch);
                let mut end = start;
                
                // Collect digits and optional decimal point
                while let Some((pos, digit)) = chars.peek() {
                    match digit {
                        '0'..='9' | '.' => {
                            number.push(*digit);
                            end = *pos;
                            chars.next();
                        }
                        _ => break,
                    }
                }
                
                match number.parse::<f64>() {
                    Ok(num) => tokens.push(Token::Number(num)),
                    Err(_) => return Err(LexError::InvalidNumber { 
                        span: Span { start, end: end + 1 } 
                    }),
                }
            }
            _ => return Err(LexError::InvalidChar { 
                ch, 
                span: Span { start, end: start + ch.len_utf8() } 
            }),
        }
    }
    
    Ok(tokens)
}


struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, pos: 0 }
    }
    
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }
    
    fn advance(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pos);
        self.pos += 1;
        token
    }
    
    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_term()?;
        
        while let Some(token) = self.peek() {
            match token {
                Token::Plus => {
                    self.advance();
                    let right = self.parse_term()?;
                    left = Expr::Bin { op: Op::Add, lhs: Box::new(left), rhs: Box::new(right) };
                }
                Token::Minus => {
                    self.advance();
                    let right = self.parse_term()?;
                    left = Expr::Bin { op: Op::Sub, lhs: Box::new(left), rhs: Box::new(right) };
                }
                _ => break,
            }
        }
        
        Ok(left)
    }
    
    fn parse_term(&mut self) -> Result<Expr, ParseError> {
        let mut left = self.parse_power()?;
        
        while let Some(token) = self.peek() {
            match token {
                Token::Star => {
                    self.advance();
                    let right = self.parse_power()?;
                    left = Expr::Bin { op: Op::Mul, lhs: Box::new(left), rhs: Box::new(right) };
                }
                Token::Slash => {
                    self.advance();
                    let right = self.parse_power()?;
                    left = Expr::Bin { op: Op::Div, lhs: Box::new(left), rhs: Box::new(right) };
                }
                _ => break,
            }
        }
        
        Ok(left)
    }
    
    fn parse_power(&mut self) -> Result<Expr, ParseError> {
        let left = self.parse_unary()?;
        
        if let Some(Token::Caret) = self.peek() {
            self.advance();
            let right = self.parse_power()?; // Right associative
            Ok(Expr::Bin { op: Op::Pow, lhs: Box::new(left), rhs: Box::new(right) })
        } else {
            Ok(left)
        }
    }
    
    fn parse_unary(&mut self) -> Result<Expr, ParseError> {
        match self.peek() {
            Some(Token::Plus) => {
                self.advance();
                Ok(Expr::Unary { op: Op::Add, rhs: Box::new(self.parse_unary()?) })
            }
            Some(Token::Minus) => {
                self.advance();
                Ok(Expr::Unary { op: Op::Sub, rhs: Box::new(self.parse_unary()?) })
            }
            _ => self.parse_primary(),
        }
    }
    
    fn parse_primary(&mut self) -> Result<Expr, ParseError> {
        match self.advance() {
            Some(Token::Number(n)) => Ok(Expr::Num(*n)),
            Some(Token::LParen) => {
                let expr = self.parse_expr()?;
                match self.advance() {
                    Some(Token::RParen) => Ok(expr),
                    _ => Err(ParseError::MismatchedParen { 
                        span: Span { start: 0, end: 0 } // Simplified for now
                    }),
                }
            }
            _ => Err(ParseError::UnexpectedEof),
        }
    }
}

pub fn parse(tokens: &[Token]) -> Result<Expr, ParseError> {
    let mut parser = Parser::new(tokens);
    parser.parse_expr()
}


impl Expr {
    pub fn eval(&self) -> Result<f64, EvalError> {
        match self {
            Expr::Num(n) => Ok(*n),
            Expr::Unary { op, rhs } => {
                let val = rhs.eval()?;
                match op {
                    Op::Add => Ok(val),
                    Op::Sub => Ok(-val),
                    _ => unreachable!(),
                }
            }
            Expr::Bin { op, lhs, rhs } => {
                let left = lhs.eval()?;
                let right = rhs.eval()?;
                match op {
                    Op::Add => Ok(left + right),
                    Op::Sub => Ok(left - right),
                    Op::Mul => Ok(left * right),
                    Op::Div => {
                        if right == 0.0 {
                            Err(EvalError::DivisionByZero)
                        } else {
                            Ok(left / right)
                        }
                    }
                    Op::Pow => Ok(left.powf(right)),
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple() {
        let tokens = tokenize("1 + 2").unwrap();
        assert_eq!(tokens, vec![
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.0)
        ]);
    }

    #[test]
    fn test_tokenize_complex() {
        let tokens = tokenize("12.5 * (3 + 4) / 2").unwrap();
        assert_eq!(tokens[0], Token::Number(12.5));
        assert_eq!(tokens[1], Token::Star);
        assert_eq!(tokens[2], Token::LParen);
        // Add more assertions as needed
    }
}
#[test]
fn test_precedence() {
    let tokens = tokenize("2 + 3 * 4").unwrap();
    let expr = parse(&tokens).unwrap();
    let result = expr.eval().unwrap();
    assert_eq!(result, 14.0); // Should be 2 + (3 * 4) = 14
}

#[test]
fn test_parentheses() {
    let tokens = tokenize("(2 + 3) * 4").unwrap();
    let expr = parse(&tokens).unwrap();
    let result = expr.eval().unwrap();
    assert_eq!(result, 20.0); // Should be (2 + 3) * 4 = 20
}


