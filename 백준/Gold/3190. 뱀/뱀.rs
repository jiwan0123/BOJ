use std::collections::{HashSet, VecDeque};
use std::io;

const DIRECTION: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
//방향 y, x 오른쪽 아래 왼쪽 위
fn read_line() -> String {
    let mut buf = String::new();
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_owned()
}
fn main() {
    let n: isize = read_line().parse::<isize>().unwrap(); //보드크기
    let mut apples = HashSet::new();

    for _ in 0..read_line().parse::<usize>().unwrap() {
        //입력받은 수만큼 반복
        //입력받은만큼 반복(사과의 개수 K)
        let location = read_line(); //사과위치 입력받음
        let mut applelocation = location
            .split_whitespace()
            .map(|x| x.parse::<isize>().unwrap());
        apples.insert((
            //입력받은 사과위치를 (a, b)형태로 apples: HashSet<>에 삽입
            applelocation.next().unwrap() - 1,
            applelocation.next().unwrap() - 1,
        )); //0, 0이 아닌 1, 1부터 시작하기 때문에 -1 해줌
    }

    let mut moves = Vec::new(); //방향전환정보

    for _ in 0..read_line().parse::<usize>().unwrap() {
        //입력받은 수만큼
        //입력받은만큼 반복(방향전환 횟수 L)
        let sdir = read_line();
        let mut snakedir = sdir.split_whitespace();
        moves.push((
            //moves:(usize, bool)
            snakedir.next().unwrap().parse::<usize>().unwrap(),
            snakedir.next().unwrap() == "D", //D 또는 L이 주어지기에 true 또는 false로 반환함
        ));
    }

    let mut snake = VecDeque::new(); //뱀의 위치
    let mut snake_location = HashSet::new(); //뱀이 지나간 공간

    snake.push_back((0, 0));
    snake_location.insert((0, 0));

    for i in 0..n { //벽
        //n은 보드크기
        snake_location.insert((-1, i));
        snake_location.insert((n, i));
        snake_location.insert((i, -1));
        snake_location.insert((i, n));
    }

    let mut dir = 0; //DIRECTION의 인데스커서
    let mut next_move = moves.into_iter().peekable();
    //moves:Vec<>를 반복자로 사용하기위해 선언
    for t in 1.. {
        //게임시간(1초부터 진행)
        //1부터 무한반복
        let (y, x) = snake.back().unwrap().clone(); //뱀의위치
        let (dy, dx) = DIRECTION[dir]; //방향 (0, 1)부터
        let next = (y + dy, x + dx); //뱀의 다음위치
        //문제에서 사과위치가 1, 2로줬을때 1은 y축 2는 x축의 위치이기때문에 y, x로 줬음

        if snake_location.contains(&next) {
            //뱀의 다음위치가 snake_location에 있으면
            //게임 종료시
            println!("{}", t); //게임시간 출력
            return;
        }

        snake.push_back(next); //snake에 뱀의 다음위치 push(길이 길어짐)
        snake_location.insert(next); //snake_location에 뱀의 다음위치 push

        if !apples.remove(&next) {
            //뱀의 다음위치가 사과의 위치가 아닐때
            let tail = snake.pop_front().unwrap(); //사과를 안먹었으니 꼬리길이 유지
            snake_location.remove(&tail); //꼬리에 부딪히면 종료하기에 remove에줌
        }

        if let Some(&(time, right)) = next_move.peek() {
            //peek()은 반복자를 사용하지않고 반환
            //if let을 통해 Some의 값을 사용할수 있다(Option값)
            //time은 방향전환하는 시간 right는 방향(true or false)
            if t == time {
                //t(게임시간)와 time(방향전환 할 시간)이 일치할 경우
                next_move.next(); //next_move의 다음 요소 가리킴
                dir = (dir + if right { 1 } else { 3 }) % 4;
            }
        }
    }
}
