use crate::app::get_line_widths;

pub struct GapBuffer {
    buffer: Vec<usize>,
    starting_of_gap: usize,
    ending_of_gap: usize,
}

impl GapBuffer {
    pub fn new(content: String) -> Self {
        let (buffer, no_of_lines) = get_line_widths(&content);
        let current_len = buffer.len();
        Self {
            buffer,
            starting_of_gap: no_of_lines,
            ending_of_gap: current_len - 1,
        }
    }

    pub fn index(&self, index: usize) -> Option<usize> {
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
    pub fn add_item(&mut self, index: usize) {
        if index == self.starting_of_gap {
            self.buffer[index] = 999;
            self.starting_of_gap += 1;
            return;
        }
        let gap_len = (self.ending_of_gap - self.starting_of_gap) + 1;
        if index > self.starting_of_gap {
            for offset in 0..index - self.starting_of_gap {
                let src_index = self.ending_of_gap + offset + 1;
                let dest_index = self.starting_of_gap + offset;
                let item = self.buffer[src_index];
                // println!("Moving item {} from index {} to index {}", item, src_index, dest_index);
                self.buffer[dest_index] = item;
                self.buffer[src_index] = 0;
            }
        } else {
            let mut j = self.starting_of_gap - index;
            for offset in 0..self.starting_of_gap - index {
                let src_index = index + offset;
                let dest_index = self.ending_of_gap - (j - 1);
                let item = self.buffer[src_index];
                // println!("Moving item {} from index {} to index {}", item, src_index, dest_index);
                self.buffer[dest_index] = item;
                self.buffer[src_index] = 0;
                j -= 1;
            }
        }

        self.buffer[index] = 999;
        self.starting_of_gap = index + 1;
        self.ending_of_gap = self.starting_of_gap + (gap_len - 2);
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
