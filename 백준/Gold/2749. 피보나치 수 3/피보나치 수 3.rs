use std::io;

static mut ANSWER: [u128; 1500000] = [0; 1500000];
fn main() {
    let mut a = String::new();
    a.clear();
    io::stdin().read_line(&mut a).unwrap();
    let num: usize = a.trim().parse().unwrap();
    a.clear();

    unsafe {
        ANSWER[0] = 0;
        ANSWER[1] = 1;
        for i in 2..1500000 {
            ANSWER[i] = (ANSWER[i - 1] + ANSWER[i - 2]) % 1000000;
        }
        print!("{}", ANSWER[num % 1500000]);
    }
}