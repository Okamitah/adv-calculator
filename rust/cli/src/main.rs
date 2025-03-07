use std::{io, collections::HashMap};

fn main() {
    println!("Welcome to my basic calculator!");
    //make_operation();
    add_whole_expression_at_once();
}

fn make_operation() {

    let mut map: HashMap<&str, Box<dyn Fn(f64, f64) -> f64>> = HashMap::new();

    map.insert("+", Box::new(|x,y| x+y));
    map.insert("-", Box::new(|x,y| x-y));
    map.insert("*", Box::new(|x,y| x*y));
    map.insert("/", Box::new(|x,y| x/y));

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    let mut a:f64 = 0.0;
    let mut b:f64 = 0.0;

    println!("Please enter the first int: ");
    io::stdin().read_line(&mut input1).expect("Couldn't read the line");
    let input1 = input1.trim();
    match input1.parse::<f64>() {
        Ok(num) => a = num,
        Err(_) => println!("Invalid input1"),
    }

    println!("Enter the second nbr: ");
    io::stdin().read_line(&mut input2).expect("Couldn't read the line");
    let input2 = input2.trim();
    match input2.parse::<f64>() {
        Ok(num) => b = num, 
        Err(_) => println!("Invalid input"),
    }
    println!("Enter which operation: ");
    io::stdin().read_line(&mut input3).expect("Couldn't read the operation");
    let input3 = input3.trim();
    if let Some(operation) = map.get(input3) {
        let result = operation(a, b);
        println!("The result of {} {} {} is {}", a, input3, b, result);
    } else {
        println!("Invalid operation. Please enter + or -");
    }
}

fn add_whole_expression_at_once() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Couldn't read the expression");
    let input: String = input.chars().filter(|c| !c.is_whitespace()).collect();
    let result: Vec<&str> = input.split('+').collect();
    println!("split res: {:?}", result);
    let mut sum: f64 = 0.0;
    for nbr in result.iter() {
        sum += match nbr.parse::<f64>() {
            Ok(num) => num,
            Err(_) => panic!("Not a number dude"),
        }
    }
    println!("sum: {}", sum)
    
}
