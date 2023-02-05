use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: i32 = input.trim().parse().unwrap();

    let mut cnt = 0;
    for i in 1..=n {
        if won(i) {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}

fn won(n: i32) -> bool {
    let a = n / 100; //첫번째
    let b = n / 10 % 10; // 두번째
    let c = n % 10; //세번째

    a == 0 || a - b == b - c
}
