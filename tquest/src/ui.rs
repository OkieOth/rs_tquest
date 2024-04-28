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


pub enum UiState {
    Help,
    Question,
    Scrolling,
}

enum InputMode {
    Normal,
    Editing,
}

pub struct Ui<'a> {
    pub state: UiState,
    pub progress: f32,

    pub questionaire: &'a Questionaire,
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
            questionaire: questionaire,
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
                            self.delete_char();
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
            if (self.input.chars().count() - self.input_display_start) > self.max_input_diplay_len  {
                self.input_display_start = self.input_display_start.saturating_sub(1);
            }
        } else {
            if (self.input.chars().count() - self.input_display_start) < self.max_input_diplay_len  {
                self.input_display_start = 0
            }
            let cursor_moved_left = self.cursor_position_by_char.saturating_sub(1);
            self.cursor_position_by_char = self.clamp_cursor(cursor_moved_left);
        }
    }

    fn move_cursor_right(&mut self) {
        let c = self.input.chars().count();
        let cursor_moved_right = self.cursor_position_by_char.saturating_add(1);
        if (cursor_moved_right < self.max_input_diplay_len) {
            if (self.input.chars().count() - self.input_display_start) >= cursor_moved_right {
                self.cursor_position_by_char = self.clamp_cursor(cursor_moved_right);
            }
        } else {
            if (self.input.chars().count() - self.input_display_start) >= self.max_input_diplay_len {
                self.input_display_start = self.input_display_start.saturating_add(1);
            }
        }
        // if (cursor_moved_right < self.max_input_diplay_len) && ((cursor_moved_right + self.input_display_start) <= c +1) {
        //     self.cursor_position_by_char = self.clamp_cursor(cursor_moved_right);
        // } else {
        //     if (self.input.chars().count() - self.input_display_start) >= self.max_input_diplay_len {
        //         self.input_display_start = self.input_display_start.saturating_add(1);
        //     }
        // }
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

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position_by_char != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.cursor_position_by_char;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
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

    fn get_input_to_display(&mut self) -> String {
        // TODO optimize the input clone
        let c = self.input.chars().count();
        if self.cursor_position_by_char == 0 {
            if c > self.max_input_diplay_len {
                let ret: String = self.input.chars().take(self.max_input_diplay_len - 1).collect();
                return format!("{}{}", ret, ARROW_RIGHT);
            } else {
                return self.input.clone();
            }
        } else {
            if self.cursor_position_by_char >= self.max_input_diplay_len -1 {
                if c >= self.max_input_diplay_len -1 {
                    let ret: String = self.input.chars().skip(self.input_display_start).take(self.max_input_diplay_len - 2).collect();
                    return format!("{}{}", ARROW_LEFT, ret);
                    // 1234567890
                } else {
                    return self.input.clone();
                }    
            } else {
                if c > self.max_input_diplay_len {
                    if c - self.input_display_start > self.max_input_diplay_len {
                        if self.input_display_start > 0 {
                            let ret: String = self.input.chars().skip(self.input_display_start).take(self.max_input_diplay_len - 2).collect();
                            return format!("{}{}{}", ARROW_LEFT, ret, ARROW_RIGHT);
                        } else {
                            let ret: String = self.input.chars().take(self.max_input_diplay_len - 1).collect();
                            return format!("{}{}", ret, ARROW_RIGHT);
                        }
                    } else {
                        // if self.input_display_start > 0 {
                        //     let ret: String = self.input.chars().skip(self.input_display_start).take(self.max_input_diplay_len - 2).collect();
                        //     return format!("{}{}{}", ARROW_LEFT, ret, ARROW_RIGHT);
                        // } else {
                        //     let ret: String = self.input.chars().take(self.max_input_diplay_len - 1).collect();
                        //     return format!("{}{}", ret, ARROW_RIGHT);
                        // }

                        return "TODO-3".to_string();
                    }
                } else {
                    return self.input.clone();
                }    
            }
        }


        // if c < self.max_input_diplay_len {
        //     return self.input.clone();
        // } else {
        //     if self.cursor_position_by_char == 0 {
        //         if  (c - self.input_display_start) > self.max_input_diplay_len {
        //             let ret: String = self.input.chars().take(self.max_input_diplay_len - 1).collect();
        //             return format!("{}{}", ret, ARROW_RIGHT);
        //         } else {
        //             return self.input.clone();
        //         }
        //     } else {
        //         if self.cursor_position_by_char >= self.max_input_diplay_len - 1 {
        //             if  (c - self.input_display_start) <= self.max_input_diplay_len {
        //                 let ret: String = self.input.chars().take(self.max_input_diplay_len - 1).collect();
        //                 return format!("{}{}", ARROW_LEFT, ret);
        //             } else {
        //                 let ret: String = self.input.chars().take(self.max_input_diplay_len - 2).collect();
        //                 return format!("{}{}{}", ARROW_LEFT, ret, ARROW_RIGHT);
        //             }
        //         } else {
        //             if  (c - self.input_display_start) <= self.max_input_diplay_len {
        //                 let ret: String = self.input.chars().take(self.max_input_diplay_len - 1).collect();
        //                 return format!("{}{}", ARROW_LEFT, ret);
        //             } else {
        //                 let ret: String = self.input.chars().take(self.max_input_diplay_len - 2).collect();
        //                 return format!("{}{}{}", ARROW_LEFT, ret, ARROW_RIGHT);
        //             }
        //         }
        //     }
        // }
    }
    
}



fn render_app(frame: &mut Frame, ui: &mut Ui) {


    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Length(3),
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


    let input = Paragraph::new(ui.get_input_to_display())
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