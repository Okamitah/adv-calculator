use std::{io, collections::HashMap};

fn main() {
    make_operation();
}

fn make_operation() {

    let mut map: HashMap<&str, Box<dyn Fn(i32, i32) -> i32>> = HashMap::new();

    map.insert("+", Box::new(|x,y| x+y));
    map.insert("-", Box::new(|x,y| x-y));
    map.insert("*", Box::new(|x,y| x*y));
    map.insert("/", Box::new(|x,y| x/y));

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    let mut a = 0;
    let mut b = 0;

    println!("Please enter the first int: ");
    io::stdin().read_line(&mut input1).expect("Couldn't read the line");
    let input1 = input1.trim();
    match input1.parse::<i32>() {
        Ok(num) => {
            a = num;
            println!("Your number is: {}", a);
        }
        Err(_) => {
            println!("Invalid input1");
        }
    }

    println!("Enter the second nbr: ");
    io::stdin().read_line(&mut input2).expect("Couldn't read the line");
    let input2 = input2.trim();
    match input2.parse::<i32>() {
        Ok(num) => {
            b = num;
            println!("Your 2nd number is: {}", b);
        }
        Err(_) => {println!("Invalid input")}
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

