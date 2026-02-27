// The simplest and best one yet
#![allow(dead_code)]
use std::io::{self, Read};
use std::str::SplitAsciiWhitespace;

struct Scanner<'a> {
    iter: SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            iter: s.split_ascii_whitespace(),
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        self.iter.next().unwrap().parse().ok().unwrap()
    }
}

fn solve(stdin: &mut Scanner) {}

fn main() {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let mut stdin = Scanner::new(&input);
}
