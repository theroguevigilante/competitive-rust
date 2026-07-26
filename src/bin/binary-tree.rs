struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
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
}
