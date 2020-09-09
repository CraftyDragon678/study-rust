extern crate rand; // 외부에 의존하는 크레이트, `cargo doc --open`으로 사용법을 알 수 있다.

// 사용자의 입력을 받고 결과값을 표시하기 위한 라이브러리
use std::io;
// Result같은 열거형 (Less, Greater, Equal)
use std::cmp::Ordering;
// 정수 생성기가 구현한 메소드를 정의한 `trait`
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

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

    // 작동하지 않는다. 형태가 일치하지 않음
    // secret_number: u32, guess: String
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too small!"),
        Ordering::Equal => println!("Too small!"),
    }
}
