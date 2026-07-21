struct Collatz {
    current: u64,
}

impl Iterator for Collatz {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == 0 {
            return None;
        }

        let current = self.current;
        if current == 1 {
            self.current = 0;
        } else if current.is_multiple_of(2) {
            self.current = current / 2;
        } else {
            self.current = self.current * 3 + 1;
        }
        Some(current)
    }
}

fn main() {
    let collatz = Collatz { current: 69420 };
    for i in collatz {
        println!("{i}");
    }
}
