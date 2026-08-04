fn main() {
    let data = ["rust", "is", "awesome"];
    let data_str = data.iter().fold(String::new(), |mut acc, x| {
        acc.push_str(x);
        acc.push(' ');
        acc
    });
    println!("{data_str}");
    let nums = [1, 2, 3, 4];
    let squares: Vec<i32> = nums.iter().map(|x| x * x).collect();
    println!("{:?}", squares);
    let nums = [1, 2, 3, 4, 5];
    let doubled: Vec<i32> = nums.iter().filter(|x| *x % 2 != 0).map(|x| 2 * x).collect();
    println!("{:?}", doubled);
    let data = ["abc", "hello", "rust"];
    let nc: usize = data.iter().map(|x| x.len()).sum();
    println!("{nc}");
    let nums = [1, 3, 7, 10, 11];
    let first = nums.iter().find(|x| *x % 2 == 0);
    println!("{}", first.unwrap());
    let product: i32 = nums.iter().product();
    println!("{product}");
    let nums = [-1, 2, 3];
    let check = nums.iter().all(|x| *x >= 0);
    println!("{check}");
    let data = "helloWorld";
    let check = data.chars().any(|x| x.is_uppercase());
    println!("{check}");
    let d1 = ["Alice", "Bob"];
    let d2 = [90, 75];
    let transformed: Vec<(&str, i32)> = d1.into_iter().zip(d2).collect();
    println!("{:?}", transformed);
    let words = ["rust", "rocks"];
    let reversed: Vec<String> = words
        .iter()
        .map(|word| word.chars().rev().collect())
        .collect();
    let joined = words.iter().fold(String::new(), |mut acc, x| {
        acc.push_str(x);
        acc.push(' ');
        acc
    });
    println!("{:?}", reversed);
    println!("{joined}");
}
