use std::{io, rc::Rc};

use ptree::{
    Color, PrintConfig, Style, TreeBuilder, print_config::UTF_CHARS_BOLD, print_tree,
    print_tree_with,
};
use text_editor::{
    app::App,
    rc_substr::{RcSubstr, find_grapheme_boundaries},
    rope::{
        Node, build_rope, collect_string, concatenate, find_length, find_sub_rope, index, insert,
        make_unbalanced_rope, rebalance, remove, sub_rope,
    },
};
use unicode_segmentation::UnicodeSegmentation;

fn main() -> io::Result<()> {
    // ratatui::run(|terminal| App::new(String::new()).run(terminal))?;
    let content = "Hello my name is Simon";
    let content: Vec<char> = content.chars().collect();

    let rope = build_rope(&content, 0, content.len() - 1).0;

    let sub_rope = find_sub_rope(rope.as_ref(), 22, 24);

    if let Some(new) = sub_rope {
        print_tree(&new).unwrap();
    }else{
        println!("bounds failure");
    }

    Ok(())
}
