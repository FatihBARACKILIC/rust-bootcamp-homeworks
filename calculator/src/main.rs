use std::io::stdin;

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

fn main() {
    let mut input: String = String::new();

    println!("Please enter first number: ");
    stdin()
        .read_line(&mut input)
        .expect("Please enter a float number!");
    let first_number: f64 = input.trim().parse().expect("Please enter a float number!");
    input.clear();

    println!("Please enter an operation (+, -, *, /):");
    stdin()
        .read_line(&mut input)
        .expect("Please enter (+, -, *, /)");
    let operation_type: char = input
        .trim()
        .chars()
        .next()
        .expect("Please enter (+, -, *, /)");
    input.clear();

    println!("Please enter second number:");
    stdin()
        .read_line(&mut input)
        .expect("Please enter a float number!");
    let second_number: f64 = input.trim().parse().expect("Please enter a float number!");
    input.clear();

    let operation = match operation_type {
        '+' => Operation::Add(first_number, second_number),
        '-' => Operation::Subtract(first_number, second_number),
        '*' => Operation::Multiply(first_number, second_number),
        '/' => Operation::Divide(first_number, second_number),
        _ => panic!("please fill right!"),
    };

    let result = calculate(operation);
    println!(
        "{} {} {} = {}",
        first_number, operation_type, second_number, result
    );
}

fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(n1, n2) => n1 + n2,
        Operation::Subtract(n1, n2) => n1 - n2,
        Operation::Multiply(n1, n2) => n1 * n2,
        Operation::Divide(n1, n2) => {
            if n2 != 0.0 {
                n1 / n2
            } else {
                panic!("$n1 cannot divide to 0!");
            }
        }
    }
}
