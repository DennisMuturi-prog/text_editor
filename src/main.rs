
use std::io;

use ptree::{Color, PrintConfig, Style, TreeBuilder, print_config::UTF_CHARS_BOLD, print_tree, print_tree_with};
use text_editor::{app::App, rope::{Node, build_rope, collect_string, concatenate, find_length, index, insert, make_unbalanced_rope, rebalance, remove}};

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::new(String::new()).run(terminal))?;
    // let mut rope=build_rope(&['h','e','l'], 0, 2).0;
    
    // rope=remove(rope,1,2);
    // print_tree(rope.as_ref()).unwrap();
    
    // rope=remove(rope,0,1);
    // print_tree(rope.as_ref()).unwrap();
    
    // rope=insert(rope,0,"hey".to_string());
    // print_tree(rope.as_ref()).unwrap();
    
    
    Ok(())
}
