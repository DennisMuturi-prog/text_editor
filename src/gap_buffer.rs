use crate::{app::get_line_widths};
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
            rope_index-=1;
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
            rope_index-=1;
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
