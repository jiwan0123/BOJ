use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("error");

    let resulta: String = input.trim().parse().unwrap();

    let resultb: String = resulta + "??!";
    println!("{}", resultb);
}