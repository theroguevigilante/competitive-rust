#![allow(dead_code)]
use std::{
    io::{self, Read},
    str::SplitAsciiWhitespace,
};

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

fn solve(stdin: &mut Scanner) {
    let n: i32 = stdin.next();
    let mut greatest = 0;
    let mut gc = 0;
    for _ in 0..n {
        let current = stdin.next::<i32>();
        if current > greatest {
            greatest = current;
            gc = 1;
        }
        if current == greatest {
            gc += 1;
        }
    }
    println!("{}", gc-1);
}

fn main() {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    let mut stdin = Scanner::new(&input);
    let t: i32 = stdin.next();
    for _ in 0..t {
        solve(&mut stdin);
    }
}
