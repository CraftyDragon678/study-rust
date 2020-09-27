fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("{}", s1);
    // println!("{}", s2); s2는 takes_and_gives_back으로 넘어가서 
    println!("{}", s3);

    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s2 = takes_and_gives_back(s2);
    println!("{}", s1);
    println!("{}", s2);

    // length
    let s1 = String::from("Hellooooooooo");
    let (s1, len) = get_length(s1);
    print!("{} {}", s1, len);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string // 반환하고
} // 아무일도 일어나지 않는다. (some_string을 가리키는 변수가 사라짐)

fn takes_and_gives_back(some_string: String) -> String {
    some_string // 반환한다
}

fn get_length(s: String) -> (String, usize) {
    let i = s.len();
    (s, i)
}
