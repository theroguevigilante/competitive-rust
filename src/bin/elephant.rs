use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: i64 = input.trim().parse().ok().unwrap();
    println!("{}", (input + 4) / 5);
}
