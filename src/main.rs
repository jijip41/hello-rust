use std::env::{args, Args};

fn main() {
    let a = 100;
    let b = 200;
    println!("Result is {}",a+b);

    let arr:[i32;4] = [1, 2, 3, 4];
    println!("array size is {}", arr.len());

    let mut args: Args = args();
    let first: String = args.nth(1).unwrap();
    let operator: char = args.nth(0).unwrap().chars().next().unwrap();
    let second: String = args.nth(0).unwrap();

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();
    let result = operate(operator, first_number, second_number);

    println!("{}", output(first_number, operator, second_number, result));
}

fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
    format!(
        "{} {} {} = {}", first_number, operator, second_number, result
    )
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    match operator {
        '+' => first_number + second_number, 
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        '*' | 'x' | 'X' => first_number * second_number,
        _ => panic!("Invalid operator used.") 
    }
}