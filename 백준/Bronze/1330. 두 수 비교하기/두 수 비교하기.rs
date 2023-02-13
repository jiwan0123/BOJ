use std::io;
use std::cmp::Ordering;

fn main() {
    let mut input_number = String::new();
    
    io::stdin().read_line(&mut input_number).expect("fuck error");
    
    let numbers:Vec<&str> = input_number.split_whitespace().collect();
    
    let number_a = match numbers[0].parse::<i32>() {
        Ok(i) => i,
        Err(_e) => {
            -1
        }
    };
    
    let number_b = match numbers[1].parse::<i32>() {
        Ok(i) => i,
        Err(_e) => {
            -1
        }
    };
    
    if number_a > number_b {
        println!(">");
    }
    else if number_a < number_b {
        println!("<");
    }
    else if number_a == number_b {
        println!("==")
    }
}