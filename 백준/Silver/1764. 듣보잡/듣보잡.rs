use std::collections::HashSet;
use std::fmt::Write;
use std::io::{stdin, Read};

fn main() {
    let mut stdin = stdin().lock();
    let mut buf = String::new();
    stdin.read_to_string(&mut buf).unwrap();

    let mut input = buf.split_whitespace();

    let mut output = String::new(); //출력버퍼

    let n = input.next().unwrap().parse::<i32>().unwrap();
    input.next();

    let nothear: HashSet<_> = (0..n).map(|_| input.next().unwrap()).collect();
    let mut nothearsee: Vec<_> = input.filter(|x| nothear.contains(x)).collect();

    nothearsee.sort();

    writeln!(output, "{}", nothearsee.len()).unwrap();
    writeln!(output, "{}", nothearsee.join("\n")).unwrap();

    print!("{}", output);
}
