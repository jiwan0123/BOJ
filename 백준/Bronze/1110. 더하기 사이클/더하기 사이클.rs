use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let compare: i32 = input.trim().parse().unwrap();
    let mut temp: i32 = input.trim().parse().unwrap();
    let mut ten: i32 = 0;
    let mut one: i32 = 0;
    let mut result: i32 = 0;
    let mut cnt: i32 = 0;
    loop {
        if (temp < 10) {
            ten = 0;
        } else {
            ten = temp / 10;
        }
        one = temp % 10;

        result = ten + one;
        cnt = cnt + 1;

        ten = temp % 10;
        one = result % 10;
        temp = (ten * 10) + one;

        if temp == compare {
            break;
        }
    }
    print!("{}", cnt);
}