use ptree::print_tree;
use std::{
    env,
    fs::File,
    io::{self, Read},
};
use unicode_segmentation::UnicodeSegmentation;

use ratatui::crossterm::terminal::size;
use text_editor::{
    app::App,
    rope::{Rope, build_rope, collect_string, insert},
};
fn main() -> io::Result<()> {
    let file_path = {
        let mut args = env::args();
        match args.nth(1) {
            Some(arg1) => {
                println!("arg is {}", arg1);
                arg1
            }
            None => {
                println!("provide the file path or file name");
                "example.txt".to_string()
            }
        }
    };
    let mut file = match File::open(&file_path) {
        Ok(existing_file) => existing_file,
        Err(err) => match err.kind() {
            io::ErrorKind::NotFound => File::create_new(&file_path)?,
            io::ErrorKind::PermissionDenied => {
                return Err(err);
            }
            _ => {
                return Err(err);
            }
        },
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let text_representation = Rope::new(contents.clone());

    ratatui::run(|terminal| {
        App::new(contents, text_representation, size().unwrap().0 as usize).run(terminal)
    })?;

    // let content: Vec<&str> = contents.graphemes(true).collect::<Vec<&str>>();
    //
    // let mut rope = build_rope(&content, 0, content.len() - 1).0;
    //
    // println!("original tree");
    //
    // print_tree(rope.as_ref()).unwrap();
    //
    // rope = insert(rope, 6, vec!["t"]);
    //
    // let mut collected_string = String::new();
    // collect_string(&rope, &mut collected_string);
    // println!("collected string");
    // println!("{}", collected_string);
    // println!("first tree");
    //
    // print_tree(rope.as_ref()).unwrap();

    // for i in (0..9).rev(){
    //     rope=remove(rope, i, 1);
    // }
    // collected_string.clear();
    // collect_string(&rope, &mut collected_string);

    // println!("collected string 2");
    // println!("{}",collected_string);
    // println!("third tree");

    // print_tree(rope.as_ref()).unwrap();

    // rope=insert(rope, 7, "\n".to_string());

    // collected_string.clear();
    // collect_string(&rope, &mut collected_string);

    // println!("collected string 3");
    // println!("{}",collected_string);
    // println!("third tree");
    // print_tree(rope.as_ref()).unwrap();

    // for i in (0..79).rev(){
    //     rope=remove(rope, i, 1);
    // }
    // println!("new tree");
    // print_tree(rope.as_ref()).unwrap();
    // let mut collected=String::new();
    // collect_string(&rope, &mut collected);
    // println!("collected is {}",collected);
    // rope=remove(rope, 0, 1);

    // println!("new tree");
    // print_tree(rope.as_ref()).unwrap();

    // collected.clear();

    // collect_string(&rope, &mut collected);
    // println!("collected 2 is {}",collected);

    // let mut gap_buffer=GapBuffer::new(&contents);
    // let sum=gap_buffer.length_up_to_non_inclusive_index(5);
    // println!("the buffer is {:?} ,start is {} and end is {} and sum is {}",gap_buffer.buffer(),gap_buffer.starting_of_gap(),gap_buffer.ending_of_gap(),sum);
    // let (row,column)=gap_buffer.find_where_rope_index_fits(5);
    // println!("row is {} and column is {}",row,column);
    // let at=gap_buffer.index(2);
    // println!("item at is {:?}",at);
    // let removed=gap_buffer.remove_item(7);
    // println!("removed is {:?}",removed);
    // println!("the second buffer is {:?} ,start is {} and end is {}",gap_buffer.buffer(),gap_buffer.starting_of_gap(),gap_buffer.ending_of_gap());

    // gap_buffer.add_item(1);

    // let sum=gap_buffer.length_up_to_non_inclusive_index(2);
    // println!("the buffer is {:?} ,start is {} and end is {} and sum is {}",gap_buffer.buffer(),gap_buffer.starting_of_gap(),gap_buffer.ending_of_gap(),sum);

    // println!("the new buffer is {:?} ,start is {} and end is {}",gap_buffer.buffer(),gap_buffer.starting_of_gap(),gap_buffer.ending_of_gap());
    // gap_buffer.add_item(5);
    // println!("the second new buffer is {:?} ,start is {} and end is {}",gap_buffer.buffer(),gap_buffer.starting_of_gap(),gap_buffer.ending_of_gap());
    // gap_buffer.add_item(1);
    // println!("the third new buffer is {:?} ,start is {} and end is {}",gap_buffer.buffer(),gap_buffer.starting_of_gap(),gap_buffer.ending_of_gap());
    // gap_buffer.add_item(2);
    // println!("the third new buffer is {:?} ,start is {} and end is {}",gap_buffer.buffer(),gap_buffer.starting_of_gap(),gap_buffer.ending_of_gap());

    // let item =gap_buffer.index(9);
    // println!("item is {:?}",item);

    // gap_buffer.add_item(5);
    // println!("the fourth new buffer is {:?} ,start is {} and end is {}",gap_buffer.buffer(),gap_buffer.starting_of_gap(),gap_buffer.ending_of_gap());

    // gap_buffer.increase(2);
    // println!("the fifth new buffer is {:?} ,start is {} and end is {}",gap_buffer.buffer(),gap_buffer.starting_of_gap(),gap_buffer.ending_of_gap());

    // let s = String::from("hello we ❤️ you");
    // let g = s.graphemes(true).collect::<Vec<&str>>();
    // // let inner :Vec<&str>=  s.graphemes(true).collect();
    // // println!("inner is {}",inner[1]);

    // let rope=build_rope(&g, 0, g.len()-1).0;
    // println!("original rope");

    // print_tree(rope.as_ref()).unwrap();

    // let rope=insert(rope, 6, "you 😇".to_string());
    // println!("new rope");

    // print_tree(rope.as_ref()).unwrap();

    // let mut collected_string=String::new();

    // if let Some(_)=find_sub_str(&rope, 11, 25,&mut collected_string){

    //     println!("collected sub str:{}",collected_string);

    // }
    // if let Some(item)=index(&rope, 10){
    //     println!("item is {}",item);
    // }

    // let rope=remove(rope, 14,1);
    // println!("removed something rope");

    // print_tree(rope.as_ref()).unwrap();

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
