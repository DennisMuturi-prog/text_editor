use ratatui::text::Line;

use crate::{
    app::{TextEditorLine, TypeOfLine, generate_lines, get_line_widths},
    text_representation::TextRepresentation,
};
#[derive(Default)]
pub struct GapBuffer {
    buffer: Vec<usize>,
    starting_of_gap: usize,
    ending_of_gap: usize,
}

impl GapBuffer {
    pub fn new(content: &str) -> Self {
        let (buffer, starting_of_gap, ending_of_gap) = get_line_widths(content);
        Self {
            buffer,
            starting_of_gap,
            ending_of_gap,
        }
    }
    pub fn new_2(content: String) -> Self {
        let (buffer, no_of_lines, _) = get_line_widths(&content);
        let current_len = buffer.len();
        Self {
            buffer,
            starting_of_gap: no_of_lines,
            ending_of_gap: current_len - 1,
        }
    }

    pub fn index(&mut self, index: usize) -> Option<usize> {
        if self.ending_of_gap < self.starting_of_gap {
            self.resize();
        }
        if index >= self.buffer.len() - ((self.ending_of_gap - self.starting_of_gap) + 1) {
            return None;
        }

        if index < self.starting_of_gap {
            Some(self.buffer[index])
        } else {
            let offset = index - self.starting_of_gap + 1;
            let new_index = self.ending_of_gap + offset;
            Some(self.buffer[new_index])
        }
    }
    pub fn find_where_rope_index_fits(&mut self, rope_index: usize) -> (usize, usize) {
        if self.ending_of_gap < self.starting_of_gap {
            self.resize();
        }
        let mut rope_index = rope_index as i32;
        for i in 0..self.starting_of_gap {
            rope_index -= (self.buffer[i]) as i32;
            if rope_index <= 0 {
                return (i, (self.buffer[i] - rope_index.unsigned_abs() as usize));
            }
            rope_index -= 1;
        }
        let before_index = self.starting_of_gap;
        for i in self.ending_of_gap + 1..self.buffer.len() {
            let after_index = i - self.ending_of_gap;
            let final_index = before_index + after_index - 1;
            rope_index -= (self.buffer[i]) as i32;
            if rope_index <= 0 {
                return (
                    final_index,
                    (self.buffer[i] - rope_index.unsigned_abs() as usize),
                );
            }
            rope_index -= 1;
        }
        (0, 0)
    }

