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
pub struct QuestionEntry {
    pub query_text: String,
    pub help_text: Option<String>,
    pub entry_type: EntryType,
    pub id: String,
}

pub struct QuestionaireResults {
    pub answers: Vec<QuestionAnswer>,
}

#[derive(Debug, Default)]
pub struct Questionaire {
    /// Hashmap of level to list of questions per level
    pub questions: HashMap<u8, Vec<Rc<RefCell<QuestionEntry>>>>,
}

impl Questionaire {
    pub fn builder() -> QuestionaireBuilder {
        QuestionaireBuilder::default()
    }
    pub fn run(&mut self) -> Result<Option<Vec<QuestionAnswer>>> {
        let mut ui = Ui::new(self);
        return ui.run();
    }
}

#[derive(Debug, Default)]
pub struct QuestionaireBuilder {
    pub questions: HashMap<u8, Vec<QuestionEntry>>,
}

impl QuestionaireBuilder {
    pub fn add_boolean_question(&mut self, 
        level: u8,
        id: &str, 
        query_text: &str,
        help_text: Option<&str>,
        entry_def: Option<BoolEntry>) -> &mut Self {
        self
    }
    
    pub fn add_string_question(&mut self,
        level: u8,
        id: &str, 
        query_text: &str,
        help_text: Option<&str>,
        entry_def: Option<StringEntry>) -> &mut Self {
        self
    }

    pub fn add_int_question(&mut self, 
        level: u8,
        id: &str, 
        query_text: &str,
        help_text: Option<&str>,
        entry_def: Option<IntEntry>) -> &mut Self {
        self
    }

    pub fn add_float_question(&mut self, 
        level: u8,
        id: &str, 
        query_text: &str,
        help_text: Option<&str>,
        entry_def: Option<FloatEntry>) -> &mut Self {
        self
    }

    pub fn add_option_question(&mut self,
        level: u8, 
        id: &str, 
        query_text: &str,
        help_text: Option<&str>,
        entry_def: Option<OptionEntry>) -> &mut Self {
        self
    }

    pub fn add_proceed_question(&mut self, 
        level: u8,
        id: &str, 
        first_query_text: &str, 
        additional_query_text: Option<&str>) -> &mut Self {
        self
    }

    pub fn add_info_text(&mut self, 
        level: u8,
        id: &str,
        text: &str) -> &mut Self {
        // self.questions.push(QuestionEntry {
        //     query_text: text.to_string(),
        //     help_text: None,
        //     entry_type: EntryType::InfoTxt,
        //     id: id.to_string(),
        // });
        self
    }

    pub fn build(&self) -> Questionaire {
        let mut q = Questionaire::default();
        for (key, val) in self.questions.iter() {
            let mut v: Vec<Rc<RefCell<QuestionEntry>>> = Vec::new();
            for e in val.iter() {
                v.push(Rc::new(RefCell::new(e.clone())));
            }
            q.questions.insert(*key, v);
        };
        q
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
