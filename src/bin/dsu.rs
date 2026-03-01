#![allow(dead_code)]
struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        let mut parent = (0..n).collect();
        let mut size = vec![1; n];
        DisjointSet { parent, size }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] == i {
            i
        } else {
            let root = self.find(self.parent[i]);
            self.parent[i] = root;
            root
        }
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i == root_j {
            return false;
        }

        if self.size[root_i] > self.size[root_j] {
            self.parent[root_j] = root_i;
            self.size[root_i] += self.size[root_j];
        } else {
            self.parent[root_i] = root_j;
            self.size[root_j] += self.size[root_i];
        }
        true
    }
}

fn main() {
    todo!();
}
