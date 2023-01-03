use io::Write;
use std::io;
fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let num: i32 = a.trim().parse().unwrap();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    for _ in 0..num {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let result: Vec<i32> = input
            .as_mut_str()
            .split_whitespace()
            .map(|input| input.parse().unwrap())
            .collect();
        writeln!(out, "{}", result[0] + result[1]).unwrap();
    }
}