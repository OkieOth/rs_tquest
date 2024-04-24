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
    state: UiState,
    progress: f32,

    questionaire: &'a Questionaire,
}

impl<'a> Ui<'a>  {
    pub fn new(questionaire: &'a Questionaire) -> Self {
        Self {
            state: UiState::Question,
            progress: 0.0,
            questionaire: questionaire,
        }
    }

    pub fn run(&mut self) -> Option<Vec<QuestionAnswer>> {
        None
    }
}