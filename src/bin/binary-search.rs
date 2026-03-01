use std::cmp::Ordering;

fn binary_search(arr: &[i32], target: i32) -> Option<i32> {
    let mut lo = 0;
    let mut hi = arr.len();
    while lo<hi {
        let mid = lo + ((hi - lo) / 2);
        match arr[mid].cmp(&target) {
            Ordering::Less => lo = mid + 1,
            Ordering::Greater => hi = mid,
            Ordering::Equal => return Some(mid as i32)
        }
    }
    None
}

fn main() {
    todo!();
}
