use std::{cell::RefCell, ops::Deref};

use unicode_segmentation::UnicodeSegmentation;

use crate::rope::{Node, insert, remove};

pub struct InsertCommand{
    content:String,
    index:usize
}

impl InsertCommand{
    pub fn new(content:String,index:usize)->Self{
        Self { content, index }
    }
    
}

pub struct DeleteCommand{
    length_cut:usize,
    index:usize,
    cut_content:RefCell<String>
}
impl DeleteCommand {
    pub fn new(length_cut:usize,index:usize)->Self{
        Self { length_cut, index,cut_content:RefCell::new(String::new())}
    }
    
    
}
impl Command for InsertCommand{
    fn execute(&self,rope:Box<Node>)->Box<Node>{
        insert(rope, self.index, &self.content) 
    }
    fn undo(&self,rope:Box<Node>)->Box<Node>{
        let content_len=self.content.graphemes(true).count();
        remove(rope, self.index, content_len).0
    }
    
}
impl Command for DeleteCommand{
    fn execute(&self,rope:Box<Node>)->Box<Node>{
        let (rope,cut_content)=remove(rope, self.index, self.length_cut); 
        let mut inner_content=self.cut_content.borrow_mut();
        *inner_content=cut_content;
        rope
    }
    fn undo(&self,rope:Box<Node>)->Box<Node>{
        let content=self.cut_content.borrow();
        insert(rope, self.index, &*content) 
    }
    
}

pub trait Command{
    fn execute(&self,rope:Box<Node>)->Box<Node>;
    fn undo(&self,rope:Box<Node>)->Box<Node>;    
}