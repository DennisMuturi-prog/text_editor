use std::cell::RefCell;

use unicode_segmentation::UnicodeSegmentation;

use crate::{
    gap_buffer::{GapBuffer, LinesGapBuffer},
    rope::{Node, insert, remove},
};

pub struct InsertCommand {
    content: String,
    index: usize,
}

impl InsertCommand {
    pub fn new(content: String, index: usize) -> Self {
        Self { content, index }
    }
}

pub struct DeleteCommand {
    length_cut: usize,
    index: usize,
    cut_content: RefCell<String>,
}
impl DeleteCommand {
    pub fn new(length_cut: usize, index: usize) -> Self {
        Self {
            length_cut,
            index,
            cut_content: RefCell::new(String::new()),
        }
    }
}
impl Command for InsertCommand {
    fn execute(&self, rope: Box<Node>) -> (Box<Node>, usize) {
        let content = self.content.graphemes(true).collect::<Vec<&str>>();
        let length = self.content.len();

        (insert(rope, self.index, content), self.index + length)
    }
    fn undo(&self, rope: Box<Node>) -> (Box<Node>, usize) {
        let content_len = self.content.graphemes(true).count();

        (remove(rope, self.index, content_len).0, self.index)
    }
}
impl Command for DeleteCommand {
    fn execute(&self, rope: Box<Node>) -> (Box<Node>, usize) {
        let (rope, cut_content) = remove(rope, self.index, self.length_cut);
        let mut inner_content = self.cut_content.borrow_mut();
        *inner_content = cut_content;
        (rope, (self.index + 1).saturating_sub(self.length_cut))
    }
    fn undo(&self, rope: Box<Node>) -> (Box<Node>, usize) {
        let content = &*self.cut_content.borrow();
        let content = content.graphemes(true).collect::<Vec<&str>>();

        (insert(rope, self.index, content), self.index + 1)
    }
}

pub trait Command {
    fn execute(&self, rope: Box<Node>) -> (Box<Node>, usize);
    fn undo(&self, rope: Box<Node>) -> (Box<Node>, usize);
}

pub struct LineMergeTopCommand {
    row_number: usize,
    length_of_line_removed: usize,
}
impl LineMergeTopCommand {
    pub fn new(row_number: usize, length_of_line_removed: usize) -> Self {
        Self {
            row_number,
            length_of_line_removed,
        }
    }
}
impl LineWidthsCommand for LineMergeTopCommand {
    fn execute(&self, lines_widths: &mut GapBuffer) {
        if let Some(length_of_line_removed) = lines_widths.remove_item(self.row_number) {
            lines_widths.increase_with_count(self.row_number - 1, length_of_line_removed);
        };
    }
    fn undo(&self, lines_widths: &mut GapBuffer) {
        lines_widths.add_item_with_count(self.row_number, self.length_of_line_removed);
        lines_widths.decrease_with_count(self.row_number - 1, self.length_of_line_removed);
    }
}

pub struct DecreaseLineCommand {
    row_number: usize,
}
impl DecreaseLineCommand {
    pub fn new(row_number: usize) -> Self {
        Self { row_number }
    }
}

impl LineWidthsCommand for DecreaseLineCommand {
    fn execute(&self, line_widths: &mut GapBuffer) {
        line_widths.decrease(self.row_number);
    }
    fn undo(&self, line_widths: &mut GapBuffer) {
        line_widths.increase(self.row_number);
    }
}

pub struct IncreaseLineCommand {
    row_number: usize,
}
impl IncreaseLineCommand {
    pub fn new(row_number: usize) -> Self {
        Self { row_number }
    }
}

impl LineWidthsCommand for IncreaseLineCommand {
    fn execute(&self, line_widths: &mut GapBuffer) {
        line_widths.increase(self.row_number);
    }
    fn undo(&self, line_widths: &mut GapBuffer) {
        line_widths.decrease(self.row_number);
    }
}

pub struct JumpToNewLineWithContentCommand {
    current_line_length: usize,
    row_number: usize,
    column_number: usize,
}
impl JumpToNewLineWithContentCommand {
    pub fn new(current_line_length: usize, row_number: usize, column_number: usize) -> Self {
        Self {
            current_line_length,
            row_number,
            column_number,
        }
    }
}

