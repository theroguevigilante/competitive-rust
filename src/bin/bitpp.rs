use std::io::{self, BufRead};

fn main() {
    let mut input = io::stdin().lock();
    let mut n: String = String::new();
    let _ = input.read_line(&mut n);
    let n: i32 = n.trim().parse().ok().unwrap();
    let mut x = 0;
    let mut line = String::new();
    for _ in 0..n {
        line.clear();
        let _ = input.read_line(&mut line);
        let line = line.trim();
        match (line.contains('+'), line.contains('-')) {
            (true, false) => x += 1,
            (false, true) => x-= 1,
            _ => continue
        }
    }
    println!("{x}");
}
