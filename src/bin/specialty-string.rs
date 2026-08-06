use std::collections::HashMap;
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

fn solve(stdin: &mut Scanner) {
    let _: i32 = stdin.next();
    let s = stdin.next::<String>();
    let mut dict = HashMap::new();
    let mut last = s.chars().next().unwrap();
    let mut nc = 1;
    for (index, i) in s.chars().enumerate() {
        if i != last {
            if nc == 1 && s.chars().nth(index + 1).unwrap_or_default('*') != i {
                println!("NO");
                return;
            }
            nc = 1;
            last = i;
        } else {
            nc += 1;
        }
        *dict.entry(i).or_insert(0) += 1;
    }
    for value in dict.values() {
        if *value % 2 != 0 {
            println!("NO");
            return;
        }
    }
    println!("YES");
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
