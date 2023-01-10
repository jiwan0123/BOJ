use std::io;
fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();

    let mut b = String::new();
    io::stdin().read_line(&mut b).unwrap();

    let b: Vec<i32> = b.split_whitespace().map(|b| b.parse().unwrap()).collect();
    let max = b.iter().copied().max().unwrap();
    let min = b.iter().copied().min().unwrap();

    print!("{} {}", min, max);
}