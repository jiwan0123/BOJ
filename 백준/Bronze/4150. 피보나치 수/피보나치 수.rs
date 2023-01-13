use io::Write;
use std::io;
fn main() {
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = buf.trim().parse().unwrap();

    let result = fibo(n);

    for i in result.iter().rev() {
        //개체 하나하나씩 반복자로 i 에 넣고 출력
        write!(out, "{}", i).unwrap();
    }
}

fn fibo(n: i32) -> Vec<i32> {
    if n <= 1 {
        return Vec::from([n]);
    }

    let (mut a, mut b) = (Vec::from([1]), Vec::from([1]));

    for _ in 2..n {
        let c = add(&mut a, &mut b);
        (a, b) = (b, c); // a = 3, b = 5, next = 8 -> a = 5, b = 8, next = 13
    }

    b
}

fn add(a: &mut Vec<i32>, b: &mut Vec<i32>) -> Vec<i32> {
    let mut carry = 0; //올림수
    let mut sum: Vec<_> = (0..a.len().max(b.len())) //0부터 a와 b의 길이 중 큰것만큼
        .map(|i| {
            let temp = carry + a.get(i).unwrap_or(&0) + b.get(i).unwrap_or(&0);
            carry = temp / 10;

            temp % 10
        })
        .collect();

    if carry > 0 {
        sum.push(carry);
    }

    sum
}
