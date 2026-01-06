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
        rebalance, remove, sub_rope,
    },
};
use unicode_segmentation::UnicodeSegmentation;

fn main() -> io::Result<()> {
    // ratatui::run(|terminal| App::new("hello we ❤️ you".to_string()).run(terminal))?;
    let s = String::from("hello we ❤️ you");
    let g = s.graphemes(true).collect::<Vec<&str>>();
    // let inner :Vec<&str>=  s.graphemes(true).collect();
    // println!("inner is {}",inner[1]);

    let rope=build_rope(&g, 0, g.len()-1).0;
    println!("original rope");
    
    print_tree(rope.as_ref()).unwrap();
    
    let rope=insert(rope, 6, "you 😇".to_string());
    println!("new rope");
    
    print_tree(rope.as_ref()).unwrap();
    
    if let Some(sub_rope)=find_sub_rope(&rope, 11, 20){
        println!("sub rope");
        
        print_tree(&sub_rope).unwrap();
        
    }
    if let Some(item)=index(&rope, 10){
        println!("item is {}",item);        
    }
    
    let rope=remove(rope, 14,1);
    println!("removed something rope");
    
    print_tree(rope.as_ref()).unwrap();
    
    // let content = "Hello my name is Simon";
    // let content: Vec<char> = content.chars().collect();

    // let rope = build_rope(&content, 0, content.len() - 1).0;

    // let sub_rope = find_sub_rope(rope.as_ref(), 21, 24);

    // if let Some(new) = sub_rope {
    //     print_tree(&new).unwrap();
    // }else{
    //     println!("bounds failure");
    // }

    Ok(())
}
