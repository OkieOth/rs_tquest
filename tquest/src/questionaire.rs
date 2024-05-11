use std::collections::HashMap;
use std::path::Iter;
use std::rc::Rc;
use std::cell::RefCell;

use builder_m4cro::Builder;
use anyhow::Result;

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



#[derive(Debug, Clone, Default, Builder)]
pub struct SubBlock {
    pub id: String,
    pub start_text: String,
    pub end_text: Option<String>,
    pub help_text: Option<String>,
    pub entries: Vec<QuestionaireEntry>,
    pub loop_over_entries: bool,
}

#[derive(Builder, Debug, Clone)]
pub struct QuestionEntry {
    pub id: String,
    pub query_text: String,
    pub help_text: Option<String>,
    pub entry_type: EntryType,
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
}


#[derive(Debug, Default)]
pub struct QuestionaireBuilder {
}

impl QuestionaireBuilder {
    pub fn add_init_block_and_build(&mut self,
        id: &str, 
        start_text: &str, 
        end_text: Option<&str>,
        help_text: Option<&str>,
        questions: Option<Vec<QuestionaireEntry>>) -> Questionaire {
        let mut init_block = SubBlock::default();
        init_block.id = id.to_string();
        init_block.start_text = start_text.to_string();
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
pub enum AnswerEntry {
    Block(BlockAnswer),
    Question(QuestionAnswerInput),
}


#[derive(Debug)]
pub enum QuestionAnswerInput {
    String(String),
    Int(i32),
    Float(f32),
    Bool(bool),
    Option(String),
}

#[derive(Debug)]
pub struct BlockAnswer {
    pub id: String,
    pub iterations: Vec<Vec<AnswerEntry>>,
}
