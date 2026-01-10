use std::{
    cmp::max,
    fs::{self, File},
    io,
};

use ptree::write_tree;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout, Position},
    prelude::Rect,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};
use unicode_segmentation::UnicodeSegmentation;

use crate::{
    gap_buffer::GapBuffer,
    rope::{Node, build_rope, collect_string, insert, remove},
};
#[derive(Default)]
pub struct App {
    text: String,
    exit: bool,
    mode: Mode,
    row_number: usize,
    column_number: usize,
    rope: Option<Box<Node>>,
    index: usize,
    lines_widths: GapBuffer,
}
#[derive(Default)]
enum Mode {
    #[default]
    Normal,
    Editing,
    Exiting,
}

impl App {
    pub fn new(starting_string: String) -> Self {
        let binding = starting_string.clone();
        let content: Vec<&str> = binding.graphemes(true).collect::<Vec<&str>>();
        let lines_widths = GapBuffer::new(&starting_string);
        let log_message = format!(
            "gap buffer is {:#?} starting is {} and ending is {}",
            lines_widths.buffer(),
            lines_widths.starting_of_gap(),
            lines_widths.ending_of_gap()
        );
        fs::write("log.txt", log_message).unwrap();
        Self {
            rope: Some(build_rope(&content, 0, content.len() - 1).0),
            text: starting_string,
            lines_widths,
            ..App::default()
        }
    }
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
    fn delete_char(&mut self) {
        let old_rope = self.rope.take();
        let mut count_to_offset = 0;
        if let Some(old_one) = old_rope {
            if self.column_number == 0 && self.row_number > 0 {
                if let Some(length_of_line_removed) = self.lines_widths.remove_item(self.row_number)
                {
                    self.lines_widths
                        .increase_with_count(self.row_number - 1, length_of_line_removed);
                    count_to_offset = length_of_line_removed;
                };
            } else if self.column_number == 0 && self.row_number == 0 {
                self.rope = Some(old_one);
                let log_message = format!(
                    "gap buffer is {:#?} starting is {} and ending is {}",
                    self.lines_widths.buffer(),
                    self.lines_widths.starting_of_gap(),
                    self.lines_widths.ending_of_gap()
                );
                fs::write("log2.txt", log_message).unwrap();
                return;
            } else {
                self.lines_widths.decrease(self.row_number);
            }
            let new_rope = remove(old_one, self.index.saturating_sub(1), 1);
            self.text.clear();
            collect_string(&new_rope, &mut self.text);
            self.rope = Some(new_rope);
        }
        self.move_cursor_left(count_to_offset);
        let log_message = format!(
            "gap buffer is {:#?} starting is {} and ending is {}",
            self.lines_widths.buffer(),
            self.lines_widths.starting_of_gap(),
            self.lines_widths.ending_of_gap()
        );
        fs::write("log.txt", log_message).unwrap();
    }
    fn add_char(&mut self, value: char) {
        let old_rope = self.rope.take();
        if let Some(old_one) = old_rope {
            let new_rope = insert(old_one, self.index, value.to_string());
            self.text.clear();
            collect_string(&new_rope, &mut self.text);
            self.rope = Some(new_rope);
            self.lines_widths.increase(self.row_number);
        }
        self.move_cursor_right();
        let log_message = format!(
            "gap buffer is {:#?} starting is {} and ending is {}",
            self.lines_widths.buffer(),
            self.lines_widths.starting_of_gap(),
            self.lines_widths.ending_of_gap()
        );
        fs::write("log.txt", log_message).unwrap();
    }
    fn paste(&mut self, value: String) {
        let old_rope = self.rope.take();
        if let Some(old_one) = old_rope {
            let length_of_paste_content=value.graphemes(true).count();
            let new_rope = insert(old_one, self.index, value);
            self.text.clear();
            collect_string(&new_rope, &mut self.text);
            self.rope = Some(new_rope);
            self.move_right_due_to_paste(length_of_paste_content);
        }
        let log_message = format!(
            "gap buffer is {:#?} starting is {} and ending is {}",
            self.lines_widths.buffer(),
            self.lines_widths.starting_of_gap(),
            self.lines_widths.ending_of_gap()
        );
        fs::write("log.txt", log_message).unwrap();
    }
    fn move_right_due_to_paste(&mut self,length:usize){
        self.index+=length;
        self.column_number+=length;
        self.lines_widths.increase_with_count(self.row_number,length);
    }

