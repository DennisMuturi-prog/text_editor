use crate::text_representation::TextRepresentation;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, Default, Debug)]
pub struct TextEditorLine {
    line: String,
    type_of_line: TypeOfLine,
}
impl TextEditorLine {
    pub fn new(line: String, type_of_line: TypeOfLine) -> Self {
        Self { line, type_of_line }
    }
    pub fn change_line(
        &mut self,
        bounds: (usize, usize),
        text_representation: &dyn TextRepresentation,
    ) {
        text_representation.collect_substring(&mut self.line, bounds);
    }
    pub fn add_to_line(&mut self, new_content: &str) {
        self.line.push_str(new_content);
    }
    pub fn get_line_length(&self) -> usize {
        self.line.len()
    }
    pub fn get_line_length_for_offset(&self) -> usize {
        match self.type_of_line {
            TypeOfLine::Independent => self.line.len() + 1,
            TypeOfLine::Child => self.line.len(),
            TypeOfLine::Terminator => self.line.len() + 1,
            TypeOfLine::Parent => self.line.len(),
        }
    }

    pub fn line(&self) -> &str {
        &self.line
    }
    pub fn clear_line(&mut self) {
        self.line.clear();
    }
    pub fn is_independent(&self) -> bool {
        matches!(self.type_of_line, TypeOfLine::Independent)
    }
    pub fn split_line(&mut self, cut_position: usize) -> String {
        let full_string = self.line.graphemes(true);
        let mut first_part: String = String::new();
        let mut second_part: String = String::new();
        for (index, line) in full_string.enumerate() {
            if index < cut_position {
                first_part.push_str(line);
            } else {
                second_part.push_str(line);
            }
        }

        self.line = first_part;
        second_part
    }

    pub fn type_of_line(&self) -> &TypeOfLine {
        &self.type_of_line
    }

    pub fn set_type_of_line(&mut self, type_of_line: TypeOfLine) {
        self.type_of_line = type_of_line;
    }
}
#[derive(Clone, Default, Debug)]
pub enum TypeOfLine {
    Parent,
    Child,
    #[default]
    Independent,
    Terminator,
}
pub fn generate_lines(content: &str, width: usize) -> (Vec<TextEditorLine>, usize, usize) {
    let lines: Vec<&str> = content.lines().collect();
    let lines_count = lines.len();
    let mut my_lines = Vec::with_capacity(lines_count * 3);
    for item in lines {
        if item.len() <= width {
            my_lines.push(TextEditorLine {
                line: item.to_string(),
                type_of_line: TypeOfLine::Independent,
            });
        } else {
            let graphemes_in_word = item.graphemes(true);
            let line_len = item.len();
            let line_count = (line_len / width) + 1;
            let mut lines_in_string = vec![
                TextEditorLine {
                    line: String::new(),
                    type_of_line: TypeOfLine::Child,
                };
                line_count
            ];
            for (index, letter) in graphemes_in_word.enumerate() {
                let index_in_lines_in_string = index / width;
                lines_in_string[index_in_lines_in_string]
                    .line
                    .push_str(letter);
            }

            let last_index = lines_in_string.len().saturating_sub(1);
            lines_in_string[0].type_of_line = TypeOfLine::Parent;
            lines_in_string[last_index].type_of_line = TypeOfLine::Terminator;
            my_lines.extend(lines_in_string.into_iter());
        }
    }
    let starting_of_gap = my_lines.len();
    let ending_of_gap = starting_of_gap + lines_count.saturating_sub(1);

    for _ in 0..lines_count {
        my_lines.push(TextEditorLine::default());
    }

    (my_lines, starting_of_gap, ending_of_gap)
}
