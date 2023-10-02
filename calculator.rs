use std::io;

fn main() {
    println!("Welcome to the Rust Calculator!");
    println!("Available operations: +, -, *, /");

    loop {
        println!("Enter an operation symbol or 'q' to quit:");

        let mut operation = String::new();
        io::stdin().read_line(&mut operation).expect("Failed to read line");

        let operation = operation.trim();

        if operation == "q" {
            println!("Goodbye!");
            break;
        }

        if operation != "+" && operation != "-" && operation != "*" && operation != "/" {
            println!("Invalid operation symbol. Please enter +, -, *, /, or 'q' to quit.");
            continue;
        }

        let mut num1 = String::new();
        let mut num2 = String::new();

        println!("Enter the first number:");
        io::stdin().read_line(&mut num1).expect("Failed to read line");
        let num1: f64 = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        };

        println!("Enter the second number:");
        io::stdin().read_line(&mut num2).expect("Failed to read line");
        let num2: f64 = match num2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        };

        let result = match operation {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 != 0.0 {
                    num1 / num2
                } else {
                    println!("Division by zero is not allowed.");
                    continue;
                }
            }
            _ => {
                println!("Invalid operation symbol. Please enter +, -, *, /, or 'q' to quit.");
                continue;
            }
        };

        println!("Result: {}", result);
    }
}