impl LineWidthsCommand for JumpToNewLineWithContentCommand {
    fn execute(&self, lines_widths: &mut GapBuffer) {
        lines_widths.add_item_with_count(
            self.row_number + 1,
            self.current_line_length - self.column_number,
        );
        lines_widths.decrease_with_count(
            self.row_number,
            self.current_line_length - self.column_number,
        );
    }
    fn undo(&self, lines_widths: &mut GapBuffer) {
        lines_widths.remove_item(self.row_number + 1);
        lines_widths.increase_with_count(
            self.row_number,
            self.current_line_length - self.column_number,
        );
    }
}

pub struct JumpToNewLineWithoutContentCommand {
    row_number: usize,
}
impl JumpToNewLineWithoutContentCommand {
    pub fn new(row_number: usize) -> Self {
        Self { row_number }
    }
}
impl LineWidthsCommand for JumpToNewLineWithoutContentCommand {
    fn execute(&self, lines_widths: &mut GapBuffer) {
        lines_widths.add_item(self.row_number + 1);
    }
    fn undo(&self, lines_widths: &mut GapBuffer) {
        lines_widths.remove_item(self.row_number + 1);
    }
}

pub struct PasteCommand {
    row_number: usize,
    length_pasted: usize,
}
impl PasteCommand {
    pub fn new(row_number: usize, length_pasted: usize) -> Self {
        Self {
            row_number,
            length_pasted,
        }
    }
}

impl LineWidthsCommand for PasteCommand {
    fn execute(&self, line_widths: &mut GapBuffer) {
        line_widths.increase_with_count(self.row_number, self.length_pasted);
    }
    fn undo(&self, line_widths: &mut GapBuffer) {
        line_widths.decrease_with_count(self.row_number, self.length_pasted);
    }
}

pub trait LineWidthsCommand {
    fn execute(&self, line_widths: &mut GapBuffer);
    fn undo(&self, line_widths: &mut GapBuffer);
}

pub trait TextEditorLineCommand {
    fn execute(&self, text_editor_lines: &mut LinesGapBuffer);
    fn undo(&self, text_editor_lines: &mut LinesGapBuffer);
}
pub struct AddLineCommand {
    index: usize,
}
impl TextEditorLineCommand for AddLineCommand {
    fn execute(&self, text_editor_lines: &mut LinesGapBuffer) {
        text_editor_lines.add_item(self.index);
    }

    fn undo(&self, text_editor_lines: &mut LinesGapBuffer) {
        text_editor_lines.remove_item(self.index);
    }
}
pub struct RemoveLineCommand {
    index:usize
}
impl TextEditorLineCommand for RemoveLineCommand {
    fn execute(&self, text_editor_lines: &mut LinesGapBuffer) {
        text_editor_lines.remove_item(self.index);
    }

    fn undo(&self, text_editor_lines: &mut LinesGapBuffer) {
        text_editor_lines.add_item(self.index);
    }
}
pub struct SplitLineCommand {
    index:usize,
    cut_position:usize
}
impl TextEditorLineCommand for SplitLineCommand{
    fn execute(&self, text_editor_lines: &mut LinesGapBuffer) {
        text_editor_lines.split_a_line(self.index, self.cut_position);
    }

    fn undo(&self, text_editor_lines: &mut LinesGapBuffer) {
        text_editor_lines.merge_two_lines(self.index+1);
    }
}
pub struct MergeLineCommand {
    index:usize,
    content_merged_len:usize
}
impl TextEditorLineCommand for MergeLineCommand{
   fn execute(&self, text_editor_lines: &mut LinesGapBuffer) {
        text_editor_lines.merge_two_lines(self.index);
    }
    fn undo(&self, text_editor_lines: &mut LinesGapBuffer) {
        text_editor_lines.split_a_line(self.index, self.content_merged_len);
    }

    
}
pub struct RefreshLineCommand {
    index:usize,
    offsets:(usize,usize),
    new_content:String,
}





impl TextEditorLineCommand for RefreshLineCommand{
    fn execute(&self, text_editor_lines: &mut LinesGapBuffer) {
        todo!()
    }

    fn undo(&self, text_editor_lines: &mut LinesGapBuffer) {
        todo!()
    }
}
