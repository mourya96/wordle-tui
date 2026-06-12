use crate::{grid::Grid, words::Trie};
use crossterm::event::{
    self,
    Event::{self},
    KeyCode, KeyEvent, KeyEventKind,
};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Padding, Paragraph, Widget, Clear},
};
use std::io;
use crate::words::AnswerWordPicker;

pub struct App {
    answer_word: [char; 5],
    allowed_words: Trie,
    grid: Grid,
    current_row: usize,
    current_col: usize,
    exit: bool,
    answer_word_picker: AnswerWordPicker,
    game_state: GameState,
}

#[derive(PartialEq)]
enum GameState {
    InProgress,
    Won,
    Lost,
}

impl App {
    pub fn new(answer_word: [char; 5]) -> Self {
        let allowed_words: Vec<String> = include_str!("allowed_words.txt")
            .lines()
            .map(|l| l.to_string())
            .collect();

        let mut trie = Trie::new();
        for word in &allowed_words {
            trie.insert(word);
        }
        
        let words: Vec<String> = include_str!("answer_words.txt").lines().map(|l| l.to_string()).collect();
        let rng = rand::rng();
        let answer_word_picker = AnswerWordPicker::new(words, Box::new(rng));
        
        Self {
            answer_word,
            allowed_words: trie,
            grid: Grid::new(),
            current_row: 0,
            current_col: 0,
            exit: false,
            answer_word_picker,
            game_state: GameState::InProgress,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }
    
    fn draw(&self, frame: &mut Frame) {
        if self.game_state != GameState::InProgress {
            self.render_game_over(frame.area(), frame.buffer_mut());
        } else {
            frame.render_widget(self, frame.area());
        }
    }
    
    fn render_game_over(&self, area: Rect, buf: &mut Buffer) {
        let popup_width = 30;
        let popup_height = 5;
        
        let popup_area = Rect::new(
            area.x + area.width.saturating_sub(popup_width) / 2,
            area.y + area.height.saturating_sub(popup_height) / 2,
            popup_width,
            popup_height,
        );

        Clear.render(popup_area, buf);

        let message = if self.game_state == GameState::Won {
            "You Win! Yay!!!\nPlay again? (y/n)"
        } else {
            "Game Over! :(\nPlay again? (y/n)"
        };

        let popup_block = Block::bordered()
            .title(Line::from(" Game Over ").bold().centered())
            .border_set(border::THICK)
            .style(Style::default());

        Paragraph::new(message)
            .block(popup_block)
            .centered()
            .render(popup_area, buf);
    }
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if self.game_state != GameState::InProgress {
            match key_event.code {
                KeyCode::Char('y') | KeyCode::Char('Y') => self.reset_game(),
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => self.exit(),
                _ => {}
            }
            return;
        }
        match key_event.code {
            KeyCode::Esc => self.exit(),
            KeyCode::Char(c) if c.is_ascii_alphabetic() && self.current_col < 5 => {
                self.grid.populate_cell(
                    self.current_row,
                    self.current_col,
                    Some(c.to_ascii_lowercase()),
                    None,
                );
                self.current_col += 1;
            }
            KeyCode::Backspace if self.current_col > 0 => {
                self.current_col -= 1;
                self.grid
                    .populate_cell(self.current_row, self.current_col, None, None);
            }
            KeyCode::Enter if self.current_col == 5 => {
                let user_word: String = self.grid.cells
                    [self.current_row * 5..(self.current_row + 1) * 5]
                    .iter()
                    .filter_map(|c| c.value)
                    .collect();
                if self.allowed_words.contains(&user_word) {
                    let mut contains_chars = [false; 5];
                    for (i, c) in user_word.chars().enumerate() {
                        let mut matched = false;
                        if !contains_chars[i] && c == self.answer_word[i]{
                            self.grid.populate_cell(self.current_row, i, Some(c), Some(crate::grid::Color::Green));
                            contains_chars[i] = true;
                            matched = true;
                        } else {
                            for j in 0..5 {
                                if !contains_chars[j] && c == self.answer_word[j] && i != j{
                                    self.grid.populate_cell(self.current_row, i, Some(c), Some(crate::grid::Color::Yellow));
                                    contains_chars[j] = true;
                                    matched = true;
                                    break;
                                }
                            }
                        }
                        if !matched {
                            self.grid.populate_cell(self.current_row, i, Some(c), Some(crate::grid::Color::Gray));
                        }
                    }
                    let is_won = self.grid.cells[self.current_row * self.grid.cols..(self.current_row + 1) * self.grid.cols]
                        .iter()
                        .all(|c| c.color == Some(crate::grid::Color::Green));
                    if is_won {
                        self.game_state = GameState::Won;                    }
                    self.current_row += 1;
                    self.current_col = 0;

                    if self.current_row == self.grid.rows && !is_won {
                        self.game_state = GameState::Lost;
                    }
                }
            }
            _ => {}
        }
    }
    fn exit(&mut self) {
        self.exit = true;
    }

    fn reset_game(&mut self) {
        let answer_word = self.answer_word_picker.get_random_word().chars().collect::<Vec<char>>();
        self.answer_word = answer_word.try_into().unwrap();
        self.game_state = GameState::InProgress;
        self.grid = Grid::new();
        self.current_row = 0;
        self.current_col = 0;
        
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Wordle ").bold();
        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::DOUBLE);
        let inner_area = block.inner(area);
        block.render(inner_area, buf);

        let cell_width: u16 = 5;
        let cell_height: u16 = 3;
        let gap: u16 = 1;
        let grid_width = cell_width * 5 + gap * 4;
        let grid_height = cell_height * 6 + gap * 5;

        let min_width = grid_width + 4; // 4 for outer border + padding
        let min_height = grid_height + 4;

        if area.width < min_width || area.height < min_height {
            Paragraph::new("Terminal too small!")
                .centered()
                .render(area, buf);
            return;
        }
        let start_x = inner_area.x + inner_area.width.saturating_sub(grid_width) / 2;
        let start_y = inner_area.y + inner_area.height.saturating_sub(grid_height) / 2;

        for row in 0..self.grid.rows {
            for col in 0..self.grid.cols {
                let cell = self.grid.get_cell(row, col).unwrap();

                let cell_tile = Rect::new(
                    start_x + col as u16 * (cell_width + gap),
                    start_y + row as u16 * (cell_height + gap),
                    cell_width,
                    cell_height,
                );

                let bg = match &cell.color {
                    Some(crate::grid::Color::Gray) => Color::DarkGray,
                    Some(crate::grid::Color::Yellow) => Color::Yellow,
                    Some(crate::grid::Color::Green) => Color::Green,
                    None => Color::Reset,
                };

                let top_pad = (cell_height - 3) / 2;
                let letter = cell.value.map(|c| c.to_ascii_uppercase().to_string()).unwrap_or_default();

                Paragraph::new(letter)
                    .block(
                        match &cell.color {
                            Some(crate::grid::Color::Gray) => Block::new().padding(Padding::new(0, 0, top_pad + 1, 0)),
                            Some(crate::grid::Color::Yellow) => Block::new().padding(Padding::new(0, 0, top_pad + 1, 0)),
                            Some(crate::grid::Color::Green) => Block::new().padding(Padding::new(0, 0, top_pad + 1, 0)),
                            None => Block::bordered().border_set(border::THICK).padding(Padding::new(0, 0, top_pad, 0)),
                        }
                    )
                    .style(Style::default().bg(bg))
                    .centered()
                    .bold()
                    .render(cell_tile, buf);
            }
        }
    }
}
