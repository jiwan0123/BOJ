use std::io;
fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();

    let num: usize = a.trim().parse().unwrap();

    let mut answer: Vec<usize> = Vec::new();
    let mut x: usize = 0;

    for i in 0..=num {
        if i == 0 || i == 1 {
            answer.push(i);
        } else {
            x = answer[i - 1] + answer[i - 2];
            answer.push(x);
        }
    }
    print!("{}", answer[num]);
}