use std::io;
fn main() {
    let mut arr: [usize; 30] = [0; 30];

    for _ in 0..28 {
        let mut a = String::new();
        io::stdin().read_line(&mut a).unwrap();
        let a: usize = a.trim().parse().unwrap(); //학번

        arr[a - 1] = 1;
    }

    for i in 0..30 {
        if arr[i] == 0 {
            print!("{}\n", i + 1);
        }
    }
}