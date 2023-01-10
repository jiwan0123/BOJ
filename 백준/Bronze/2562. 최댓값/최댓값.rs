use std::io;
fn main() {
    let mut arr: Vec<i32> = Vec::new();

    for _ in 0..9 {
        let mut a = String::new();
        io::stdin().read_line(&mut a).unwrap();
        let b: i32 = a.trim().parse().unwrap();

        arr.push(b);
    }

    let max: i32 = arr.iter().copied().max().unwrap();
    let mut cnt: usize = 0;
    for i in 0..9 {
        cnt = match arr[i] == max {
            true => i + 1,
            false => cnt,
        };
    }

    print!("{}\n{}", max, cnt);
}