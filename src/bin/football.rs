use std::io::{self, BufRead};

fn main() {
    let mut stdin = io::stdin().lock();
    let mut condition = String::new();
    let _ = stdin.read_line(&mut condition);
    let input = condition.as_bytes();
    let mut current = input[0];
    let mut len = 1;
    for i in input.iter().skip(1) { 
        if *i == current {
            len+=1;
        }
        else {
            current = *i;
            len = 1;
        }
        if len == 7 {
            println!("YES");
            return;
        }
    }
    println!("NO");
}
