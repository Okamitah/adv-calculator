use std::io;

fn main() {
    println!("Welcome to my basic calculator!");
    let expression = start_calculator();
    let xp = separate_expression(&expression);
    println!("{:?}", xp);
}

fn start_calculator() -> String {
    println!("Enter your expression: ");
    let mut expression = String::new();
    io::stdin().read_line(&mut expression).expect("Failed to read the line :(");
    expression
}

fn separate_expression(expression: &str)-> Vec<String> {
    let mut xp : Vec<String> = Vec::new();
    let mut side = String::new();
    let operators = ['+','-','*','-','(',')','^','%'];
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
    xp
}