    pub fn increase(&mut self, index: usize) -> Option<()> {
        if self.ending_of_gap < self.starting_of_gap {
            self.resize();
        }
        if index >= self.buffer.len() - ((self.ending_of_gap - self.starting_of_gap) + 1) {
            return None;
        }

        if index < self.starting_of_gap {
            self.buffer[index] += 1;
            Some(())
        } else {
            let offset = index - self.starting_of_gap + 1;
            let new_index = self.ending_of_gap + offset;
            self.buffer[new_index] += 1;
            Some(())
        }
    }
    pub fn length_up_to_non_inclusive_index(&mut self, index: usize) -> usize {
        let mut index = index;
        if self.ending_of_gap < self.starting_of_gap {
            self.resize();
        }
        let content_len = self.buffer.len() - ((self.ending_of_gap - self.starting_of_gap) + 1);
        if index >= content_len {
            index = content_len;
        }

        if index < self.starting_of_gap {
            self.buffer[0..index].iter().sum::<usize>()
        } else {
            let offset = index - self.starting_of_gap + 1;
            let new_index = self.ending_of_gap + offset;

            let sum_before_gap = self.buffer[0..self.starting_of_gap()].iter().sum::<usize>();
            let sum_after_gap = self.buffer[self.ending_of_gap + 1..new_index]
                .iter()
                .sum::<usize>();
            sum_before_gap + sum_after_gap
        }
    }
    pub fn length(&mut self) -> usize {
        if self.ending_of_gap < self.starting_of_gap {
            self.resize();
        }
        self.buffer.len() - ((self.ending_of_gap - self.starting_of_gap) + 1)
    }
    pub fn increase_with_count(&mut self, index: usize, count: usize) -> Option<()> {
        if self.ending_of_gap < self.starting_of_gap {
            self.resize();
        }
        if index >= self.buffer.len() - ((self.ending_of_gap - self.starting_of_gap) + 1) {
            return None;
        }

        if index < self.starting_of_gap {
            self.buffer[index] += count;
            Some(())
        } else {
            let offset = index - self.starting_of_gap + 1;
            let new_index = self.ending_of_gap + offset;
            self.buffer[new_index] += count;
            Some(())
        }
    }
    pub fn decrease_with_count(&mut self, index: usize, count: usize) -> Option<()> {
        if self.ending_of_gap < self.starting_of_gap {
            self.resize();
        }
        if index >= self.buffer.len() - ((self.ending_of_gap - self.starting_of_gap) + 1) {
            return None;
        }

        if index < self.starting_of_gap {
            self.buffer[index] = self.buffer[index].saturating_sub(count);
            Some(())
        } else {
            let offset = index - self.starting_of_gap + 1;
            let new_index = self.ending_of_gap + offset;
            self.buffer[new_index] = self.buffer[new_index].saturating_sub(count);
            Some(())
        }
    }
    pub fn decrease(&mut self, index: usize) -> Option<()> {
        if self.ending_of_gap < self.starting_of_gap {
            self.resize();
        }
        if index >= self.buffer.len() - ((self.ending_of_gap - self.starting_of_gap) + 1) {
            return None;
        }

        if index < self.starting_of_gap {
            self.buffer[index] = self.buffer[index].saturating_sub(1);
            Some(())
        } else {
            let offset = index - self.starting_of_gap + 1;
            let new_index = self.ending_of_gap + offset;
            self.buffer[new_index] = self.buffer[new_index].saturating_sub(1);
            Some(())
        }
    }
    fn resize(&mut self) {
        let previous_len = self.buffer.len();
        for _ in 0..previous_len {
            self.buffer.push(999);
        }
        self.starting_of_gap = previous_len;
        self.ending_of_gap = self.buffer.len() - 1;
    }

    pub fn add_item_with_count(&mut self, index: usize, count: usize) {
        if self.ending_of_gap < self.starting_of_gap {
            self.resize();
        }
        if index == self.starting_of_gap {
            self.buffer[index] = count;
            self.starting_of_gap += 1;
            if self.starting_of_gap < self.buffer.len() {
                self.buffer[self.starting_of_gap] = 999;
            }
            return;
        }
        let gap_len = (self.ending_of_gap - self.starting_of_gap) + 1;
        if index > self.starting_of_gap {
            for offset in 0..index - self.starting_of_gap {
                let src_index = self.ending_of_gap + offset + 1;
                let dest_index = self.starting_of_gap + offset;
                let item = self.buffer[src_index];
                self.buffer[dest_index] = item;
                self.buffer[src_index] = 999;
            }
        } else {
            for src_index in (index..self.starting_of_gap).rev() {
                let distance_from_start_of_gap = self.starting_of_gap - src_index;
                let dest_index = self.ending_of_gap - (distance_from_start_of_gap - 1);
                let item = self.buffer[src_index];
                self.buffer[dest_index] = item;
                self.buffer[src_index] = 999;
            }
        }
        self.buffer[index] = count;
        self.starting_of_gap = index + 1;
        self.ending_of_gap = self.starting_of_gap + gap_len - 2;
    }
    pub fn add_item(&mut self, index: usize) {
        if self.ending_of_gap < self.starting_of_gap {
            self.resize();
        }
        if index == self.starting_of_gap {
            self.buffer[index] = 0;
            self.starting_of_gap += 1;
            if self.starting_of_gap < self.buffer.len() {
                self.buffer[self.starting_of_gap] = 999;
            }
            return;
        }
        let gap_len = (self.ending_of_gap - self.starting_of_gap) + 1;
        if index > self.starting_of_gap {
            for offset in 0..index - self.starting_of_gap {
                let src_index = self.ending_of_gap + offset + 1;
                let dest_index = self.starting_of_gap + offset;
                let item = self.buffer[src_index];
                self.buffer[dest_index] = item;
                self.buffer[src_index] = 999;
            }
        } else {
            for src_index in (index..self.starting_of_gap).rev() {
                let distance_from_start_of_gap = self.starting_of_gap - src_index;
                let dest_index = self.ending_of_gap - (distance_from_start_of_gap - 1);
                let item = self.buffer[src_index];
                self.buffer[dest_index] = item;
                self.buffer[src_index] = 999;
            }
        }
        self.buffer[index] = 0;
        self.starting_of_gap = index + 1;
        self.ending_of_gap = self.starting_of_gap + gap_len - 2;
    }

