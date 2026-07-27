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
    
    fn post_order(&self)
    where
        T: Display,
    {
        if let Some(left) = &self.left {
            left.post_order();
        }
        if let Some(right) = &self.right {
            right.post_order();
        }
        print!("{} ", self.val);
    }
    
    fn in_order(&self)
    where
        T: Display,
    {
        if let Some(left) = &self.left {
            left.in_order();
        }
        print!("{} ", self.val);
        if let Some(right) = &self.right {
            right.in_order();
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
    let y = TreeNode {
        val: 8,
        left: Some(Box::new(TreeNode {
            val: 4,
            left: Some(Box::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 6,
                left: None,
                right: None,
            })),
        })),
        right: Some(Box::new(TreeNode {
            val: 12,
            left: Some(Box::new(TreeNode {
                val: 10,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 14,
                left: None,
                right: None,
            })),
        })),
    };
    x.pre_order();
    println!();
    x.post_order();
    println!();
    x.in_order();
    println!();
    y.in_order();
    println!();
}
