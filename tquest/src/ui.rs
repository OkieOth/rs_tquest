use std::{
    io::{self, Stdout},
    time::Duration,
};

use anyhow::{Context, Result};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, 
     widgets::{Block, Borders, LineGauge, Padding, Paragraph, Widget}};

use crate::questionaire::{QuestionAnswer, Questionaire};



pub enum UiState {
    Help,
    Question,
    Scrolling,
}

pub struct Ui<'a> {
    pub state: UiState,
    pub progress: f32,

    pub questionaire: &'a Questionaire,
}

impl<'a> Ui<'a>  {
    pub fn new(questionaire: &'a Questionaire) -> Self {
        Self {
            state: UiState::Question,
            progress: 0.0,
            questionaire: questionaire,
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
            if self.should_quit()? {
                break;
            }
        }
        Ok(())
    }

    fn should_quit(&mut self, ) -> Result<bool> {
        if event::poll(Duration::from_millis(250)).context("event poll failed")? {
            if let Event::Key(key) = event::read().context("event read failed")? {
                return Ok(KeyCode::Char('q') == key.code);
            }
        }
        Ok(false)
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
    
}

fn render_app(frame: &mut Frame, ui: &mut Ui) {
    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Min(6),
            Constraint::Percentage(80),
            Constraint::Length(1),
            Constraint::Length(3),
        ],
    )
    .margin(0)
    .split(frame.size());

    frame.render_widget(
        Block::new().borders(Borders::TOP).title(" I am a header "),
        main_layout[0],
    );

    let question_txt = Paragraph::new("What's your favourite Linux shell? (press 'h' for help)");

    let block = Block::new()
        .borders(Borders::ALL)
        .padding(Padding::horizontal(1))
        .border_style(Style::new().gray().bold().italic())
        .title(" Question: ");

    let inner = main_layout[1].inner(&Margin {
        vertical: 1,
        horizontal: 1,
    });

    frame.render_widget(question_txt.block(block), inner);



    let inner2 = main_layout[2].inner(&Margin {
        vertical: 1,
        horizontal: 1,
    });

    frame.render_widget(
        Block::new().borders(Borders::NONE).title(" Your Answer: "),
        inner2,
    );


    let navigation = Paragraph::new("(ENTER - to take answer, arrows to move back and forth, press 'q' to quit) ")
    .right_aligned();
    frame.render_widget(navigation, main_layout[3]);

    // if app.show_popup {
    //     let block = Block::default().title("Popup").borders(Borders::ALL);
    //     let area = centered_rect(60, 20, area);
    //     f.render_widget(Clear, area); //this clears out the background
    //     f.render_widget(block, area);
    // }

    let line_gauge = LineGauge::default()
        .block(Block::default().borders(Borders::ALL).title(" Progress "))
        .gauge_style(
        Style::default()
            .fg(Color::Yellow)
            .bg(Color::Black)
            .add_modifier(Modifier::BOLD),
    )
    .line_set(symbols::line::THICK)
    .ratio(0.8);
    frame.render_widget(line_gauge, main_layout[4]);
    // frame.render_widget(
    //     Block::new().borders(Borders::TOP).title("Progress"),
    //     main_layout[2],
    // );
}
