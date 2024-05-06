use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use builder_m4cro::Builder;
use anyhow::{Result};

use crate::Ui;

#[derive(Debug, Builder, Clone)]
pub struct StringEntry {
    pub default_value: Option<String>,
    pub reqexp: Option<String>,
    pub max_length: Option<usize>,
    pub min_length: Option<usize>,
}



#[derive(Debug, Builder, Clone)]
pub struct IntEntry {
    pub default_value: Option<i32>,
    pub max: Option<i32>,
    pub min: Option<i32>,
}

#[derive(Debug, Builder, Clone)]
pub struct FloatEntry {
    pub default_value: Option<i32>,
    pub max: Option<f32>,
    pub min: Option<f32>,
}

#[derive(Debug, Builder, Clone)]
pub struct BoolEntry {
    pub default_value: Option<bool>,
}

#[derive(Debug, Builder, Clone)]
pub struct OptionEntry {
    pub default_value: Option<u32>,
    pub options: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum EntryType {
    String(StringEntry),
    Int(IntEntry),
    Float(FloatEntry),
    Bool(BoolEntry),
    Option(OptionEntry),
    ProceedQuery(u32),
    InfoTxt,
}

#[derive(Debug, Clone)]
pub enum QuestionaireEntry {
    Block(SubBlock),
    Question(QuestionEntry),
}


#[derive(Debug, Clone, Builder)]
pub struct SubBlock {
    pub start_text: String,
    pub end_text: Option<String>,
    pub help_text: Option<String>,
    pub entries: Vec<QuestionaireEntry>,
}

#[derive(Builder, Debug, Clone)]
pub struct QuestionEntry {
    pub query_text: String,
    pub help_text: Option<String>,
    pub entry_type: EntryType,
    pub id: String,
}

pub struct QuestionaireResults {
    pub answers: Vec<QuestionAnswer>,
}

#[derive(Debug)]
pub struct Questionaire {
    /// Hashmap of level to list of questions per level
    pub init_block: SubBlock,
}

impl Questionaire {
    pub fn builder() -> QuestionaireBuilder {
        QuestionaireBuilder::default()
    }
    pub fn run(&mut self) -> Result<Option<Vec<QuestionAnswer>>> {
        let mut ui = Ui::new(Some(self));
        return ui.run();
    }
}

#[derive(Debug, Default)]
pub struct QuestionaireBuilder {
}

impl QuestionaireBuilder {
    pub fn add_init_block_and_build(&mut self, 
        start_text: &str, 
        end_text: Option<&str>,
        help_text: Option<&str>,
        questions: Option<Vec<QuestionaireEntry>>) -> Questionaire {
        let mut init_block = SubBlock::builder()
            .start_text(start_text).build().unwrap();
        if end_text.is_some() {
            init_block.end_text = Some(end_text.unwrap().to_string());
        }
        if help_text.is_some() {
            init_block.help_text = Some(help_text.unwrap().to_string());
        }
        if questions.is_some() {
            init_block.entries = questions.unwrap();
        }
        Questionaire {
            init_block,
        }
    }
}


#[derive(Debug)]
pub struct QuestionAnswer {
    pub id: String,
    pub level: u8,
    pub answer: EntryInput,
}


#[derive(Debug)]
pub enum EntryInput {
    String(String),
    Int(i32),
    Float(f32),
    Bool(bool),
    Option(String),
    ProceedQuery(bool),
}
