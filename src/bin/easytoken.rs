// The simplest and best one yet
#![allow(dead_code)]
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

fn solve() {}

fn main() {}
