use std::io;

fn main() {
    let mut a = String::new();
    io::stdin().read_line(&mut a).unwrap();

    let arr: Vec<char> = a.trim().chars().collect();

    let mut ch: char = char::default(); //char 초기화
    let mut alphabet: [i32; 26] = [0; 26]; //알파벳 배열

    let length: usize = a.trim().len(); //입력받은 글자의 길이

    let mut max: i32 = 0; //글자수 최대로 많은값

    for i in 0..length as usize {
        //글자 길이만큼 반복
        if arr[i as usize] >= 'a' {
            alphabet[arr[i] as usize - 'a' as usize] = alphabet[arr[i] as usize - 'a' as usize] + 1;
        //특정 알파벳의 개수 카운트
        } else {
            alphabet[arr[i] as usize - 'A' as usize] = alphabet[arr[i] as usize - 'A' as usize] + 1;
            //대소문자 구분없이 카운트
        }
    }

    for ii in 0..26 {
        if alphabet[ii] > max {
            //특정알파벳 갯수가 max값보다 클시
            max = alphabet[ii]; //max에 갯수 넣기
            ch = ('A' as u8 + ii as u8) as char; //문자를 ascii로 캐스팅 후 연산 그 후 usize 또는 u32 는 char로 캐스팅 불가
        } else if alphabet[ii] == max {
            //max와 같은게 여러개 있을 때 ?를 ch에 넣는다
            ch = '?';
        }
    }
    print!("{}", ch);
}