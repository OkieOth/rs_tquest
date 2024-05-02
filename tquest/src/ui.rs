use std::{
    io::{self, Stdout},
    time::Duration,
};

use anyhow::{Context, Result};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, 
     widgets::{Block, Borders, LineGauge, Padding, Paragraph, Wrap, Clear}};
use ratatui::text::Text;
use crate::questionaire::{QuestionAnswer, Questionaire};

const ARROW_LEFT: &str = "←";
const ARROW_RIGHT: &str = "→";


#[derive(Default)]
pub enum UiState {
    Help,
    #[default]
    Question,
    Scrolling,
}

#[derive(Default)]
pub enum InputMode {
    Normal,
    #[default]
    Editing,
}

#[derive(Default)]
pub struct Ui<'a> {
    pub state: UiState,
    pub progress: f32,

    pub questionaire: Option<&'a Questionaire>,
    pub show_popup: bool,
    pub input_mode: InputMode,
    pub cursor_position_by_char: usize,
    pub input: String,
    pub max_input_diplay_len: usize,
    pub input_display_start: usize,
}

impl<'a> Ui<'a>  {
    pub fn new(questionaire: &'a Questionaire) -> Self {
        Self {
            state: UiState::Question,
            progress: 0.0,
            questionaire: Some(questionaire),
            show_popup: false,
            input_mode: InputMode::Editing,
            cursor_position_by_char: 0,
            input: "".to_string(),
            max_input_diplay_len: 0,
            input_display_start: 0,
        }   
    }

    pub fn run(&mut self) -> Result<Option<Vec<QuestionAnswer>>> {
        let mut terminal = self.setup_terminal().context("setup failed")?;
        self.process_questionaire(&mut terminal).context("app loop failed")?;
        self.restore_terminal(&mut terminal).context("restore terminal failed")?;
        Ok(None)
    }

    fn process_questionaire(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        loop {
            terminal.draw(|f| render_app(f, self))?;
            if let Event::Key(key) = event::read()? {
                match self.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('e') => {
                            self.input_mode = InputMode::Editing;
                        }
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        _ => {}
                    },
                    InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Enter => self.submit_message(),
                        KeyCode::Char(to_insert) => {
                            self.enter_char(to_insert);
                        }
                        KeyCode::Backspace => {
                            self.delete_char_before();
                        }
                        KeyCode::Delete => {
                            self.delete_char_under();
                        }
                        KeyCode::Left => {
                            self.move_cursor_left();
                        }
                        KeyCode::Right => {
                            self.move_cursor_right();
                        }
                        KeyCode::Esc => {
                            self.input_mode = InputMode::Normal;
                        }
                        _ => {}
                    },
                    InputMode::Editing => {}
                }
            }
        }
    }

    fn setup_terminal(&mut self) -> Result<Terminal<CrosstermBackend<Stdout>>> {
        let mut stdout = io::stdout();
        enable_raw_mode().context("failed to enable raw mode")?;
        execute!(stdout, EnterAlternateScreen).context("unable to enter alternate screen")?;
        Terminal::new(CrosstermBackend::new(stdout)).context("creating terminal failed")
    }

    fn restore_terminal(&mut self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
        disable_raw_mode().context("failed to disable raw mode")?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)
            .context("unable to switch to main screen")?;
        terminal.show_cursor().context("unable to show cursor")
    }

    fn get_question_text<'b> (&mut self) -> &str {
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
        Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. \
        Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi \
        ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit \
        in voluptate velit esse cillum dolore eu fugiat nulla pariatur. \
        Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia \
        deserunt mollit anim id est laborum."
    }

    fn move_cursor_left(&mut self) {
        if (self.cursor_position_by_char == 0) && (self.input_display_start > 0) {
            //if (self.input.chars().count() - self.input_display_start) > self.max_input_diplay_len  {
                self.input_display_start = self.input_display_start.saturating_sub(1);
            //}
        } else {
            if (self.input.chars().count() - self.input_display_start) < self.max_input_diplay_len  {
                self.input_display_start = self.input_display_start.saturating_sub(1);
                //self.input_display_start = 0
            }
            let cursor_moved_left = self.cursor_position_by_char.saturating_sub(1);
            self.cursor_position_by_char = self.clamp_cursor(cursor_moved_left);
        }
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position_by_char.saturating_add(1);
        if cursor_moved_right < self.max_input_diplay_len {
            if (self.input.chars().count() - self.input_display_start) >= cursor_moved_right {
                self.cursor_position_by_char = self.clamp_cursor(cursor_moved_right);
            }
        } else {
            if (self.input.chars().count() - self.input_display_start) >= self.max_input_diplay_len {
                self.input_display_start = self.input_display_start.saturating_add(1);
            }
        }
    }

    fn byte_index(&mut self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.cursor_position_by_char + self.input_display_start)
            .unwrap_or(self.input.len())
    }


    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    fn delete_char_before(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position_by_char != 0;
        if is_not_cursor_leftmost {

            let current_index = self.input_display_start + self.cursor_position_by_char;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);

            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn delete_char_under(&mut self) {
        let is_not_cursor_rightmost = self.cursor_position_by_char < (self.input.chars().count() - self.input_display_start);
        if is_not_cursor_rightmost {

            let current_index = self.input_display_start + self.cursor_position_by_char;
            let from_left_to_current_index = current_index;

            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index + 1);

            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
        }
    }


    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.max_input_diplay_len)
    }

    fn reset_cursor(&mut self) {
        self.cursor_position_by_char = 0;
    }

    fn submit_message(&mut self) {
        self.input.clear();
        self.reset_cursor();
    }

    fn get_scroll_info(&mut self) -> String {
        let c = self.input.chars().count();
        let s = " ".repeat(self.max_input_diplay_len - 2);
        let left = if self.input_display_start > 0 {
            ARROW_LEFT
        } else {
            " "
        };
        let right = if (c - self.input_display_start) > self.max_input_diplay_len - 1 {
            ARROW_RIGHT
        } else {
            " "
        };
        let ret = format!("{}{}{}", left, s, right);

        // // debug - start
        // let max_input_disply_len = self.max_input_diplay_len.to_string();
        // let debug_str = " ".repeat(max_input_disply_len.len());
        // let ret = ret.replacen(&debug_str, &max_input_disply_len, 1);
        // // debug - end
        ret
    }

    fn get_input_to_display(&mut self) -> String {
        let c = self.input.chars().count();
        if c - self.input_display_start > self.max_input_diplay_len {
            return self.input.chars().skip(self.input_display_start).take(self.max_input_diplay_len - 1).collect();
        } else {
            return self.input.chars().skip(self.input_display_start).take(c - self.input_display_start).collect();
        }
    }
    
}



