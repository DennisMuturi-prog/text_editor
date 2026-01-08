use crate::app::get_line_widths;
#[derive(Default)]
pub struct GapBuffer {
    buffer: Vec<usize>,
    starting_of_gap: usize,
    ending_of_gap: usize,
}

impl GapBuffer {
    
    pub fn new(content: &str) -> Self {
        let (buffer,starting_of_gap,ending_of_gap) = get_line_widths(content);
        Self {
            buffer,
            starting_of_gap,
            ending_of_gap
        }
    }
    pub fn new_2(content: String) -> Self {
        let (buffer, no_of_lines,_) = get_line_widths(&content);
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
    
    pub fn increase(&mut self, index: usize) -> Option<()> {
        if index >= self.buffer.len() - ((self.ending_of_gap - self.starting_of_gap) + 1) {
            return None;
        }

        if index < self.starting_of_gap {
            self.buffer[index]+=1;
            Some(())
        } else {
            let offset = index - self.starting_of_gap + 1;
            let new_index = self.ending_of_gap + offset;
            self.buffer[new_index]+=1;
            Some(())
        }
    }
    pub fn decrease(&mut self, index: usize) -> Option<()> {
        if index >= self.buffer.len() - ((self.ending_of_gap - self.starting_of_gap) + 1) {
            return None;
        }

        if index < self.starting_of_gap {
            self.buffer[index]-=1;
            Some(())
        } else {
            let offset = index - self.starting_of_gap + 1;
            let new_index = self.ending_of_gap + offset;
            self.buffer[new_index]-=1;
            Some(())
        }
    }
    fn resize(&mut self){
        let previous_len=self.buffer.len();
        for _ in 0..previous_len{
            self.buffer.push(0);
        }
        self.starting_of_gap=previous_len;
        self.ending_of_gap=self.buffer.len()-1; 
    }
    pub fn add_item(&mut self, index: usize) {
        if self.ending_of_gap<self.starting_of_gap{
            self.resize();
        }
        if index == self.starting_of_gap {
            self.buffer[index] = 0;
            self.starting_of_gap += 1;
            return;
        }
        let gap_len = (self.ending_of_gap - self.starting_of_gap) + 1;
        if index > self.starting_of_gap {
            for offset in 0..index - self.starting_of_gap {
                let src_index = self.ending_of_gap + offset + 1;
                let dest_index = self.starting_of_gap + offset;
                let item = self.buffer[src_index];
                self.buffer[dest_index] = item;
                // self.buffer[src_index] = 0;
            }
        } else {
            for src_index in (index..self.starting_of_gap).rev(){
                let distance_from_start_of_gap=self.starting_of_gap-src_index;
                let dest_index = self.ending_of_gap - (distance_from_start_of_gap-1);
                let item = self.buffer[src_index];
                self.buffer[dest_index] = item;
                // self.buffer[src_index] = 0;
            }
        }
        self.buffer[index] =0;
        self.starting_of_gap = index + 1;
        self.ending_of_gap = self.starting_of_gap + gap_len - 2;
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
