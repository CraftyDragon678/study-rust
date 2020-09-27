fn main() {
    let s1 = String::from("hello");

    let len = calc_len(&s1); // 참조자 생성

    println!("{} {}", s1, len);

    let mut s1 = s1;
    mutation_str(&mut s1); // 가변 참조자 생성
    println!("{}", s1);
}

fn calc_len(s: &String) -> usize {
    s.len()
} // 참조자 삭제

fn mutation_str(s: &mut String) {
    s.push_str(", world");
} // 참조자 삭제
