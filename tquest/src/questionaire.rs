use std::str::FromStr;

use builder_m4cro::{Builder, BuilderFromDefault};
use anyhow::{Result, anyhow};
use regex::{self, Regex};

#[derive(Debug, Builder, Clone)]
pub struct StringEntry {
    pub default_value: Option<String>,
    pub reqexp: Option<String>,
    pub max_length: Option<usize>,
    pub min_length: Option<usize>,
}

impl StringEntry {
    pub fn validate<'a>(&self, input: &'a str) -> Result<QuestionAnswerInput> {
        if let Some(min) = self.min_length {
            if input.len() < min {
                return Err(anyhow!("Min input len not respected"));
            }
        }
        if let Some(max) = self.max_length {
            if input.len() > max {
                return Err(anyhow!("Max input len not respected"));
            }
        }
        if let Some(regex) = self.reqexp.as_ref() {
            let re = Regex::from_str(regex).unwrap();
            if ! re.is_match(input) {
                return Err(anyhow!("Max input len not respected"));
            }
        }
        Ok(QuestionAnswerInput::String(input.to_string()))
    }
}


#[derive(Debug, Builder, Clone)]
pub struct IntEntry {
    pub default_value: Option<i32>,
    pub max: Option<i32>,
    pub min: Option<i32>,
}

impl IntEntry {
    pub fn validate<'a>(&self, input: &'a str) -> Result<QuestionAnswerInput> {
        todo!()
    }
}

#[derive(Debug, Builder, Clone)]
pub struct FloatEntry {
    pub default_value: Option<i32>,
    pub max: Option<f32>,
    pub min: Option<f32>,
}

impl FloatEntry {
    pub fn validate<'a>(&self, input: &'a str) -> Result<QuestionAnswerInput> {
        todo!()
    }
}

#[derive(Debug, Builder, Clone)]
pub struct BoolEntry {
    pub default_value: Option<bool>,
}

impl BoolEntry {
    pub fn validate<'a>(&self, input: &'a str) -> Result<QuestionAnswerInput> {
        todo!()
    }
}

#[derive(Debug, Builder, Clone)]
pub struct OptionEntry {
    pub default_value: Option<u32>,
    pub options: Vec<String>,
}

impl OptionEntry {
    pub fn validate<'a>(&self, input: &'a str) -> Result<QuestionAnswerInput> {
        todo!()
    }
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



#[derive(Debug, Clone, Default, BuilderFromDefault)]
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
    pub title: String,
    pub init_block: SubBlock,
}

impl Questionaire {
    pub fn builder<'a> () -> QuestionaireBuilder<'a> {
        QuestionaireBuilder::default()
    }
}


#[derive(Debug, Default)]
pub struct QuestionaireBuilder<'a> {
    title: &'a str,
    id: &'a str,
    start_text: &'a str,
    end_text: Option<&'a str>,
    help_text: Option<&'a str>,
    questions: Option<Vec<QuestionaireEntry>>,
}

impl <'a> QuestionaireBuilder<'a> {
    pub fn id(&mut self, id: &'a str) -> &mut Self {
        self.id = id;
        self
    }

    pub fn title(&mut self, t: &'a str) -> &mut Self {
        self.title = t;
        self
    }

    pub fn start_text(&mut self, t: &'a str) -> &mut Self {
        self.start_text = t;
        self
    }

    pub fn end_text(&mut self, t: &'a str) -> &mut Self {
        self.end_text = Some(t);
        self
    }

    pub fn help_text(&mut self, t: &'a str) -> &mut Self {
        self.help_text = Some(t);
        self
    }

    pub fn questions(&mut self, q: Vec<QuestionaireEntry>) -> &mut Self {
        self.questions = Some(q);
        self
    }

    pub fn build(&self) -> Questionaire {
        let mut init_block = SubBlock::default();
        init_block.id = self.id.to_string();
        init_block.start_text = self.start_text.to_string();
        if self.end_text.is_some() {
            init_block.end_text = Some(self.end_text.unwrap().to_string());
        }
        if self.help_text.is_some() {
            init_block.help_text = Some(self.help_text.unwrap().to_string());
        }

        if self.questions.is_some() {
            init_block.entries = self.questions.as_ref().unwrap().clone();
        }
        Questionaire {
            title: self.title.to_string(),
            init_block,
        }
    }
}

#[derive(Debug)]
pub enum AnswerEntry {
    Block(BlockAnswer),
    Question(QuestionAnswerInput),
}


#[derive(Debug, PartialEq)]
pub enum QuestionAnswerInput {
    String(String),
    Int(i32),
    Float(f32),
    Bool(bool),
    Option(u32),
}

#[derive(Debug)]
pub struct BlockAnswer {
    pub id: String,

    /// Vector of itereations, with the answers of each iteration in its own vector
    pub iterations: Vec<Vec<AnswerEntry>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_validate_min_length() {
        let entry = StringEntry {
            default_value: None,
            min_length: Some(5),
            max_length: None,
            reqexp: None,
        };
        assert!(entry.validate("12345").is_ok());
        assert!(entry.validate("1234").is_err());
    }

    #[test]
    fn test_string_validate_max_length() {
        let entry = StringEntry {
            default_value: None,
            min_length: None,
            max_length: Some(5),
            reqexp: None,
        };
        assert!(entry.validate("12345").is_ok());
        assert!(entry.validate("123456").is_err());
    }

    #[test]
    fn test_string_validate_regex() {
        let entry = StringEntry {
            default_value: None,
            min_length: None,
            max_length: None,
            reqexp: Some(r"^\d+$".to_string()),
        };
        assert!(entry.validate("12345").is_ok());
        assert!(entry.validate("1234a").is_err());
    }

    #[test]
    fn test_string_validate_all_constraints_met() {
        let entry = StringEntry {
            default_value: None,
            min_length: Some(3),
            max_length: Some(5),
            reqexp: Some(r"^\d+$".to_string()),
        };
        assert!(entry.validate("123").is_ok());
        assert!(entry.validate("1234").is_ok());
        assert!(entry.validate("12345").is_ok());
        assert!(entry.validate("12").is_err());
        assert!(entry.validate("123456").is_err());
        assert!(entry.validate("12a34").is_err());
    }

    #[test]
    fn test_string_validate_no_constraints() {
        let entry = StringEntry {
            default_value: None,
            min_length: None,
            max_length: None,
            reqexp: None,
        };
        assert!(entry.validate("any string").is_ok());
    }

    #[test]
    fn test_string_validate_combined_constraints() {
        let entry = StringEntry {
            default_value: None,
            min_length: Some(3),
            max_length: Some(5),
            reqexp: Some(r"^\d+$".to_string()),
        };

        // Valid input that meets all constraints
        assert_eq!(entry.validate("123").unwrap(), QuestionAnswerInput::String("123".to_string()));
        assert_eq!(entry.validate("1234").unwrap(), QuestionAnswerInput::String("1234".to_string()));
        assert_eq!(entry.validate("12345").unwrap(), QuestionAnswerInput::String("12345".to_string()));

        // Invalid inputs
        assert!(entry.validate("12").is_err()); // Too short
        assert!(entry.validate("123456").is_err()); // Too long
        assert!(entry.validate("12a34").is_err()); // Invalid format (non-digit character)
    }
}