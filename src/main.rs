use text_editor::rope::{build_rope, collect_string, find_length, index, insert, remove};

fn main() {
    println!("small change");
    let content_str = "Hello my name is Simon";
    let content: Vec<char> = content_str.chars().collect();
    println!("content len is {}", content.len());
    let rope = build_rope(&content, 0, content.len() - 1);

    let whole_length = find_length(&rope);
    println!("whole length is {}", whole_length);

    if rope.right().is_some() {
        let right_length = find_length(rope.right().unwrap());
        println!("right length is {}", right_length);
    }

    let rope = insert(6, rope, "people ".to_string());

    let mut collected_string = String::new();

    collect_string(&rope, &mut collected_string);
    let item_index: usize = 9;
    println!("char at {} is {:?}", item_index, index(&rope, item_index));

    // println!("rope is {:?}",rope);
    let content_str = "Hello people my name is Simon";

    assert_eq!(content_str, collected_string);
    let rope = remove(27, Box::new(rope), 1);
    let mut collected_string = String::new();

    collect_string(&rope, &mut collected_string);

    let content_str = "Hello people my name is Simo";

    assert_eq!(content_str, collected_string);
}
