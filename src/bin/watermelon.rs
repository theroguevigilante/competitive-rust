use std::io;
fn main() {
    let mut x = String::new();
    let _ = io::stdin().read_line(&mut x);
    let x: i32 = x.trim().parse().ok().unwrap();
    if x % 2 == 0 && x != 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
