fn main() {
    let s1 = String::from("hello");

    let len = calc_len(&s1); // 참조자 생성

    println!("{} {}", s1, len);

    let mut s1 = s1;
    mutation_str(&mut s1); // 가변 참조자 생성
    println!("{}", s1);

    let r1 = &mut s1;
    // let r2 = &mut s1; 데이터 레이스: 가변 참조자를 같은 스코프에서 여러 개 만들 수 없다.
    println!("r1: {}", r1);
    // println!("r2: {}", r2);
}

fn calc_len(s: &String) -> usize {
    s.len()
} // 참조자 삭제

fn mutation_str(s: &mut String) {
    s.push_str(", world");
} // 참조자 삭제
