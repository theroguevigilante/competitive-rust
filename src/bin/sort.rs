enum SortVariation {
    Bubble,
    Select,
    Insert,
    Unknown,
}

impl SortVariation {
    fn bubble_sort(nums: &mut [i32]) {
        let length = nums.len();
        for i in 0..length {
            let mut swapped = false;
            for j in 1..length - i {
                if nums[j - 1] > nums[j] {
                    nums.swap(j - 1, j);
                    swapped = true;
                }
            }
            if !swapped {
                return;
            }
        }
    }

    fn insertion_sort(nums: &mut [i32]) {
        let length = nums.len();
        for i in 1..length {
            for j in (1..=i).rev() {
                if nums[j] < nums[j - 1] {
                    nums.swap(j, j - 1);
                } else {
                    break;
                }
            }
        }
    }

    fn selection_sort(nums: &mut [i32]) {
        let length = nums.len();
        for i in 0..length {
            let mut min_index = i;
            for j in i..length {
                if nums[j] < nums[min_index] {
                    min_index = j;
                }
            }
            nums.swap(i, min_index);
        }
    }
}

fn main() {
    println!("Hello, user!");
    println!("We are considering an array:");
    let mut nums: Vec<i32> = vec![64, 34, 25, 12, 22, 11, 90];
    for num in &nums {
        print!("{} ", num);
    }
    println!();
    println!("Enter 1 for selection, 2 for insertion and 3 for bubble:");
    let mut x: String = String::new();
    let _ = std::io::stdin().read_line(&mut x);
    let x: i32 = x.trim().parse().unwrap();
    let sort = match x {
        1 => SortVariation::Select,
        2 => SortVariation::Insert,
        3 => SortVariation::Bubble,
        _ => SortVariation::Unknown,
    };

    match sort {
        SortVariation::Bubble => SortVariation::bubble_sort(&mut nums),
        SortVariation::Insert => SortVariation::insertion_sort(&mut nums),
        SortVariation::Select => SortVariation::selection_sort(&mut nums),
        SortVariation::Unknown => println!("Unknown Sort"),
    }
    println!("Sorted Array!");
    for num in &nums {
        print!("{} ", num);
    }
    println!("\n");
}
