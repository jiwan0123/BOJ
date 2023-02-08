use std::io;

fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();

    let n = a.trim().chars();

    let mut alphabet: [i32; 26] = [-1; 26];

    let mut cnt: i32 = 0;
    for i in n {
        let idx = i as i32 - 'a' as i32;
        if alphabet[idx as usize] == -1 {
            alphabet[idx as usize] = cnt;
        }
        cnt = cnt + 1;
    }

    for i in alphabet {
        print! {"{} ", i};
    }
    println!("");
}
