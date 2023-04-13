use std::collections::HashMap;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut stdin = stdin().lock();
    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf //입력받은 값을 i32로 파싱
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap());

    let mut output = String::new(); //출력버퍼

    let mut counts = HashMap::new();
    let n = input.next().unwrap(); //상근이가 가지고있는 카드 수 할당 후 반복자 넘어감

    for _ in 0..n {
        counts
            .entry(input.next().unwrap()) //값을 참조해서
            .and_modify(|counter| *counter += 1) //있으면 1더해줌
            .or_insert(1); //없으면 1삽입
    }

    for num in input.skip(1) {
        write!(output, "{} ", counts.get(&num).unwrap_or(&0)).unwrap(); //get()은 키에 해당하는 값을 가져옴
    }
    print!("{}", output);
}