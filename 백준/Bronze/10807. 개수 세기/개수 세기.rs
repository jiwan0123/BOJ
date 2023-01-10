use std::io;
fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();

    let mut b = String::new();
    io::stdin().read_line(&mut b).unwrap();

    let mut c = String::new();
    io::stdin().read_line(&mut c).unwrap();

    let numa: i32 = a.trim().parse().unwrap();

    let arr: Vec<i32> = b
        .as_mut_str()
        .split_whitespace()
        .map(|b| b.parse().unwrap())
        .collect();

    let mut arr_iter = arr.iter();
    let numc: i32 = c.trim().parse().unwrap();

    let mut cnt: u32 = 0;

    for _ in 0..numa {
        if arr_iter.next() == Some(&numc) {
            cnt += 1;
        }
    }
    print!("{}", cnt);
}