use std::io;

fn main() {
    // Prompt the user for the first number.
    println!("Please enter first number:");

    let mut num1 = String::new();

    io::stdin().read_line(&mut num1)
        .expect("Failed to read the line.");

    let num1: i32 = num1.trim().parse()
        .expect("Please enter a valid number.");

    println!("");

    // Prompt the user for the second number.
    println!("Please enter second number:");

    let mut num2 = String::new();

    io::stdin().read_line(&mut num2)
        .expect("Failed to read the line.");

    let num2: i32 = num2.trim().parse()
        .expect("Please enter a valid number.");

    println!("");

    // List all the available operators to select one.
    let operators = ["Add", "Subtract", "Multiply", "Divide"];

    for (num, operator) in operators.iter().enumerate() {
        println!("{}: {}", num + 1, operator);
    }

    println!("");

    // Prompt the user for an operator.
    println!("Please select an operator:");

    let mut operator = String::new();

    io::stdin().read_line(&mut operator)
        .expect("Failed to read the line.");

    let operator = operator.trim();

    // Calculate the result of two numbers.
    let mut result: i32 = 0;

    match operator {
        "Add" => result = add(num1, num2),
        "Subtract" => result = subtract(num1, num2),
        "Multiply" => result = multiply(num1, num2),
        "Divide" => result = divide(num1, num2),
        _ => println!("Sorry, invalid operator selected."),
    };

    println!("");

    println!("Result for operator '{}' of numbers {} and {} is: {}", operator, num1, num2, result);
}

fn add(num: i32, num2: i32) -> i32 {
    num + num2
}

fn subtract(num: i32, num2: i32) -> i32 {
    num - num2
}

fn multiply(num: i32, num2: i32) -> i32 {
    num * num2
}

fn divide(num: i32, num2: i32) -> i32 {
    num / num2
}