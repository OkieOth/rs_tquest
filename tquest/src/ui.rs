
use anyhow::Result;
use colored::Colorize;


use crate::questionaire::{QuestionAnswerInput, QuestionEntry};


pub enum QuestionScreenResult {
    Canceled,
    Proceeded(QuestionAnswerInput)
}

pub enum ProceedScreenResult {
    Canceled,
    Proceeded(bool)
}


pub trait QuestionaireView {
    fn print_title<'a>(&mut self, title: &str);
    fn show_proceed_screen<'a, T: Into<Option<&'a str>>>(&mut self, id: &str, text: &str, help_text: T) -> Result<ProceedScreenResult>;
    fn show_question_screen(&mut self, question_entry: &QuestionEntry) -> Result<QuestionScreenResult>;
}


pub struct Ui {
}


impl Ui  {
    pub fn new() -> Result<Self> {
        Ok(Self {
        }) 
    }
}


impl QuestionaireView for Ui {
    fn print_title<'a>(&mut self, title: &str) {
        println!("\n{}\n", title.bold());
    }

    fn show_proceed_screen<'a, T: Into<Option<&'a str>>>(&mut self, _id: &str, text: &str, help_text: T) -> Result<ProceedScreenResult> {
        let ht = help_text.into();
        println!("{}\n", text.italic());
        Ok(ProceedScreenResult::Canceled)
    }

    fn show_question_screen(&mut self, _question_entry: &QuestionEntry) -> Result<QuestionScreenResult>{
        Ok(QuestionScreenResult::Canceled)
    }
}

