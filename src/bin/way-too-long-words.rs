use std::io::{self, BufRead};

fn main() {
    let mut input = String::new();
    let mut stdin = io::stdin().lock();
    let _ = stdin.read_line(&mut input);
    let x: i32 = input.trim().parse().ok().unwrap();
    for _ in 0..x {
        let mut word = String::new();
        let _ = stdin.read_line(&mut word);
        word  = word.trim().to_string();
        if word.len() > 10 {
            let first_char = word.chars().next().unwrap();
            let last_char = word.chars().next_back().unwrap();
            println!("{first_char}{}{last_char}", word.len() - 2)
        }
        else {
            println!{"{word}"};
        }
    }
}
