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
    fn execute(&self,rope:Box<Node>)->(Box<Node>,usize){
        let content = self.content.graphemes(true).collect::<Vec<&str>>();
        let length=self.content.len();
        
        (insert(rope, self.index, content),self.index+length)
    }
    fn undo(&self,rope:Box<Node>,current_index:usize)->(Box<Node>,usize){
        let content_len=self.content.graphemes(true).count();
        let mut final_index=current_index;
        if current_index>=self.index{
            final_index-=content_len;
        }
        (remove(rope, self.index, content_len).0,final_index)
    }
    
}
impl Command for DeleteCommand{
    fn execute(&self,rope:Box<Node>)->(Box<Node>,usize){
        let (rope,cut_content)=remove(rope, self.index, self.length_cut); 
        let mut inner_content=self.cut_content.borrow_mut();
        *inner_content=cut_content;
        (rope,(self.index+1).saturating_sub(self.length_cut))
    }
    fn undo(&self,rope:Box<Node>,current_index:usize)->(Box<Node>,usize){
        let content=&*self.cut_content.borrow();
        let content = content.graphemes(true).collect::<Vec<&str>>();
        let mut final_index=current_index;
        if current_index>=self.index{
            final_index+=self.length_cut;  
        }
        (insert(rope, self.index, content),final_index)
    }
    
}

pub trait Command{
    fn execute(&self,rope:Box<Node>)->(Box<Node>,usize);
    fn undo(&self,rope:Box<Node>,current_index:usize)->(Box<Node>,usize);    
}