    pub fn remove_item(&mut self, index: usize) -> Option<usize> {
        if self.ending_of_gap < self.starting_of_gap {
            self.resize();
        }
        if index >= self.buffer.len() - ((self.ending_of_gap - self.starting_of_gap) + 1) {
            return None;
        }

        let index = {
            if index < self.starting_of_gap {
                index
            } else {
                let offset = index - self.starting_of_gap + 1;
                self.ending_of_gap + offset
            }
        };
        let value_to_be_removed = self.buffer[index];

        if index == self.starting_of_gap {
            return None;
        }
        if index > self.starting_of_gap {
            for offset in 0..(index - self.ending_of_gap) - 1 {
                let src_index = self.ending_of_gap + offset + 1;
                let dest_index = self.starting_of_gap + offset;
                let item = self.buffer[src_index];
                self.buffer[dest_index] = item;
                self.buffer[src_index] = 999;
            }
            self.starting_of_gap += (index - self.ending_of_gap) - 1;
            self.ending_of_gap = index;
            self.buffer[index] = 999;
            Some(value_to_be_removed)
        } else {
            for offset in 0..(self.starting_of_gap - index) - 1 {
                let src_index = self.starting_of_gap - (offset + 1);
                let dest_index = self.ending_of_gap - offset;
                let item = self.buffer[src_index];
                self.buffer[dest_index] = item;
                self.buffer[src_index] = 999;
            }
            self.ending_of_gap -= (self.starting_of_gap - index) - 1;
            self.starting_of_gap = index;
            self.buffer[index] = 999;
            Some(value_to_be_removed)
        }
    }

    pub fn buffer(&self) -> &[usize] {
        &self.buffer
    }

    pub fn starting_of_gap(&self) -> usize {
        self.starting_of_gap
    }

    pub fn ending_of_gap(&self) -> usize {
        self.ending_of_gap
    }
}
#[derive(Default)]
pub struct LinesGapBuffer {
    buffer: Vec<TextEditorLine>,
    starting_of_gap: usize,
    ending_of_gap: usize,
}

impl LinesGapBuffer {
    pub fn new(content: &str, width: usize) -> Self {
        let (buffer, starting_of_gap, ending_of_gap) = generate_lines(content, width);
        Self {
            buffer,
            starting_of_gap,
            ending_of_gap,
        }
    }

    pub fn index(&self, index: usize) -> Option<usize> {
        if index >= self.buffer.len() - ((self.ending_of_gap - self.starting_of_gap) + 1) {
            return None;
        }

        if index < self.starting_of_gap {
            Some(self.buffer[index].get_line_length())
        } else {
            let offset = index - self.starting_of_gap + 1;
            let new_index = self.ending_of_gap + offset;
            Some(self.buffer[new_index].get_line_length())
        }
    }
    pub fn clear(&mut self, index: usize) -> Option<()> {
        if index >= self.buffer.len() - ((self.ending_of_gap - self.starting_of_gap) + 1) {
            return None;
        }

        if index < self.starting_of_gap {
            self.buffer[index].clear_line();
            Some(())
        } else {
            let offset = index - self.starting_of_gap + 1;
            let new_index = self.ending_of_gap + offset;
            self.buffer[new_index].clear_line();
            Some(())
        }
    }

