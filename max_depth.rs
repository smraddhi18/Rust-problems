use std::io;

#[derive(Debug)]
struct TreeNode {
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new() -> Self {
        TreeNode {
            left: None,
            right: None,
        }
    }
}

fn max_depth(root: &Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(&node.left);
            let right_depth = max_depth(&node.right);
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}

fn main() {
    println!("Enter the values of the nodes in the binary tree (enter -1 to indicate a null node):");
    let root = build_tree();

    let depth = max_depth(&root);
    println!("The maximum depth of the binary tree is: {}", depth);
}

fn build_tree() -> Option<Box<TreeNode>> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let trimmed = input.trim();
    if trimmed == "-1" {
        return None;
    }

    let mut root = Box::new(TreeNode::new());
    println!("Enter left child of node {}:", trimmed);
    root.left = build_tree();
    println!("Enter right child of node {}:", trimmed);
    root.right = build_tree();

    Some(root)
}
