use std::io;

fn main() {
    let mut a = String::new();

    io::stdin().read_line(&mut a).unwrap();

    let numbers: Vec<&str> = a.split_whitespace().collect();

    let king = numbers[0].parse::<i32>().unwrap();
    let queen = numbers[1].parse::<i32>().unwrap();
    let rook = numbers[2].parse::<i32>().unwrap();
    let bishop = numbers[3].parse::<i32>().unwrap();
    let knight = numbers[4].parse::<i32>().unwrap();
    let pawn = numbers[5].parse::<i32>().unwrap();

    println!(
        "{} {} {} {} {} {}",
        1 - king,
        1 - queen,
        2 - rook,
        2 - bishop,
        2 - knight,
        8 - pawn
    );
}
