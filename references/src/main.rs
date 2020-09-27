fn main() {
    let s1 = String::from("hello");

    let len = calc_len(&s1);

    println!("{} {}", s1, len);
}

fn calc_len(s: &String) -> usize {
    s.len()
}
