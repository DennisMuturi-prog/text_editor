use std::{cell::RefCell};

use unicode_segmentation::UnicodeSegmentation;

use crate::{gap_buffer::GapBuffer, rope::{Node, insert, remove}};

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

pub struct LineMergeTopCommand{
    row_number:usize,
    length_of_line_removed:usize
    
}
impl LineMergeTopCommand{
    pub fn new(row_number:usize,length_of_line_removed:usize)->Self{
        Self { 
            row_number,
            length_of_line_removed
        }
    }
}
impl LineWidthsCommand for LineMergeTopCommand{
    fn execute(&self,lines_widths:&mut GapBuffer){
        if let Some(length_of_line_removed) = lines_widths.remove_item(self.row_number) {
            lines_widths
                .increase_with_count(self.row_number-1, length_of_line_removed);
        };
    }
    fn undo(&self,lines_widths:&mut GapBuffer){
        lines_widths.add_item_with_count(self.row_number,self.length_of_line_removed);
        lines_widths.decrease_with_count(self.row_number-1, self.length_of_line_removed);
        
    }
}

pub struct DecreaseLineCommand{
    row_number:usize
}
impl DecreaseLineCommand {
    pub fn new(row_number: usize) -> Self {
        Self { row_number }
    }
}


impl LineWidthsCommand for DecreaseLineCommand{
    fn execute(&self,line_widths:&mut GapBuffer){
        line_widths.decrease(self.row_number);  
    }
    fn undo(&self,line_widths:&mut GapBuffer){
        line_widths.increase(self.row_number);     
    }
    
}

pub struct IncreaseLineCommand{
    row_number:usize
}
impl IncreaseLineCommand {
    pub fn new(row_number: usize) -> Self {
        Self { row_number }
    }
}

impl LineWidthsCommand for IncreaseLineCommand{
    fn execute(&self,line_widths:&mut GapBuffer){
        line_widths.increase(self.row_number);     
    }
    fn undo(&self,line_widths:&mut GapBuffer){
        line_widths.decrease(self.row_number);  
    }
    
}

pub struct JumpToNewLineWithContentCommand{
    current_line_length:usize,
    row_number:usize,
    column_number:usize
}
impl JumpToNewLineWithContentCommand {
    pub fn new(current_line_length: usize, row_number: usize, column_number: usize) -> Self {
        Self {
            current_line_length,
            row_number,
            column_number
        }
    }
}

impl LineWidthsCommand for JumpToNewLineWithContentCommand{
    fn execute(&self,lines_widths:&mut GapBuffer){
        lines_widths.add_item_with_count(
            self.row_number + 1,
            self.current_line_length - self.column_number,
        );
        lines_widths
            .decrease_with_count(self.row_number, self.current_line_length - self.column_number);
        
    }
    fn undo(&self,lines_widths:&mut GapBuffer){
        lines_widths.remove_item(
            self.row_number + 1,
        );
        lines_widths
            .increase_with_count(self.row_number, self.current_line_length - self.column_number);
        
        
    }
}

pub struct JumpToNewLineWithoutContentCommand{
    row_number:usize
}
impl JumpToNewLineWithoutContentCommand {
    pub fn new(row_number: usize) -> Self {
        Self { row_number }
    }
}
impl LineWidthsCommand for JumpToNewLineWithoutContentCommand{
    fn execute(&self,lines_widths:&mut GapBuffer){
        lines_widths.add_item(self.row_number + 1);
    }
    fn undo(&self,lines_widths:&mut GapBuffer){
        lines_widths.remove_item(self.row_number + 1);
    }
    
}

pub struct PasteCommand{
    row_number:usize,
    length_pasted:usize
}
impl PasteCommand {
    pub fn new(row_number: usize, length_pasted: usize) -> Self {
        Self {
            row_number,
            length_pasted
        }
    }
}

impl LineWidthsCommand for PasteCommand{
    fn execute(&self,line_widths:&mut GapBuffer){
        line_widths.increase_with_count(self.row_number,self.length_pasted);     
    }
    fn undo(&self,line_widths:&mut GapBuffer){
        line_widths.decrease_with_count(self.row_number,self.length_pasted);  
    }
}

pub trait LineWidthsCommand{
    fn execute(&self,line_widths:&mut GapBuffer);
    fn undo(&self,line_widths:&mut GapBuffer);   
}