    pub fn index_for_offset(&self, index: usize) -> Option<usize> {
        if index >= self.buffer.len() - ((self.ending_of_gap - self.starting_of_gap) + 1) {
            return None;
        }

        if index < self.starting_of_gap {
            Some(self.buffer[index].get_line_length_for_offset())
        } else {
            let offset = index - self.starting_of_gap + 1;
            let new_index = self.ending_of_gap + offset;
            Some(self.buffer[new_index].get_line_length_for_offset())
        }
    }
    pub fn find_where_rope_index_fits(&self, rope_index: usize) -> (usize, usize) {
        let mut rope_index = rope_index as i32;
        for i in 0..self.starting_of_gap {
            rope_index -= (self.buffer[i].get_line_length()) as i32;
            if rope_index <= 0 {
                return (
                    i,
                    (self.buffer[i].get_line_length() - rope_index.unsigned_abs() as usize),
                );
            }
            rope_index -= 1;
        }
        let before_index = self.starting_of_gap;
        for i in self.ending_of_gap + 1..self.buffer.len() {
            let after_index = i - self.ending_of_gap;
            let final_index = before_index + after_index - 1;
            rope_index -= (self.buffer[i].get_line_length()) as i32;
            if rope_index <= 0 {
                return (
                    final_index,
                    (self.buffer[i].get_line_length() - rope_index.unsigned_abs() as usize),
                );
            }
            rope_index -= 1;
        }
        (0, 0)
    }
    pub fn length_up_to_non_inclusive_index(&self, index: usize) -> usize {
        let mut index = index;

        let content_len = self.buffer.len() - ((self.ending_of_gap - self.starting_of_gap) + 1);
        if index >= content_len {
            index = content_len;
        }

        if index < self.starting_of_gap {
            self.buffer[0..index]
                .iter()
                .map(|a| a.get_line_length())
                .sum::<usize>()
        } else {
            let offset = index - self.starting_of_gap + 1;
            let new_index = self.ending_of_gap + offset;

            let sum_before_gap = self.buffer[0..self.starting_of_gap()]
                .iter()
                .map(|a| a.get_line_length())
                .sum::<usize>();
            let sum_after_gap = self.buffer[self.ending_of_gap + 1..new_index]
                .iter()
                .map(|a| a.get_line_length())
                .sum::<usize>();
            sum_before_gap + sum_after_gap
        }
    }
    pub fn length(&self) -> usize {
        self.buffer.len() - ((self.ending_of_gap - self.starting_of_gap) + 1)
    }

    fn resize(&mut self) {
        let previous_len = self.buffer.len();
        for _ in 0..previous_len {
            self.buffer.push(TextEditorLine::default());
        }
        self.starting_of_gap = previous_len;
        self.ending_of_gap = self.buffer.len() - 1;
    }

