use std::io;

fn main() {
    println!("Welcome to my basic calculator!");
    let expression = start_calculator();
    let xp = separate_expression(&expression);
    println!("{:?}", tokenize(xp));
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
    Plus,
    Minus,
    Multiply,
    Divide,
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
            "(" => tokens.push(Token::LeftParen),
            ")" => tokens.push(Token::RightParen),
            _ => {
                match element.parse::<f64>() {
                    Ok(num) => tokens.push(Token::Number(num)),
                    Err(_) => panic!("Invalid Token: {}", element)
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
            xp.push(side.trim().to_string());
            side = String::new();
            xp.push(char.to_string());
        }
        else {
            side.push(char);
        }
    }
    if !side.is_empty() {xp.push(side.trim().to_string())}
    xp
}
