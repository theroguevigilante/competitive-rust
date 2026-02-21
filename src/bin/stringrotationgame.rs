#![allow(dead_code)]
use std::{
    collections::HashSet,
    io::{self, Read},
};

struct Scanner<'a> {
    input: &'a str,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Self {
        Self { input: s }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        self.input = self.input.trim_start();

        let end = self
            .input
            .find(char::is_whitespace)
            .unwrap_or(self.input.len());
        let token = &self.input[..end];
        self.input = &self.input[end..];
        token.parse().ok().expect("Failed to parse token")
    }

    fn next_line(&mut self) -> &'a str {
        self.input = self.input.trim_start_matches([' ', '\t']);
        if let Some(stripped) = self.input.strip_prefix('\r') {
            self.input = stripped;
        }
        if let Some(stripped) = self.input.strip_prefix('\n') {
            self.input = stripped;
        }

        let end = self.input.find('\n').unwrap_or(self.input.len());
        let line = &self.input[..end];

        self.input = &self.input[end..];
        if !self.input.is_empty() {
            self.input = &self.input[1..];
        }

        line.trim_end_matches('\r')
    }
}

fn solve(cin: &mut Scanner) {
    let _: i32 = cin.next();
    let word: String = cin.next();
    let d: HashSet<char> = word.chars().collect();
    if d.len() == word.len() {
        println!("{}",word.len());
    }
    else if d.len()==1 {
        println!("{}", 1);
    }
    else {
        println!("{}",d.len()+1);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");
    let mut cin = Scanner::new(&input);

    let t: u32 = cin.next();
    for _ in 0..t {
        solve(&mut cin);
    }
}