    pub fn find_offsets_for_line(&self, index: usize) -> (usize, usize, bool) {
        let mut landmark_offset = 0;

        let mut starting = 0;
        while starting < index {
            landmark_offset += self.index_for_offset(starting).unwrap_or_default();
            starting += 1;
        }
        let starting = landmark_offset;
        let line_length = self.index(index).unwrap();
        let ending = landmark_offset + self.index(index).unwrap().saturating_sub(1);
        (starting, ending, line_length > 0)
    }
    pub fn add_item(&mut self, index: usize) {
        if self.ending_of_gap < self.starting_of_gap {
            self.resize();
        }

        if index == self.starting_of_gap {
            self.buffer[index] = TextEditorLine::default();
            self.starting_of_gap += 1;
            if self.ending_of_gap < self.starting_of_gap {
                self.resize();
            }
            return;
        }
        let gap_len = (self.ending_of_gap - self.starting_of_gap) + 1;
        if index > self.starting_of_gap {
            for offset in 0..index - self.starting_of_gap {
                let src_index = self.ending_of_gap + offset + 1;
                let dest_index = self.starting_of_gap + offset;
                self.buffer.swap(src_index, dest_index);
            }
        } else {
            for src_index in (index..self.starting_of_gap).rev() {
                let distance_from_start_of_gap = self.starting_of_gap - src_index;
                let dest_index = self.ending_of_gap - (distance_from_start_of_gap - 1);
                self.buffer.swap(src_index, dest_index);
            }
        }
        self.buffer[index] = TextEditorLine::default();
        self.starting_of_gap = index + 1;
        self.ending_of_gap = self.starting_of_gap + gap_len - 2;
        if self.ending_of_gap < self.starting_of_gap {
            self.resize();
        }
    }
    pub fn add_item_with_content(
        &mut self,
        index: usize,
        content: String,
        type_of_line: TypeOfLine,
    ) {
        if self.ending_of_gap < self.starting_of_gap {
            self.resize();
        }

        if index == self.starting_of_gap {
            self.buffer[index] = TextEditorLine::new(content, type_of_line);
            self.starting_of_gap += 1;
            if self.ending_of_gap < self.starting_of_gap {
                self.resize();
            }
            return;
        }
        let gap_len = (self.ending_of_gap - self.starting_of_gap) + 1;
        if index > self.starting_of_gap {
            for offset in 0..index - self.starting_of_gap {
                let src_index = self.ending_of_gap + offset + 1;
                let dest_index = self.starting_of_gap + offset;
                self.buffer.swap(src_index, dest_index);
            }
        } else {
            for src_index in (index..self.starting_of_gap).rev() {
                let distance_from_start_of_gap = self.starting_of_gap - src_index;
                let dest_index = self.ending_of_gap - (distance_from_start_of_gap - 1);
                self.buffer.swap(src_index, dest_index);
            }
        }
        self.buffer[index] = TextEditorLine::new(content, type_of_line);
        self.starting_of_gap = index + 1;
        self.ending_of_gap = self.starting_of_gap + gap_len - 2;
        if self.ending_of_gap < self.starting_of_gap {
            self.resize();
        }
    }
    pub fn change_line(
        &mut self,
        index: usize,
        bounds: (usize, usize),
        text_representation: &dyn TextRepresentation,
    ) -> Option<()> {
        if self.ending_of_gap < self.starting_of_gap {
            self.resize();
        }
        if index >= self.buffer.len() - ((self.ending_of_gap - self.starting_of_gap) + 1) {
            return None;
        }
        if index < self.starting_of_gap {
            self.buffer[index].change_line(bounds, text_representation);
            Some(())
        } else {
            let index_offset = index - self.starting_of_gap + 1;
            let new_index = self.ending_of_gap + index_offset;
            self.buffer[new_index].change_line(bounds, text_representation);
            Some(())
        }
    }
    fn is_parent_or_independent(&self, index: usize) -> bool {
        if index >= self.buffer.len() - ((self.ending_of_gap - self.starting_of_gap) + 1) {
            return true;
        }

        if index < self.starting_of_gap {
            self.buffer[index].is_parent_or_independent()
        } else {
            let offset = index - self.starting_of_gap + 1;
            let new_index = self.ending_of_gap + offset;
            self.buffer[new_index].is_parent_or_independent()
        }
    }
    pub fn split_a_line_with_wrapping(&mut self, index: usize, cut_position: usize) -> Option<()> {
        self.split_a_line(index, cut_position);
    }
    pub fn split_a_line(&mut self, index: usize, cut_position: usize) -> Option<()> {
        if self.ending_of_gap < self.starting_of_gap {
            self.resize();
        }
        if index >= self.buffer.len() - ((self.ending_of_gap - self.starting_of_gap) + 1) {
            return None;
        }

        if index < self.starting_of_gap {
            let cut_content = self.buffer[index].split_line(cut_position);
            let type_of_line = if self.is_parent_or_independent(index + 1) {
                TypeOfLine::Terminator
            } else {
                TypeOfLine::Child
            };
            self.add_item_with_content(index + 1, cut_content, type_of_line);
            Some(())
        } else {
            let index_offset = index - self.starting_of_gap + 1;
            let new_index = self.ending_of_gap + index_offset;
            let cut_content = self.buffer[new_index].split_line(cut_position);
            let type_of_line = if self.is_parent_or_independent(index + 1) {
                TypeOfLine::Terminator
            } else {
                TypeOfLine::Child
            };
            self.add_item_with_content(index + 1, cut_content, type_of_line);

            Some(())
        }
    }
    pub fn merge_two_lines(&mut self, index: usize) -> Option<()> {
        if self.ending_of_gap < self.starting_of_gap {
            self.resize();
        }
        if index >= self.buffer.len() - ((self.ending_of_gap - self.starting_of_gap) + 1) {
            return None;
        }
        let removed_line = self.remove_item(index)?;
        self.increase_upper_line(index - 1, removed_line.line());
        Some(())
    }