    fn jump_to_new_line(&mut self) {
        let old_rope = self.rope.take();
        if let Some(old_one) = old_rope {
            let new_rope = insert(old_one, self.index, "\n".to_string());
            self.text.clear();
            collect_string(&new_rope, &mut self.text);
            self.rope = Some(new_rope);
            let current_line_length = self.lines_widths.index(self.row_number).unwrap_or_default();

            if self.column_number < current_line_length {
                self.lines_widths.add_item_with_count(
                    self.row_number + 1,
                    current_line_length - self.column_number,
                );
                self.lines_widths
                    .decrease_with_count(self.row_number, current_line_length - self.column_number);
                self.index += 1;
            } else {
                self.index += 1;
                self.lines_widths.add_item(self.row_number + 1);
            }
        }
        self.move_cursor_down();
        let log_message = format!(
            "gap buffer is {:#?} starting is {} and ending is {}",
            self.lines_widths.buffer(),
            self.lines_widths.starting_of_gap(),
            self.lines_widths.ending_of_gap()
        );
        fs::write("log.txt", log_message).unwrap();
    }

    fn move_cursor_left(&mut self, offset: usize) {
        self.index = self.index.saturating_sub(1);
        if self.column_number == 0 {
            if self.row_number > 0 {
                self.column_number = self
                    .lines_widths
                    .index(self.row_number - 1)
                    .unwrap_or_default()
                    .saturating_sub(offset);
                self.row_number -= 1;
            }
        } else {
            let cursor_moved_left = self.column_number - 1;
            self.column_number = cursor_moved_left;
        }
    }
    fn move_cursor_down(&mut self) {
        self.column_number = 0;
        let cursor_moved_down = self.row_number.saturating_add(1);
        self.row_number = cursor_moved_down;
    }

    fn move_cursor_right(&mut self) {
        if self.column_number == self.lines_widths.index(self.row_number).unwrap_or_default() {
            if self.lines_widths.index(self.row_number + 1).is_some() {
                self.index += 1;
                self.row_number += 1;
                self.column_number = 0;
            } else {
                let value = self.lines_widths.index(self.row_number).unwrap_or_default();
                if value == 0 {
                    return;
                }
                self.jump_to_new_line();
            }
        } else {
            self.index += 1;
            let cursor_moved_right = self.column_number.saturating_add(1);
            self.column_number = cursor_moved_right;
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(3),
            ])
            .split(area);

