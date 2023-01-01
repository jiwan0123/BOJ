use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();

    io::stdin().read_line(&mut a).unwrap();
    io::stdin().read_line(&mut b).unwrap();

    let anum: i32 = a.trim().parse().unwrap();
    let bnum: i32 = b.trim().parse().unwrap();

    let b_one: i32 = bnum % 10;
    let b_ten: i32 = (bnum % 100 - b_one) / 10;
    let b_hund: i32 = (bnum - b_ten * 10 - b_one) / 100;

    let mut num_vec = Vec::new();

    num_vec.push(b_one);
    num_vec.push(b_ten);
    num_vec.push(b_hund);

    for i in num_vec.iter() {
        println!("{}", anum * i);
    }
    println!("{}", anum * bnum);
}