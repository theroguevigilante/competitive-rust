use std::str::{FromStr, SplitAsciiWhitespace};
use std::io::{self, Read};

struct Scanner<'a> {
    iter: SplitAsciiWhitespace<'a>,
}

impl<'a> Scanner<'a> {
    fn new(s: &'a str) -> Self {
        Self {
            iter: s.split_ascii_whitespace(),
        }
    }
    fn next<T: FromStr>(&mut self) -> T {
        self.iter.next().unwrap().parse().ok().unwrap()
    }
}

fn main() {
    let mut input = String::new();
    let _ = io::stdin().lock().read_to_string(&mut input);
    let mut cin = Scanner::new(&input);
    let t: i32 = cin.next();
    for _ in 0..t {

    }
}
