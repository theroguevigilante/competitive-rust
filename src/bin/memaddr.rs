struct Addr;

impl Addr {
    fn realizer(arr: &[i32], index: &[i32]) -> Option<(Vec<i32>, Vec<i32>)> {
        if arr.is_empty() || !arr.len().is_multiple_of(2) || arr.len() / 2 != index.len() {
            return None;
        }
        let mut ra = Vec::new();
        let mut ri = Vec::new();
        for (i, v) in index.iter().enumerate() {
            let curr: usize = i * 2;
            if arr[curr] > arr[curr + 1] {
                return None;
            }
            if *v < arr[curr] || *v > arr[curr + 1] {
                return None;
            }
            ra.push(arr[curr + 1] - arr[curr] + 1);
            ri.push(v - arr[curr]);
        }
        Some((ra, ri))
    }

    fn rmo(arr: &[i32], index: &[i32]) -> Option<i32> {
        let (ra, ri) = Self::realizer(arr, index)?;
        let mut offset = ri[0];
        for i in 1..ra.len() {
            offset = offset * ra[i] + ri[i];
        }
        Some(offset)
    }

    fn cmo(arr: &[i32], index: &[i32]) -> Option<i32> {
        let (ra, ri) = Self::realizer(arr, index)?;
        let mut offset = ri[ri.len() - 1];
        for i in (0..ra.len() - 1).rev() {
            offset = offset * ra[i] + ri[i];
        }
        Some(offset)
    }
}

fn main() {
    let v = [-5, 5, 7, 20];
    let i = [3, 10];
    let x = Addr::rmo(&v, &i).unwrap();
    println!("{x}");
    let x = Addr::cmo(&v, &i).unwrap();
    println!("{x}");
}
