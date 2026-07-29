struct Range {
    low: usize,
    high: usize,
}

impl Range {
    fn new(low: usize, high: usize) -> Range {
        Range { low, high }
    }
}

fn quick_sort(arr: &mut [i32]) {
    fn partition(arr: &mut [i32], r: &Range) -> usize {
        let mut i = r.low;
        let low = r.low;
        let high = r.high;
        let pivot = arr[high];
        for j in low..high {
            if arr[j] <= pivot {
                arr.swap(i, j);
                i += 1;
            }
        }
        arr.swap(i, high);
        i
    }
    if arr.len() < 2 {
        return;
    }
    let mut stack = vec![Range::new(0, arr.len() - 1)];
    while let Some(r) = stack.pop() {
        if r.low >= r.high {
            continue;
        }
        let pi = partition(arr, &r);
        if (pi - r.low) > 1 {
            stack.push(Range {
                low: r.low,
                high: pi - 1,
            });
        }
        if (r.high - pi) > 1 {
            stack.push(Range {
                low: pi + 1,
                high: r.high,
            });
        }
    }
}

fn main() {
    let mut v = vec![10, 9, 5, 1, 7];
    quick_sort(&mut v);
    for i in &v {
        print!("{i} ");
    }
}
