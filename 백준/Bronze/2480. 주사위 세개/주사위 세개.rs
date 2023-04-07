use std::io;

fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();
    let anum: Vec<i32> = a
        .as_mut_str()
        .split_whitespace()
        .map(|a| a.parse().unwrap())
        .collect();
    let max = *anum.iter().max().unwrap();

    let result: i32 = match (anum[0] == anum[1], anum[1] == anum[2], anum[0] == anum[2]) {
        (true, true, true) => anum[0] * 1000 + 10000, //모두 같을 때
        (true, false, false) => anum[0] * 100 + 1000, //첫째 둘째만 같을 때
        (false, true, false) => anum[1] * 100 + 1000, //둘째 셋째만 같을 때
        (false, false, true) => anum[2] * 100 + 1000, //첫째 셋째만 같을 때
        (false, false, false) => max * 100,
        _ => unreachable!(),
    };
    print!("{}", result);
}