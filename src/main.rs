
use ptree::{Color, PrintConfig, Style, TreeBuilder, print_config::UTF_CHARS_BOLD, print_tree, print_tree_with};
use text_editor::rope::{Node, build_rope, collect_string, concatenate, find_length, index, insert, make_unbalanced_rope, rebalance, remove};

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
    
    
    let a = Node::new("a".to_string());      // length 1
        let b = Node::new("b".to_string());      // length 1  
        let c = Node::new("c".to_string());      // length 1
        let d = Node::new("d".to_string());      // length 1
        let e = Node::new("e".to_string());      // length 1
        
        // Build a deep, unbalanced chain: ((((a,b),c),d),e)
        let ab = concatenate(Box::new(a), Box::new(b));
        let abc = concatenate(Box::new(ab), Box::new(c));
        let abcd = concatenate(Box::new(abc), Box::new(d));
        let abcde = concatenate(Box::new(abcd), Box::new(e));
        
        println!("=== Unbalanced Rope ===");
        println!("This rope is intentionally unbalanced with depth 4 and total length 5");
        print_tree(&abcde).unwrap();
        
        // Check if it's balanced according to the rope's criteria
        // For depth 4, FIBONACCI[4+2] = FIBONACCI[6] = 8
        // Since length 5 < 8, this rope is unbalanced
        
        let balanced = rebalance(Box::new(abcde));
        println!("\n=== After Rebalancing ===");
        print_tree(balanced.as_ref()).unwrap();
        
        println!("tree balanced {}",balanced.is_balanced());
        
        
        println!("\n=== Making unbalanced rope ===");
        let rope=make_unbalanced_rope();
        print_tree(rope.as_ref()).unwrap();
        println!("unbalanced balanced {}",rope.is_balanced());
        
        let balanced = rebalance(rope);
        println!("\n=== After Rebalancing made rope===");
        print_tree(balanced.as_ref()).unwrap();
}
