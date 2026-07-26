use std::fmt::Display;

struct TreeNode<T> {
    val: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    fn pre_order(&self)
    where
        T: Display,
    {
        print!("{} ", self.val);
        if let Some(left) = &self.left {
            left.pre_order();
        }
        if let Some(right) = &self.right {
            right.pre_order();
        }
        
    }
}

fn main() {
    let x = TreeNode {
        val: 3,
        left: Some(Box::new(TreeNode {
            val: 5,
            left: Some(Box::new(TreeNode {
                val: 12,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 11,
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(TreeNode {
            val: 2,
            left: None,
            right: Some(Box::new(TreeNode {
                val: 11,
                left: None,
                right: None,
            })),
        })),
    };
    x.pre_order();
    println!();
}