fn render_app(frame: &mut Frame, ui: &mut Ui) {
    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Length(4),
            Constraint::Length(3),
        ],
    )
    .margin(0)
    .split(frame.size());
    ui.max_input_diplay_len = (frame.size().width - 6) as usize;

    frame.render_widget(
        Block::new().borders(Borders::TOP).title(" I am a header "),
        main_layout[0],
    );

    let question_txt = Paragraph::new(ui.get_question_text()).wrap(Wrap{trim: true});


    let block = Block::new()
        .borders(Borders::ALL)
        .padding(Padding::uniform(1))
        .border_style(Style::new().gray().bold().italic())
        .title(" Question: ");

    let inner = main_layout[1].inner(&Margin {
        vertical: 0,
        horizontal: 1,
    });

    frame.render_widget(question_txt.block(block), inner);

    let navigation = Paragraph::new("(ENTER - to take answer, press 'q' to quit) ")
    .right_aligned();
    frame.render_widget(navigation, main_layout[2]);


    let inner_answer = main_layout[3].inner(&Margin {
        vertical: 0,
        horizontal: 1,
    });


    let mut text= Text::from(ui.get_input_to_display());
    text.push_line(ui.get_scroll_info());
    let input = Paragraph::new(text)
        .style(match ui.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default()
            .border_style(
                Style::new()
                    .bold()
                    .italic())
            .borders(Borders::ALL).title("Answer: ")
            .padding(Padding::horizontal(1)));
    frame.render_widget(input, inner_answer);

    match ui.input_mode {
        InputMode::Normal =>
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            {}

        InputMode::Editing => {
            #[allow(clippy::cast_possible_truncation)]
            frame.set_cursor(
                inner_answer.x + ui.cursor_position_by_char as u16 + 2,
                inner_answer.y + 1,
            );
        }
    }

    if ui.show_popup {
        let help_txt = Paragraph::new("This is a quite long help text. I wonder how this will be rendered and if all parts of the text will be visible. :-/");

        let block = Block::default().title("Help").borders(Borders::ALL);
        let popup_area = centered_rect(60, 20, frame.size());
        frame.render_widget(Clear, popup_area); //this clears out the background

        frame.render_widget(help_txt.block(block), popup_area);
    }

    let line_gauge = LineGauge::default()
        .block(Block::new()
            .borders(Borders::ALL)
            .title(" Progress "))
        .gauge_style(
        Style::default()
            .fg(Color::Yellow)
            .bg(Color::Black)
            .add_modifier(Modifier::BOLD),
    )
    .line_set(symbols::line::THICK)
    .ratio(0.8);
    let inner_gauge = main_layout[4].inner(&Margin {
        vertical: 0,
        horizontal: 1,
    });

    frame.render_widget(line_gauge, inner_gauge);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer_scrolling_01() {
        let test_input = "1234567890".to_string();
        let mut ui = super::Ui::default();
        ui.max_input_diplay_len = 20;
        ui.input = test_input.clone();
        assert_eq!(test_input.clone(),ui.get_input_to_display());
        assert_eq!(ui.input_display_start,0);
        assert_eq!(ui.cursor_position_by_char,0);
        assert_eq!(" ".repeat(20),ui.get_scroll_info());

        ui.move_cursor_left();
        assert_eq!(test_input.clone(),ui.get_input_to_display());
        assert_eq!(ui.input_display_start,0);
        assert_eq!(ui.cursor_position_by_char,0);
        assert_eq!(" ".repeat(20),ui.get_scroll_info());

        ui.move_cursor_right();
        assert_eq!(test_input.clone(),ui.get_input_to_display());
        assert_eq!(ui.input_display_start,0);
        assert_eq!(ui.cursor_position_by_char,1);
        assert_eq!(" ".repeat(20),ui.get_scroll_info());

        ui.move_cursor_left();
        assert_eq!(test_input.clone(),ui.get_input_to_display());
        assert_eq!(ui.input_display_start,0);
        assert_eq!(ui.cursor_position_by_char,0);
        assert_eq!(" ".repeat(20),ui.get_scroll_info());

        ui.move_cursor_right();
        assert_eq!(test_input.clone(),ui.get_input_to_display());
        assert_eq!(ui.input_display_start,0);
        assert_eq!(ui.cursor_position_by_char,1);
        assert_eq!(" ".repeat(20),ui.get_scroll_info());

        (0..7).for_each(|_| {
            ui.move_cursor_right();
        });
        assert_eq!(test_input.clone(),ui.get_input_to_display());
        assert_eq!(ui.input_display_start,0);
        assert_eq!(ui.cursor_position_by_char,8);
        assert_eq!(" ".repeat(20),ui.get_scroll_info());

        ui.move_cursor_right();
        ui.move_cursor_right();
        assert_eq!(test_input.clone(),ui.get_input_to_display());
        assert_eq!(ui.input_display_start,0);
        assert_eq!(ui.cursor_position_by_char,10);
        assert_eq!(" ".repeat(20),ui.get_scroll_info());

        ui.move_cursor_right();
        ui.move_cursor_right();
        assert_eq!(test_input.clone(),ui.get_input_to_display());
        assert_eq!(ui.input_display_start,0);
        assert_eq!(ui.cursor_position_by_char,10);
        assert_eq!(" ".repeat(20),ui.get_scroll_info());

        ui.move_cursor_left();
        ui.move_cursor_left();
        assert_eq!(test_input.clone(),ui.get_input_to_display());
        assert_eq!(ui.input_display_start,0);
        assert_eq!(ui.cursor_position_by_char,8);
        assert_eq!(" ".repeat(20),ui.get_scroll_info());

        (0..8).for_each(|_| {
            ui.move_cursor_left();
        });
        assert_eq!(test_input.clone(),ui.get_input_to_display());
        assert_eq!(ui.input_display_start,0);
        assert_eq!(ui.cursor_position_by_char,0);
        assert_eq!(" ".repeat(20),ui.get_scroll_info());

        ui.move_cursor_left();
        ui.move_cursor_left();
        ui.move_cursor_left();
        assert_eq!(test_input.clone(),ui.get_input_to_display());
        assert_eq!(ui.input_display_start,0);
        assert_eq!(ui.cursor_position_by_char,0);
        assert_eq!(" ".repeat(20),ui.get_scroll_info());

    }

    #[test]
    fn test_answer_scrolling_02() {
        let test_input = "123456789012345678901234567890".to_string();
        let mut ui = super::Ui::default();
        ui.max_input_diplay_len = 20;
        ui.input = test_input.clone();
        assert_eq!("1234567890123456789".to_string(),ui.get_input_to_display());
        assert_eq!(ui.input_display_start,0);
        assert_eq!(ui.cursor_position_by_char,0);
        let scroll_info = format!("{}{}"," ".repeat(19), ARROW_RIGHT);
        assert_eq!(scroll_info.clone(), ui.get_scroll_info());

        ui.move_cursor_left();
        assert_eq!("1234567890123456789".to_string(),ui.get_input_to_display());
        assert_eq!(ui.input_display_start,0);
        assert_eq!(ui.cursor_position_by_char,0);
        assert_eq!(scroll_info.clone(), ui.get_scroll_info());

        ui.move_cursor_right();
        assert_eq!("1234567890123456789".to_string(),ui.get_input_to_display());
        assert_eq!(ui.input_display_start,0);
        assert_eq!(ui.cursor_position_by_char,1);
        assert_eq!(scroll_info.clone(), ui.get_scroll_info());

        ui.move_cursor_left();
        assert_eq!("1234567890123456789".to_string(),ui.get_input_to_display());
        assert_eq!(ui.input_display_start,0);
        assert_eq!(ui.cursor_position_by_char,0);
        assert_eq!(scroll_info.clone(), ui.get_scroll_info());

        ui.move_cursor_right();
        assert_eq!("1234567890123456789".to_string(),ui.get_input_to_display());
        assert_eq!(ui.input_display_start,0);
        assert_eq!(ui.cursor_position_by_char,1);
        assert_eq!(scroll_info.clone(), ui.get_scroll_info());

        (0..18).for_each(|_| {
            ui.move_cursor_right();
        });
        assert_eq!("1234567890123456789".to_string(),ui.get_input_to_display());
        assert_eq!(ui.input_display_start,0);
        assert_eq!(ui.cursor_position_by_char,19);
        assert_eq!(scroll_info.clone(), ui.get_scroll_info());

        ui.move_cursor_right();
        assert_eq!("2345678901234567890".to_string(),ui.get_input_to_display());
        assert_eq!(ui.input_display_start,1);
        assert_eq!(ui.cursor_position_by_char,19);
        let scroll_info2 = format!("{}{}{}", ARROW_LEFT, " ".repeat(18), ARROW_RIGHT);
        assert_eq!(scroll_info2.clone(), ui.get_scroll_info());

        ui.move_cursor_left();
        assert_eq!("2345678901234567890".to_string(),ui.get_input_to_display());
        assert_eq!(ui.input_display_start,1);
        assert_eq!(ui.cursor_position_by_char,18);
        assert_eq!(scroll_info2.clone(), ui.get_scroll_info());

        ui.move_cursor_left();
        assert_eq!("2345678901234567890".to_string(),ui.get_input_to_display());
        assert_eq!(ui.input_display_start,1);
        assert_eq!(ui.cursor_position_by_char,17);
        assert_eq!(scroll_info2.clone(), ui.get_scroll_info());

        (0..10).for_each(|_| {
            ui.move_cursor_left();
        });

        assert_eq!("2345678901234567890".to_string(),ui.get_input_to_display());
        assert_eq!(ui.input_display_start,1);
        assert_eq!(ui.cursor_position_by_char,7);
        assert_eq!(scroll_info2.clone(), ui.get_scroll_info());

        ui.enter_char('ß');
        assert_eq!("2345678ß90123456789".to_string(),ui.get_input_to_display());
        assert_eq!(ui.input_display_start,1);
        assert_eq!(ui.cursor_position_by_char,8);
        assert_eq!(scroll_info2.clone(), ui.get_scroll_info());

        (0..3).for_each(|_| {
            ui.move_cursor_right();
        });
        ui.delete_char_before();
        assert_eq!("2345678ß91234567890".to_string(),ui.get_input_to_display());
        assert_eq!(ui.input_display_start,1);
        assert_eq!(ui.cursor_position_by_char,10);
        assert_eq!(scroll_info2.clone(), ui.get_scroll_info());

    }
}
