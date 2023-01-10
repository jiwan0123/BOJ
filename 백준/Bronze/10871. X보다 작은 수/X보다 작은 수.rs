use std::io;
fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();

    let mut b = String::new();
    io::stdin().read_line(&mut b).unwrap();

    let first: Vec<usize> = a.split_whitespace().map(|a| a.parse().unwrap()).collect();
    let second: Vec<usize> = b.split_whitespace().map(|b| b.parse().unwrap()).collect();

    let num: usize = first[0] as usize;
    for i in 0..num {
        if second[i] < first[1] {
            print!("{} ", second[i]);
        }
    }
}