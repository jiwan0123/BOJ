use std::io;
fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();

    for i in 1..=n {
        println!("{:*>i$}", "*");
    }
}