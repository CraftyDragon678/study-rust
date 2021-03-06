use std::io::{self, Write};

fn menu() {
    println!("");
    println!("");
    println!("===== my util =====");
    println!("1. c2f");
    println!("2. f2c");
    println!("3. fibo");
    println!("0. quit");
    print!(">> ");
    io::stdout().flush().unwrap();
}

fn main() {
    loop {
        menu();
        let mut menu = String::new();
        io::stdin().read_line(&mut menu)
            .expect("io error");
        let menu: i32 = menu.trim().parse()
            .expect("Please input 1~5");

        if menu == 1 {
            let mut x = String::new();
            io::stdin().read_line(&mut x)
                .expect("io error");
            let x: f64 = x.trim().parse()
                .expect("Please input real number");
            println!("{}", c2f(x))
        } else if menu == 2 {
            let mut x = String::new();
            io::stdin().read_line(&mut x)
                .expect("io error");
            let x: f64 = x.trim().parse()
                .expect("Please input real number");
            println!("{}", f2c(x))
        } else if menu == 3 {
            let mut x = String::new();
            io::stdin().read_line(&mut x)
                .expect("io error");
            let x: i32 = x.trim().parse()
                .expect("Please input number");
            println!("{}", fibo(x))
        } else if menu == 0 {
            break;
        }
    }
}

fn c2f(c: f64) -> f64 {
    c * 1.8 + 32.
}

fn f2c(f: f64) -> f64 {
    (f - 32.) / 1.8
}

fn fibo(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibo(n - 1) + fibo(n - 2)
    }
}
