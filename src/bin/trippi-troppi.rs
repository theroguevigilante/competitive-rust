#![allow(dead_code)]
use std::io::{self, BufRead};

struct Scanner<R> {
    reader: R,
    buffer: Vec<String>,
}

impl<R: BufRead> Scanner<R> {
    fn new(reader: R) -> Self {
        Self {
            reader,
            buffer: Vec::new(),
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            self.reader.read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }

    fn next_line(&mut self) -> String {
        let mut input = String::new();
        self.reader.read_line(&mut input).expect("Failed read");
        input.trim_end().to_string()
    }
}

fn solve<R: BufRead>(input: &mut Scanner<R>) {
    let mut name = String::new();
    for _ in 0..3 {
        let get: String = input.next();
        name.push(get.chars().next().unwrap());
    }
    println!("{name}")
}

fn main() {
    let stdin = io::stdin();
    let mut cin = Scanner::new(stdin.lock());

    let t: u32 = cin.next();
    for _ in 0..t {
        solve(&mut cin);
    }
}
