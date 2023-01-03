use std::io;
fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();
    let test: String = std::iter::repeat(" ").take(n).collect();

    for i in (0..n).rev() {
        println!("{:*<n$}", &test[0..i]);
    }
}