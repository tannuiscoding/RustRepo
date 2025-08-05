mod calculator;
use std::io;

fn main() {
    println!("welcome to the calculator");
    println!("Pick what do you want to do to the numbers");
    println!("1. Add");
    println!("2. Sub");
    println!("3. Mul");
    println!("4. Div");
    println!("5. Exit");
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read");

    let mut num1_str = String::new();
    let mut num2_str = String::new();

    println!("Enter the first number: ");
    io::stdin().read_line(&mut num1_str).expect("Failed to read");
    let num1: i32 = num1_str.trim().parse().expect("Please enter a valid number");

    println!("Enter the second number: ");
    io::stdin().read_line(&mut num2_str).expect("Failed to read");
    let num2: i32 = num2_str.trim().parse().expect("Please enter a valid number");

    if choice.trim() == "1" {
        println!("You chose to add");
        let result = calculator::add(num1, num2);
        println!("The result is: {}", result);
    } else if choice.trim() == "2" {
        println!("You chose to sub");
        let result = calculator::sub(num1, num2);
        println!("The result is: {}", result);
    } else if choice.trim() == "3" {
        println!("You chose to mul");
        let result = calculator::mul(num1, num2);
        println!("The result is: {}", result);
    } else if choice.trim() == "4" {
        println!("You chose to div");
        let result = calculator::div(num1, num2);
        println!("The result is: {}", result);
    } else if choice.trim() == "5" {
        println!("You chose to exit");
        return;
    } else {
        println!("Invalid choice");
        println!("Please try again");
        return;
    }
}
