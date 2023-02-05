use std::fmt::Write;

fn main() {
    let mut v: Vec<bool> = vec![false; 10001];
    let mut output = String::new();

    for mut i in 1..10001 as i32 {
        let mut n = d(i) as usize;
        if (n < 10001) {
            v[n] = true;
        }
        if v[i as usize] == false {
            writeln!(&mut output, "{}", i).unwrap();
        }
    }
    print!("{}", output);
}

fn d(mut n: i32) -> i32 {
    let mut re = n;
    while n != 0 {
        re += n % 10; // 입력받은수에 첫째자리수를 더함
        n /= 10; // 첫째 자리수를 없앰 ex) 1234 =>123 그후 반복하면 12 1이 된다
    } // 다 더하고 0이 되면 탈출하여 리턴함
    re //1234입력받고 1244를 리턴함 이 수는 생성자가 있으므로 거른다
}
