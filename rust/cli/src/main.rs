use std::{f64::consts, io};

fn main() {
    println!("Welcome to my basic calculator!");
    loop {
        let expression = start_calculator();
        let xp = separate_expression(&expression);
        println!("{:?}", evaluate(shunting_yard(tokenize(xp))));
        
    }
}

fn start_calculator() -> String {
    println!("Enter your expression: ");
    let mut expression = String::new();
    io::stdin().read_line(&mut expression).expect("Failed to read the line :(");
    expression
}

#[derive(Debug, PartialEq)]
enum Token {
    Number(f64),
    E,
    Pi,
    Plus,
    Minus,
    Multiply,
    Divide,
    Exponent,
    Derive,
    Integrate,
    Log,
    Ln,
    Exp,
    Cos,
    Sin,
    Tan,
    Ch,
    Sh,
    Th,
    Acos,
    Asin,
    Atan,
    LeftParen,
    RightParen
}

fn tokenize(expr_vec : Vec<String>) -> Vec<Token> {
    let mut tokens = Vec::new();

    for element in expr_vec.iter() {
        match element.as_str() {
            "+" => tokens.push(Token::Plus),
            "-" => tokens.push(Token::Minus),
            "*" => tokens.push(Token::Multiply),
            "/" => tokens.push(Token::Divide),
            "^" => tokens.push(Token::Exponent),
            "(" => tokens.push(Token::LeftParen),
            ")" => tokens.push(Token::RightParen),
            "log" => tokens.push(Token::Log),
            "ln" => tokens.push(Token::Ln),
            "exp" => tokens.push(Token::Exp),
            "cos" => tokens.push(Token::Cos),
            "sin" => tokens.push(Token::Sin),
            "tan" => tokens.push(Token::Tan),
            "ch" => tokens.push(Token::Ch),
            "sh" => tokens.push(Token::Sh),
            "th" => tokens.push(Token::Th),
            "acos" => tokens.push(Token::Acos),
            "asin" => tokens.push(Token::Asin),
            "atan" => tokens.push(Token::Atan),
            "d" => tokens.push(Token::Derive),
            "i" => tokens.push(Token::Integrate),
            "e" => tokens.push(Token::E),
            "pi" => tokens.push(Token::Pi),
            _ => {
                match element.parse::<f64>() {
                    Ok(num) => tokens.push(Token::Number(num)),
                    Err(_) => println!("Invalid Token: {}", element)
                }
            }
        }
    }
    tokens
}

fn separate_expression(expression: &str)-> Vec<String> {
    let mut xp : Vec<String> = Vec::new();
    let mut side = String::new();
    let operators = ['+','-','*','/','(',')','^','%'];
    for char in expression.chars() {
        if operators.contains(&char) {
            if !side.trim().is_empty() {
                xp.push(side.trim().to_string());
            }
            side = String::new();
            xp.push(char.to_string());
        }
        else {
            side.push(char);
        }
    }
    if !side.trim().is_empty() { xp.push(side.trim().to_string()); }
    xp
}

fn precedence(token: &Token) -> u8 {
    let p = match token {
        Token::Minus | Token::Plus => 1,
        Token::Multiply | Token::Divide => 2,
        Token::Exponent => 3,
        Token::Log | Token::Ln | Token::Exp | Token::Cos | Token::Sin| Token::Tan |Token::Ch | Token::Sh | Token::Th |Token::Acos| Token::Asin | Token::Atan => 4,
        Token::Derive | Token::Integrate => 5,
        _ => 0
    };
    p
}

fn shunting_yard(tokens: Vec<Token>) -> Vec<Token> {
    let mut output = Vec::new();
    let mut operators = Vec::new();
    for token in tokens {
        match token {
            Token::Number(_) | Token::E | Token::Pi => output.push(token),
            Token::Plus | Token::Minus | Token::Multiply | Token::Divide | Token::Exponent => {
                while let Some(op) = operators.last() {
                    if precedence(op) >= precedence(&token) {
                        output.push(operators.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operators.push(token);
            }
            Token::Log | Token::Ln | Token::Exp | Token::Cos | Token::Sin |
            Token::Tan | Token::Ch | Token::Sh | Token::Th | Token::Acos | 
            Token::Asin | Token::Atan | Token::Derive | Token::Integrate => {
                while let Some(op) = operators.last() {
                    if precedence(op) > precedence(&token) {
                        output.push(operators.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operators.push(token);
            }
            Token::LeftParen => operators.push(token),
            Token::RightParen => {
                while let Some(op) = operators.pop() {
                    if op == Token::LeftParen {
                        break;
                    }
                    output.push(op);
                }
            }
        }
    }

    while let Some(op) = operators.pop() {
        output.push(op);
    }
    output
}

fn evaluate(postfix: Vec<Token>) -> f64 {
    let mut stack = Vec::new();
    
    for token in postfix {
        match token {
            Token::Number(num) => stack.push(num),
            Token::E => stack.push(consts::E),
            Token::Pi => stack.push(consts::PI),
            Token::Plus => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(y+x);
            }
            Token::Minus => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(y-x);
            }
            Token::Multiply => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(y*x);
            }
            Token::Divide => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                if y == 0.0 {panic!("Division by 0!");}
                stack.push(y/x);
            }
            Token::Exponent => {
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(f64::powf(y,x));
            }
            Token::Log => {
                let x = stack.pop().unwrap();
                if x <= 0.0 {
                    panic!("Logarithm of non-positive number!");
                }
                stack.push(f64::log(x, 10.0));
            }
            Token::Ln => {
                let x = stack.pop().unwrap();
                if x <= 0.0 {
                    panic!("Logarithm of non-positive number!");
                }
                stack.push(x.ln());
            }
            Token::Exp => {
                let x = stack.pop().unwrap();
                stack.push(f64::exp(x));
            }
            Token::Cos => {
                let x = stack.pop().unwrap();
                stack.push(f64::cos(x));
            }
            Token::Sin => {
                let x = stack.pop().unwrap();
                stack.push(f64::sin(x));
            }
            Token::Tan => {
                let x = stack.pop().unwrap();
                stack.push(f64::tan(x));
            }
            Token::Ch => {
                let x = stack.pop().unwrap();
                stack.push(f64::exp(x));
            }
            Token::Sh => {
                let x = stack.pop().unwrap();
                stack.push(f64::exp(x));
            }
            Token::Th => {
                let x = stack.pop().unwrap();
                stack.push(f64::exp(x));
            }
            Token::Acos => {
                let x = stack.pop().unwrap();
                stack.push(f64::exp(x));
            }
            Token::Asin => {
                let x = stack.pop().unwrap();
                stack.push(f64::exp(x));
            }
            Token::Atan => {
                let x = stack.pop().unwrap();
                stack.push(f64::exp(x));
            }

            _ => println!("Unexpected token")
        }
    }

    let multiplier = 10f64.powi(7 as i32);
    (stack.pop().unwrap() * multiplier).round() / multiplier
}
