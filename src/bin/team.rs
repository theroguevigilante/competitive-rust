use std::io::{self, BufRead};
fn main() {
    let mut input = io::stdin().lock();
    let mut n =  String::new();
    let _ = input.read_line(&mut n);
    let n: i32 = n.trim().parse().ok().unwrap();
    let mut solve = 0;
    for _ in 0..n {
        let mut line = String::new();
        let _ = input.read_line(&mut line);
        line = line.trim().to_string();
        let a: Vec<char> = line.split_ascii_whitespace().flat_map(|word| word.chars()).collect();
        let sum: u32 = a.into_iter().map(|x| x.to_digit(10).unwrap()).sum();
        if sum >= 2 {
            solve += 1;
        }
    }
    println!("{solve}");
}
