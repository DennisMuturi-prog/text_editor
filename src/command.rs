use std::cell::RefCell;

use unicode_segmentation::UnicodeSegmentation;

use crate::{
    gap_buffer::{GapBuffer, LinesGapBuffer},
    rope::{Node, insert, remove},
    text_representation::TextRepresentation,
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
    fn execute(&self, line_command_ctx: LineCommandContext);
    fn undo(&self, line_command_ctx: LineCommandContext);
}
pub struct AddLineCommand {
    index: usize,
}
impl AddLineCommand {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}
impl TextEditorLineCommand for AddLineCommand {
    fn execute(&self, line_command_ctx: LineCommandContext) {
        line_command_ctx.text_editor_lines.add_item(self.index);
    }

    fn undo(&self, line_command_ctx: LineCommandContext) {
        line_command_ctx.text_editor_lines.remove_item(self.index);
    }
}
pub struct RemoveLineCommand {
    index: usize,
}
impl RemoveLineCommand {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}
impl TextEditorLineCommand for RemoveLineCommand {
    fn execute(&self, line_command_ctx: LineCommandContext) {
        line_command_ctx.text_editor_lines.remove_item(self.index);
    }

    fn undo(&self, line_command_ctx: LineCommandContext) {
        line_command_ctx.text_editor_lines.add_item(self.index);
    }
}
pub struct SplitLineCommand {
    index: usize,
    cut_position: usize,
}
impl SplitLineCommand {
    pub fn new(index: usize, cut_position: usize) -> Self {
        Self {
            index,
            cut_position,
        }
    }
}
impl TextEditorLineCommand for SplitLineCommand {
    fn execute(&self, line_command_ctx: LineCommandContext) {
        line_command_ctx
            .text_editor_lines
            .split_a_line(self.index, self.cut_position);
    }

    fn undo(&self, line_command_ctx: LineCommandContext) {
        line_command_ctx
            .text_editor_lines
            .merge_two_lines(self.index + 1);
    }
}
pub struct MergeLineCommand {
    index: usize,
    content_merged_len: usize,
}
impl MergeLineCommand {
    pub fn new(index: usize, content_merged_len: usize) -> Self {
        Self {
            index,
            content_merged_len,
        }
    }
}
impl TextEditorLineCommand for MergeLineCommand {
    fn execute(&self, line_command_ctx: LineCommandContext) {
        line_command_ctx
            .text_editor_lines
            .merge_two_lines(self.index)
            .unwrap();
    }
    fn undo(&self, line_command_ctx: LineCommandContext) {
        line_command_ctx
            .text_editor_lines
            .split_a_line(self.index, self.content_merged_len);
    }
}
pub struct InsertIntoLineCommand {
    index: usize,
    initial_offsets: (usize, usize),
    content_len_added: usize,
}
impl InsertIntoLineCommand {
    pub fn new(index: usize, content_len_added: usize, initial_offsets: (usize, usize)) -> Self {
        Self {
            index,
            content_len_added,
            initial_offsets,
        }
    }
}

impl TextEditorLineCommand for InsertIntoLineCommand {
    fn execute(&self, line_command_ctx: LineCommandContext) {
        line_command_ctx.text_editor_lines.change_line(
            self.index,
            (
                self.initial_offsets.0,
                self.initial_offsets.1 + self.content_len_added,
            ),
            line_command_ctx.text_representation,
        );
    }

    fn undo(&self, line_command_ctx: LineCommandContext) {
        line_command_ctx.text_editor_lines.change_line(
            self.index,
            self.initial_offsets,
            line_command_ctx.text_representation,
        );
    }
}
pub struct RemoveFromLineCommand {
    index: usize,
    initial_offsets: (usize, usize),
    content_len_removed: usize,
}
impl RemoveFromLineCommand {
    pub fn new(index: usize, content_len_removed: usize, initial_offsets: (usize, usize)) -> Self {
        Self {
            index,
            content_len_removed,
            initial_offsets,
        }
    }
}

impl TextEditorLineCommand for RemoveFromLineCommand {
    fn execute(&self, line_command_ctx: LineCommandContext) {
        line_command_ctx.text_editor_lines.change_line(
            self.index,
            (
                self.initial_offsets.0,
                self.initial_offsets
                    .1
                    .saturating_sub(self.content_len_removed),
            ),
            line_command_ctx.text_representation,
        );
    }

    fn undo(&self, line_command_ctx: LineCommandContext) {
        line_command_ctx.text_editor_lines.change_line(
            self.index,
            self.initial_offsets,
            line_command_ctx.text_representation,
        );
    }
}
pub struct LineCommandContext<'a> {
    text_editor_lines: &'a mut LinesGapBuffer,
    text_representation: &'a dyn TextRepresentation,
}
impl<'a> LineCommandContext<'a> {
    pub fn new(
        text_editor_lines: &'a mut LinesGapBuffer,
        text_representation: &'a dyn TextRepresentation,
    ) -> Self {
        Self {
            text_editor_lines,
            text_representation,
        }
    }
}

