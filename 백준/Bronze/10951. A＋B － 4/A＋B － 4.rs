use io::Write;
use std::io;
fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    loop {
        let mut a = String::new();
        io::stdin().read_line(&mut a).unwrap();
        let num: Vec<i32> = a
            .as_mut_str()
            .split_whitespace()
            .map(|a| a.parse().unwrap())
            .collect();
        match num.is_empty() {
            true => break,
            false => writeln!(out, "{}", num[0] + num[1]).unwrap(),
        }
    }
}