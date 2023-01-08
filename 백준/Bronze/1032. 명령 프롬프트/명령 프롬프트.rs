use std::io;

fn main() {
    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("error");

    let num: i32 = n.trim().parse().unwrap();

    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("error");

    let mut a_list: Vec<char> = a.chars().collect();

    let a_len: usize = a_list.len();

    for i in 0..num - 1 {
        let mut b = String::new();
        io::stdin().read_line(&mut b).expect("b error");

        let mut b_list: Vec<char> = b.chars().collect();
        for j in 0..a_len {
            if a_list[j] != b_list[j] {
                a_list[j] = '?';
            }
        }
    }
    let result = String::from_iter(a_list);
    let bindresult = result.trim();
    println!("{}", bindresult);
}