use text_editor::rope::{build_rope, collect_string, find_length, index, insert, remove};

fn main() {
    let content_str = "hello";
    let content: Vec<char> = content_str.chars().collect();
    let rope = build_rope(&content, 0, content.len() - 1);
    println!("rope is {:?}", rope);
    let rope_len = find_length(&rope);
    println!("rope len is {}", rope_len);
    
    let rope = insert(5,rope,"p".to_string());
    println!("rope is {:?}", rope);
    let rope_len = find_length(&rope);
    println!("rope len is {}", rope_len);
    let mut collected_string=String::new();
    collect_string(&rope,&mut collected_string);
    println!("collected string is {}",collected_string);
    
    
    let rope = remove(3,Box::new(rope),2);
    // println!("rope is {:?}", rope);
    let rope_len = find_length(&rope);
    println!("rope len is {}", rope_len);
    let mut collected_string=String::new();
    collect_string(&rope,&mut collected_string);
    println!("collected string is {}",collected_string);
}
