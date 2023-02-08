use std::{io, usize};

fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();

    let num: usize = a.trim().parse().unwrap();//반복횟수

    for _ in 0..num {
        let mut b = String::new();
        io::stdin().read_line(&mut b).unwrap();
        let arr: Vec<&str> = b.trim().split_whitespace().collect();//반복횟수와 문자열
        let cnt: usize = arr[0].parse().unwrap(); //횟수만 분리
        let ch: Vec<char> = arr[1].chars().collect(); // 문자열을 vec으로 만들어줌
        for idx in 0..ch.len() as usize { //문자열 길이만큼 반복하는데 인덱스를 뽑아줌
            for _ in 0..cnt { //반복횟수만큼 돌림
                print!("{}", ch[idx]); //문자열의 특정인덱스만 출력 ex) ch[0]출력후 idx가 1 늘어나고 ch[1]출력 
            }
        }
        print!("\n");
    }
}