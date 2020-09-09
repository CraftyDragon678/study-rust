// 사용자의 입력을 받고 결과값을 표시하기 위한 라이브러리
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // 러스트에서 변수는 기본적으로 불변,
    // 그래서 mut(mutable) 키워드를 사용해서 가변함수를 만든다.
    let mut guess = String::new();
    // :: 는 new가 String타입의 연관함수임을 나타낸다.
    // 연관함수는 하나의 타입을 위한 함수.
    // String 인스턴스가 아니라 String 타입을 위한 함수 (static method)

    io::stdin().read_line(&mut guess)  // io::Result => Ok 또는 Err
        .expect("Failed to read line");  // Err일 때 실행하는 메소드. 프로그램 작동을 멈추고 인자를 출력

    println!("You guessed: {}", guess); // placeholder 사용
}
