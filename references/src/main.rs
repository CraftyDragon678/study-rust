fn main() {
    let s1 = String::from("hello");

    let len = calc_len(&s1);

    println!("{} {}", s1, len);

    let mut s1 = s1;
    mutation_str(&mut s1);
    println!("{}", s1);
}

fn calc_len(s: &String) -> usize {
    s.len()
}

fn mutation_str(s: &mut String) {
    s.push_str(", world");
}
