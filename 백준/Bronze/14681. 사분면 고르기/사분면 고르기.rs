use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();

    io::stdin().read_line(&mut a).unwrap();
    io::stdin().read_line(&mut b).unwrap();

    let anum: i32 = a.trim().parse().unwrap();
    let bnum: i32 = b.trim().parse().unwrap();

    let result: i32 = match (anum < 0, bnum < 0) {
        (false, false) => 1,
        (true, false) => 2,
        (true, true) => 3,
        (false, true) => 4,
    };
    print!("{}", result)
}