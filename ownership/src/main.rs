fn main() {
    let s1 = String::from("test string");
    println!("{}", s1);

    let s2 = s1;
    // println!("{}", s1); s1이 pop되기 때문에 사용할 수 없다
    println!("{}", s2);

    takes_ownership(s2);
    // println!("{}", s2); takes_ownership에 의해서 drop이 된 변수이다

    let x1 = 5;
    println!("{}", x1);

    let x2 = x1;
    println!("{}", x1); // Copy가능한 타입이기에 사라지지 않고 계속 사용할 수 있다.

    makes_copy(x2);
    println!("{}", x2); // Copy가능해서 함수 안에서 사라지지 않는다.
} // s1, s2는 이미 해제 됐기 때문에 아무일도 일어나지 않고, x1, x2는 사라진다.

fn takes_ownership(some_str: String) { // 스코프 안으로 들어왔다
    println!("{}", some_str);
} // some_str가 가리키는 메모리 해제

fn makes_copy(some_int: i32) {
    println!("{}", some_int);
} // 아무일도 일어나지 않는다.
