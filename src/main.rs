use std::io::{self, Write};

use text_editor::rope::{Node, build_rope, collect_string, concatenate, find_length, index, insert, print_tree_horizontal, rebalance, remove};

fn main() {
    let a_node=Node::new("a".to_string());
    let bc_node=Node::new("bc".to_string());
    let d_node=Node::new("d".to_string());
    let ef_node=Node::new("ef".to_string());
    let bottom=concatenate(Box::new(d_node),Box::new(ef_node));
    let second_last=concatenate(Box::new(bc_node), Box::new(bottom));
    let root=Box::new(concatenate(Box::new(a_node),Box::new(second_last)));
    
    // println!("root is {:?}",root);
    print_tree_horizontal(&root, "".to_string(), true);
    
    let root=rebalance(root);
    
    print_tree_horizontal(&root, "".to_string(), true);
    
    // println!("root balanced is {:?}",root);
    // Example: unbalanced tree
        let leaf1 = Node::new("a".to_string());
        let leaf2 = Node::new("b".to_string());
        let leaf3 = Node::new("c".to_string());
        let leaf4 = Node::new("d".to_string());
        let leaf5 = Node::new("e".to_string());
    
        // Right-skewed
        let t1 = concatenate(Box::new(leaf1), Box::new(leaf2));
        let t2 = concatenate(Box::new(t1), Box::new(leaf3));
        let t3 = concatenate(Box::new(t2), Box::new(leaf4));
        let root = Box::new(concatenate(Box::new(t3), Box::new(leaf5)));
    
        println!("--- Unbalanced Tree ---");
        print_tree_horizontal(&root, "".to_string(), true);
    
        // Rebalance
        let balanced_root = rebalance(root);
        println!("\n--- Balanced Tree ---");
        print_tree_horizontal(&balanced_root, "".to_string(), true);
    
}
