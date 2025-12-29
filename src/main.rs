use text_editor::rope::{build_rope, collect_string, index};

fn main(){
    println!("small change");
    let content_str="Lorem Ipsum";
    let content:Vec<char>=content_str.chars().collect();
    println!("content len is {}",content.len());
    let rope=build_rope(&content,0,content.len()-1);
    
    let mut collected_string=String::new();
    
    collect_string(&rope,&mut collected_string);
    let item_index:usize=9;
    println!("char at {} is {:?}",item_index,index(&rope,item_index));
    
    println!("rope is {:?}",rope);
    
    assert_eq!(content_str,collected_string);    
}
