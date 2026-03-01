use std::collections::HashMap;

struct DisjointSet {
    parent: Vec<usize>,
    size: Vec<usize>,
}

struct Solution;

impl DisjointSet {
    fn new(n: usize) -> Self {
        let mut parent = (0..n).collect();
        let mut size = vec![1; n];

        DisjointSet { parent, size }
    }

    fn find(&mut self, mut i: usize) -> usize {
        let mut root = i;
        while self.parent[root] != root {
            root = self.parent[root];
        }
        while self.parent[i] != root {
            let next = self.parent[i];
            self.parent[i] = root;
            i = next;
        }
        root
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i != root_j {
            if self.size[root_i] > self.size[root_j] {
                self.parent[root_j] = root_i;
                self.size[root_i] += self.size[root_j];
            } else {
                self.parent[root_i] = root_j;
                self.size[root_j] += self.size[root_i];
            }
            return true;
        }
        false
    }
}

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut email_to_index = HashMap::new();
        let mut ds = DisjointSet::new(accounts.len());
        for (i, account) in accounts.iter().enumerate() {
            for email in account.into_iter().skip(1) {
                match email_to_index.get(&email) {
                    Some(&idx) => {
                        ds.union(i, idx);
                    }
                    None => {
                        email_to_index.insert(email, i);
                    }
                }
            }
        }

        let mut index_to_email = HashMap::new();
        for (email, index) in email_to_index.into_iter() {
            let root = ds.find(index);
            index_to_email.entry(root).or_insert(Vec::new()).push(email);
        }

        let mut ans = Vec::new();
        for (index, mut emails) in index_to_email.into_iter() {
            let mut temp = Vec::new();
            temp.push(accounts[index][0].clone());
            emails.sort();
            for email in emails {
                temp.push(email.to_string());
            }
            ans.push(temp);
        }

        ans
    }
}

fn main() {
    todo!();
}