    pub fn remove_item(&mut self, index: usize) -> Option<TextEditorLine> {
        if self.ending_of_gap < self.starting_of_gap {
            self.resize();
        }
        if index >= self.buffer.len() - ((self.ending_of_gap - self.starting_of_gap) + 1) {
            return None;
        }

        let index = {
            if index < self.starting_of_gap {
                index
            } else {
                let offset = index - self.starting_of_gap + 1;
                self.ending_of_gap + offset
            }
        };
        let value_to_be_removed = self.buffer[index].clone();

        if index == self.starting_of_gap {
            return None;
        }
        if index > self.starting_of_gap {
            for offset in 0..(index - self.ending_of_gap) - 1 {
                let src_index = self.ending_of_gap + offset + 1;
                let dest_index = self.starting_of_gap + offset;
                self.buffer.swap(src_index, dest_index);
            }
            self.starting_of_gap += (index - self.ending_of_gap) - 1;
            self.ending_of_gap = index;
        } else {
            for offset in 0..(self.starting_of_gap - index) - 1 {
                let src_index = self.starting_of_gap - (offset + 1);
                let dest_index = self.ending_of_gap - offset;
                self.buffer.swap(src_index, dest_index);
            }
            self.ending_of_gap -= (self.starting_of_gap - index) - 1;
            self.starting_of_gap = index;
        }

        Some(value_to_be_removed)
    }

    fn increase_upper_line(&mut self, index: usize, content: &str) -> Option<()> {
        if self.ending_of_gap < self.starting_of_gap {
            self.resize();
        }
        if index >= self.buffer.len() - ((self.ending_of_gap - self.starting_of_gap) + 1) {
            return None;
        }

        if index < self.starting_of_gap {
            self.buffer[index].add_to_line(content);
            Some(())
        } else {
            let index_offset = index - self.starting_of_gap + 1;
            let new_index = self.ending_of_gap + index_offset;
            self.buffer[new_index].add_to_line(content);
            Some(())
        }
    }

    pub fn starting_of_gap(&self) -> usize {
        self.starting_of_gap
    }
    pub fn get_lines(&self) -> Vec<Line<'_>> {
        let mut lines = Vec::new();
        for line in self.buffer.iter().take(self.starting_of_gap) {
            lines.push(Line::raw(line.line()));
        }
        for line in self.buffer.iter().skip(self.ending_of_gap + 1) {
            lines.push(Line::raw(line.line()));
        }
        lines
    }

    pub fn ending_of_gap(&self) -> usize {
        self.ending_of_gap
    }

    pub fn buffer(&self) -> &[TextEditorLine] {
        &self.buffer
    }
}