        let title_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());

        let title = Paragraph::new(Text::styled(
            "Text editor",
            Style::default().fg(Color::Green),
        ))
        .block(title_block.clone());
        frame.render_widget(title, chunks[0]);

        let text_content =
            Paragraph::new(Text::styled(self.text(), Style::default().fg(Color::White)))
                .block(title_block);
        frame.render_widget(text_content, chunks[1]);
        if let Mode::Editing = self.mode {
            frame.set_cursor_position(Position::new(
                // Draw the cursor at the current position in the input field.
                // This position is can be controlled via the left and right arrow key
                chunks[1].x + self.column_number as u16 + 1,
                // Move one line down, from the border to the input line
                chunks[1].y + 1 + self.row_number as u16,
            ))
        }
        let footer_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[2]);

        let current_navigation_text = vec![
            // The first half of the text
            match self.mode {
                Mode::Normal => Span::styled("Normal Mode", Style::default().fg(Color::Green)),
                Mode::Editing => Span::styled("Editing Mode", Style::default().fg(Color::Yellow)),
                Mode::Exiting => Span::styled("Exiting", Style::default().fg(Color::LightRed)),
            }
            .to_owned(),
            // A white divider bar to separate the two sections
            Span::styled(" | ", Style::default().fg(Color::White)),
            // The final section of the text, with hints on what the user is editing
            Span::styled(
                format!(
                    "column {} row {} index:{}",
                    self.column_number, self.row_number, self.index
                ),
                Style::default().fg(Color::Green),
            ),
        ];

        let mode_footer = Paragraph::new(Line::from(current_navigation_text))
            .block(Block::default().borders(Borders::ALL));
        let current_keys_hint = {
            match self.mode {
                Mode::Normal => {
                    Span::styled("(q) to quit / (e) to edit", Style::default().fg(Color::Red))
                }
                Mode::Editing => Span::styled(
                    "(ESC) to go to normal mode",
                    Style::default().fg(Color::Red),
                ),
                Mode::Exiting => Span::styled("(q) to quit", Style::default().fg(Color::Red)),
            }
        };

        let key_notes_footer = Paragraph::new(Line::from(current_keys_hint))
            .block(Block::default().borders(Borders::ALL));

        frame.render_widget(mode_footer, footer_chunks[0]);
        frame.render_widget(key_notes_footer, footer_chunks[1]);

        if let Mode::Exiting = self.mode {
            // frame.render_widget(Clear, frame.area()); //this clears the entire screen and anything already drawn
            let popup_block = Block::default()
                .title("Y/N")
                .borders(Borders::NONE)
                .style(Style::default().bg(Color::DarkGray));

            let exit_text = Text::styled(
                "Would you save the file? (y/n)",
                Style::default().fg(Color::Red),
            );
            // the `trim: false` will stop the text from being cut off when over the edge of the block
            let exit_pop_up_area = centered_rect(60, 25, area);
            let exit_paragraph = Paragraph::new(exit_text)
                .block(popup_block)
                .wrap(Wrap { trim: false });
            frame.render_widget(exit_paragraph, exit_pop_up_area);
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key) => {
                if key.kind == event::KeyEventKind::Release {
                    // Skip events that are not KeyEventKind::Press
                    return Ok(());
                }
                match self.mode {
                    Mode::Normal => match key.code {
                        KeyCode::Char('e') => {
                            self.mode = Mode::Editing;
                        }
                        KeyCode::Char('q') => {
                            self.mode = Mode::Exiting;
                        }
                        _ => {}
                    },
                    Mode::Exiting => match key.code {
                        KeyCode::Char('y') => {
                            self.exit = true;
                            return Ok(());
                        }
                        KeyCode::Char('n') | KeyCode::Char('q') => {
                            self.exit = true;
                            return Ok(());
                        }
                        _ => {}
                    },
                    Mode::Editing if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Enter => {
                            self.jump_to_new_line();
                        }
                        KeyCode::Backspace => {
                            self.delete_char();
                        }
                        KeyCode::Esc => {
                            self.mode = Mode::Normal;
                        }
                        KeyCode::Tab => {}
                        KeyCode::Char(value) => {
                            self.add_char(value);
                        }
                        KeyCode::Left => self.move_cursor_left(0),
                        KeyCode::Right => self.move_cursor_right(),
                        _ => {}
                    },
                    _ => {}
                }
            },
            Event::Paste(pasted_string)=>{
                if let Mode::Editing=self.mode{
                    self.paste(pasted_string); 
                }
                
            },
            _ => (),
        }
        Ok(())
    }

    fn text(&self) -> &str {
        &self.text
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {}
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

pub fn get_line_widths(content: &str) -> (Vec<usize>, usize, usize) {
    //TODO add support for grapheme and utf8
    let mut lines: Vec<_> = content.lines().collect();
    let lines_counts = lines.len();
    let mut lines_widths: Vec<usize> = Vec::with_capacity(lines_counts * 5);

    if lines_counts == 1 {
        lines_widths.push(lines[0].len());
        let gap = (lines_counts * 2) - (lines_counts / 2);
        lines_widths.extend(std::iter::repeat_n(999, gap));
        return (lines_widths, 1, 2);
    }

    let second_part = lines.split_off(lines_counts / 2);
    // max(100,content.len())/5
    for line_count in lines.into_iter().map(|a| a.len()) {
        lines_widths.push(line_count);
    }
    let gap = (lines_counts * 2) - (lines_counts / 2);
    lines_widths.extend(std::iter::repeat_n(999, gap));

    for line_count in second_part.into_iter().map(|a| a.len()) {
        lines_widths.push(line_count);
    }
    (lines_widths, lines_counts / 2, (lines_counts * 2) - 1)
}
