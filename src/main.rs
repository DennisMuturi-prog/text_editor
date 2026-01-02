
use ptree::{Color, PrintConfig, Style, TreeBuilder, print_config::UTF_CHARS_BOLD, print_tree, print_tree_with};
use text_editor::rope::{Node, build_rope, collect_string, concatenate, find_length, index, insert, rebalance, remove};

fn main() {
    let a_node=Node::new("a".to_string());
    let bc_node=Node::new("bc".to_string());
    let d_node=Node::new("d".to_string());
    let ef_node=Node::new("ef".to_string());
    let bottom=concatenate(Box::new(d_node),Box::new(ef_node));
    let second_last=concatenate(Box::new(bc_node), Box::new(bottom));
    let root=concatenate(Box::new(a_node),Box::new(second_last));
    println!("original tree");    
    print_tree(&root).unwrap();
    
    
    
    let root=rebalance(Box::new(root));
    
    println!("balanced tree");
    print_tree(root.as_ref()).unwrap();
    
}
