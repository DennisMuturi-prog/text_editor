use std::{cmp::max, io};

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

use crate::rope::{Node, build_rope, collect_string, insert, remove};
#[derive(Default)]
pub struct App {
    text: String,
    exit: bool,
    mode: Mode,
    row_number: usize,
    column_number: usize,
    rope: Option<Box<Node>>,
    index:usize,
    lines_widths:Vec<usize>
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
        let content:Vec<&str> = binding.graphemes( true).collect::<Vec<&str>>();
        let lines_widths=get_line_widths(&starting_string).0;
        Self {
            rope: Some(build_rope(&content, 0, content.len()-1).0),
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
        if let Some(old_one) = old_rope {
            let new_rope = remove(old_one, self.index.saturating_sub(1), 1);
            self.text.clear();
            collect_string(&new_rope, &mut self.text);
            self.rope = Some(new_rope);
            self.lines_widths[self.row_number]-=1;
        }
        self.move_cursor_left();
    }
    fn add_char(&mut self, value: char) {
        let old_rope = self.rope.take();
        if let Some(old_one) = old_rope {
            let new_rope = insert(old_one, self.index, value.to_string());
            self.text.clear();
            collect_string(&new_rope, &mut self.text);
            self.rope = Some(new_rope);
            self.lines_widths[self.row_number]+=1;  
        }
        self.move_cursor_right();
    }
    
    fn jump_to_new_line(&mut self) {
        let old_rope = self.rope.take();
        if let Some(old_one) = old_rope {
            
            let new_rope = insert(old_one, self.index, "\n".to_string());
            self.text.clear();
            collect_string(&new_rope, &mut self.text);
            self.rope = Some(new_rope);
            self.index+=1;
        }
        self.move_cursor_down();
    }

    fn move_cursor_left(&mut self) {
        self.index=self.index.saturating_sub(1);
        if self.column_number==0{
            self.column_number=self.lines_widths[self.row_number.saturating_sub(1)];
            self.row_number=self.row_number.saturating_sub(1);
        }else{
            let cursor_moved_left = self.column_number-1;
            self.column_number = self.clamp_cursor(cursor_moved_left); 
        }
        
    }
    fn move_cursor_down(&mut self) {
        self.column_number=0;
        let cursor_moved_down = self.row_number.saturating_add(1);
        self.row_number = cursor_moved_down;
    }

    fn move_cursor_right(&mut self) {
        self.index+=1;
        let cursor_moved_right = self.column_number.saturating_add(1);
        self.column_number = self.clamp_cursor(cursor_moved_right);
    }
    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.lines_widths[self.row_number])
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
        if let Mode::Editing = self.mode { frame.set_cursor_position(Position::new(
            // Draw the cursor at the current position in the input field.
            // This position is can be controlled via the left and right arrow key
            chunks[1].x + self.column_number as u16 + 1,
            // Move one line down, from the border to the input line
            chunks[1].y + 1+self.row_number as u16,
        )) }
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
                format!("column {} row {} index:{}", self.column_number, self.row_number,self.index),
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
        if let Event::Key(key) = event::read()? {
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
                    KeyCode::Left => self.move_cursor_left(),
                    KeyCode::Right => self.move_cursor_right(),
                    _ => {}
                },
                _ => {}
            }
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

pub fn get_line_widths(content:&str)->(Vec<usize>,usize){
    let mut lines_widths:Vec<usize>=vec![0;9];
    // max(100,content.len())/5
    let mut i=0;
    for (index,line_count) in content.lines().map(|a|a.len()).enumerate(){
        lines_widths[index]=line_count;  
        i+=1;
    };
    (lines_widths,i)
    
}
