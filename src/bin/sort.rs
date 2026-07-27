enum SortVariation {
    Bubble,
    Select,
    Insert,
    Merge,
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
            let temp = nums[i];
            let mut curr = i;
            for j in (1..=i).rev() {
                if temp < nums[j - 1] {
                    nums[j] = nums[j - 1];
                    curr = j - 1;
                } else {
                    break;
                }
            }
            nums[curr] = temp;
        }
    }

    fn selection_sort(nums: &mut [i32]) {
        let length = nums.len();
        for i in 0..length {
            let mut min_index = i;
            for j in i+1..length {
                if nums[j] < nums[min_index] {
                    min_index = j;
                }
            }
            nums.swap(i, min_index);
        }
    }

    fn merge_sort(nums: &[i32]) -> Vec<i32> {
        fn merge(a1: &[i32], a2: &[i32]) -> Vec<i32> {
            let mut i = 0;
            let mut j = 0;
            let mut temp = Vec::new();
            while i < a1.len() && j < a2.len() {
                if a1[i] > a2[j] {
                    temp.push(a2[j]);
                    j += 1;
                } else {
                    temp.push(a1[i]);
                    i += 1;
                }
            }
            while i < a1.len() {
                temp.push(a1[i]);
                i += 1;
            }
            while j < a2.len() {
                temp.push(a2[j]);
                j += 1;
            }
            temp
        }

        let length = nums.len();
        if length < 2 {
            return nums.to_vec();
        }
        let mid = length / 2;
        let a1 = Self::merge_sort(&nums[..mid]);
        let a2 = Self::merge_sort(&nums[mid..]);

        merge(&a1, &a2)
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
    println!("Enter 1 for selection, 2 for insertion, 3 for bubble and 4 for merge:");
    let mut x: String = String::new();
    let _ = std::io::stdin().read_line(&mut x);
    let x: i32 = x.trim().parse().unwrap();
    let sort = match x {
        1 => SortVariation::Select,
        2 => SortVariation::Insert,
        3 => SortVariation::Bubble,
        4 => SortVariation::Merge,
        _ => SortVariation::Unknown,
    };

    match sort {
        SortVariation::Bubble => SortVariation::bubble_sort(&mut nums),
        SortVariation::Insert => SortVariation::insertion_sort(&mut nums),
        SortVariation::Select => SortVariation::selection_sort(&mut nums),
        SortVariation::Merge => {nums = SortVariation::merge_sort(&nums);},
        SortVariation::Unknown => println!("Unknown Sort"),
    }
    println!("Sorted Array!");
    for num in &nums {
        print!("{} ", num);
    }
    println!("\n");
}
