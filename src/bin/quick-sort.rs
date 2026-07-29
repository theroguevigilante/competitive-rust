struct Range{
    low: i32,
    high: i32
}

impl Range{
    fn new(low: i32, high: i32) -> Range{
        Range{ low, high }
    }
}

fn quick_sort(arr:&mut [i32]){
    fn partition(arr:&mut [i32], r: &Range) -> i32{
        let mut i = r.low;
        let low = r.low;
        let high = r.high;
        let pivot = arr[high as usize];
        for j in low..high{
            if arr[j as usize] <= pivot{
                arr.swap(i as usize, j as usize);
                i+=1;
            }
        }
        arr.swap(i as usize, high as usize);
        return i;
    }
    if arr.len() < 2{
        return;
    }
    let mut stack = vec![Range::new(0, arr.len() as i32 - 1)];
    while !stack.is_empty(){
        let r = stack.pop().unwrap();
        if r.low >= r.high{
            continue;
        }
        let pi = partition(arr, &r);
        if (pi - 1 - r.low + 1) > 1{
            stack.push(Range{low: r.low, high: pi - 1});
        }
        if (r.high - pi - 1 + 1) > 1{
            stack.push(Range{low: pi + 1, high: r.high});
        }
    }
}

fn main() {
    let mut v = vec![10, 9, 5, 1, 7];
    quick_sort(&mut v);
    for i in &v{
        print!("{i} ");
    }
}
