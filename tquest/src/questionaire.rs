//! Main types of the questionaire implementation
//! 
use std::str::FromStr;
use std::fmt::{Display, Formatter};

use builder_m4cro::BuilderFromDefault;
use anyhow::{Result, anyhow};
use regex::{self, Regex};
use serde::{Deserialize, Serialize};

/// Expected string entry
#[derive(Debug, BuilderFromDefault, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct StringEntry {
    pub default_value: Option<String>,
    pub regexp: Option<String>,
    pub max_length: Option<usize>,
    pub min_length: Option<usize>,
}

impl StringEntry {
    pub fn validate<'a>(&self, input: &'a str, required: bool) -> Result<QuestionAnswerInput> {
        if input.len() == 0 {
            if let Some(def_value) = self.default_value.as_ref() {
                return Ok(QuestionAnswerInput::String(Some(def_value.clone())));
            } else {
                if ! required {
                    return Ok(QuestionAnswerInput::String(None));
                } else {

                }
                return Err(anyhow!("No default value is set. Input is needed."));
            }
        }
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
        if let Some(regex) = self.regexp.as_ref() {
            let re = Regex::from_str(regex).unwrap();
            if ! re.is_match(input) {
                return Err(anyhow!("Max input len not respected"));
            }
        }
        Ok(QuestionAnswerInput::String(Some(input.to_string())))
    }
}


/// Expected int entry
#[derive(Debug, BuilderFromDefault, Default, Clone, Deserialize, Serialize, PartialEq)]
pub struct IntEntry {
    pub default_value: Option<i32>,
    pub max: Option<i32>,
    pub min: Option<i32>,
}

impl IntEntry {
    pub fn validate<'a>(&self, input: &'a str, required: bool) -> Result<QuestionAnswerInput> {
        if input.len() == 0 {
            if let Some(def_value) = self.default_value {
                return Ok(QuestionAnswerInput::Int(Some(def_value)));
            } else {
                if ! required {
                    return Ok(QuestionAnswerInput::Int(None));
                } else {
                    return Err(anyhow!("No default value is set. Input is needed."));
                }
            }
        }
        let input_value = if let Ok(i) = input.parse() {
            i
        } else {
            return Err(anyhow!("Input can not be cast into an int value."));
        };
        if let Some(min) = self.min {
            if input_value < min {
                return Err(anyhow!("Input doesn't respect min value constraint."));
            }
        }
        if let Some(max) = self.max {
            if input_value > max {
                return Err(anyhow!("Input doesn't respect max value constraint."));
            }
        }
        Ok(QuestionAnswerInput::Int(Some(input_value)))
    }
}

/// Expected floating point entry
#[derive(Debug, BuilderFromDefault, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct FloatEntry {
    pub default_value: Option<f32>,
    pub max: Option<f32>,
    pub min: Option<f32>,
}

impl FloatEntry {
    pub fn validate<'a>(&self, input: &'a str, required: bool) -> Result<QuestionAnswerInput> {
        if input.len() == 0 {
            if let Some(def_value) = self.default_value {
                return Ok(QuestionAnswerInput::Float(Some(def_value)));
            } else {
                if ! required {
                    return Ok(QuestionAnswerInput::Float(None));
                } else {
                    return Err(anyhow!("No default value is set. Input is needed."));
                }
            }
        }
        let input_value: f32 = if let Ok(i) = input.parse() {
            i
        } else {
            return Err(anyhow!("Input can not be cast into an int value."));
        };
        if let Some(min) = self.min {
            if input_value < min {
                return Err(anyhow!("Input doesn't respect min value constraint."));
            }
        }
        if let Some(max) = self.max {
            if input_value > max {
                return Err(anyhow!("Input doesn't respect max value constraint."));
            }
        }
        Ok(QuestionAnswerInput::Float(Some(input_value)))
    }
}

/// Expected bool entry
#[derive(Debug, BuilderFromDefault, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct BoolEntry {
    pub default_value: Option<bool>,
}

impl BoolEntry {
    pub fn validate<'a>(&self, input: &'a str, required: bool) -> Result<QuestionAnswerInput> {
        if input.len() == 0 {
            if let Some(def_value) = self.default_value {
                return Ok(QuestionAnswerInput::Bool(Some(def_value)));
            } else {
                if ! required {
                    return Ok(QuestionAnswerInput::Bool(None));
                } else {
                    return Err(anyhow!("No default value is set. Input is needed."));
                }
            }
        }
        match input.to_lowercase().as_str() {
            "y" | "yes" | "true" => Ok(QuestionAnswerInput::Bool(Some(true))),
            "n" | "no" | "false" => Ok(QuestionAnswerInput::Bool(Some(false))),
            _ => return Err(anyhow!("Input can not be cast into a bool value.")),
        }
    }
}

/// Expected String entry based on a number of predefined options
#[derive(Debug, BuilderFromDefault, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct OptionEntry {
    /// represents the index of the array, used as default value
    pub default_value: Option<u32>,

    /// valid input options
    pub options: Vec<String>,
}

impl OptionEntry {
    pub fn validate<'a>(&self, input: &'a str, required: bool) -> Result<QuestionAnswerInput> {
        if input.len() == 0 {
            if let Some(def_value) = self.default_value {
                if def_value < self.options.len() as u32 {
                    return Ok(QuestionAnswerInput::Option(Some(self.options.get(def_value as usize).unwrap().clone())));
                } else {
                    if ! required {
                        return Ok(QuestionAnswerInput::Option(None));
                    } else {
                        return Err(anyhow!("Default value index is bigger than the options list"));
                    }
                }
            } else {
                return Err(anyhow!("No default value is set. Input is needed."));
            }
        }
        if let Ok(i) = input.parse::<usize>() {
            if i < self.options.len() {
                return Ok(QuestionAnswerInput::Option(Some(self.options.get(i as usize).unwrap().clone())));
            } else {
                return Err(anyhow!("Default value index is bigger than the options list"));
            }
        } else {
            return Err(anyhow!("Input can't be cast into the option index."));
        };
    }
}


#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum EntryType {
    String(StringEntry),
    Int(IntEntry),
    Float(FloatEntry),
    Bool(BoolEntry),
    Option(OptionEntry),
    ProceedQuery(u32),
    InfoTxt,
}

impl Default for EntryType {
    fn default() -> Self{
        EntryType::String(StringEntry::default())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum QuestionaireEntry {
    Block(SubBlock),
    Question(QuestionEntry),
    RepeatedQuestion(RepeatedQuestionEntry),
}



#[derive(Debug, Clone, Default, BuilderFromDefault, Deserialize, Serialize, PartialEq)]
pub struct SubBlock {
    pub id: String,
    pub pos: Option<usize>,
    pub start_text: String,
    pub end_text: Option<String>,
    pub help_text: Option<String>,
    pub entries: Vec<QuestionaireEntry>,
    pub loop_over_entries: bool,
}

#[derive(BuilderFromDefault, Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct QuestionEntry {
    pub id: String,
    pub pos: usize,
    pub required: bool,
    pub query_text: String,
    pub help_text: Option<String>,
    pub entry_type: EntryType,
}

#[derive(BuilderFromDefault, Debug, Clone, Default, Deserialize, Serialize, PartialEq)]
pub struct RepeatedQuestionEntry {
    pub id: String,
    pub pos: usize,
    pub min_count: usize,
    pub max_count: usize,
    pub query_text: String,
    pub secondary_query_text: Option<String>,
    pub help_text: Option<String>,
    pub entry_type: EntryType,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Questionaire {
    /// Hashmap of level to list of questions per level
    pub title: String,
    pub pos_count: Option<usize>,
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
    question_count: usize,
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

    fn init_positions(&mut self) {
        fn init_positions_block(block: &mut SubBlock, current_counter: usize) -> usize {
            block.pos = Some(current_counter);
            let mut counter = current_counter + 1;
            for q in block.entries.iter_mut() {
                match q {
                    QuestionaireEntry::Question(e) => {
                        e.pos = counter;
                        counter += 1;
                    },
                    QuestionaireEntry::Block(b) => {
                        counter = init_positions_block(b, counter);
                    }
                    QuestionaireEntry::RepeatedQuestion(e) => {
                        e.pos = counter;
                        counter += 1;
                    },
                }
            }
            counter
        }
        
        if let Some(questions) = self.questions.as_mut() {
            let mut counter: usize = 1;
            for q in questions.iter_mut() {
                match q {
                    QuestionaireEntry::Question(e) => {
                        e.pos = counter;
                        counter += 1;
                    },
                    QuestionaireEntry::RepeatedQuestion(e) => {
                        e.pos = counter;
                        counter += 1;
                    },
                    QuestionaireEntry::Block(b) => {
                        counter = init_positions_block(b, counter);
                    }
                }
            }
            self.question_count = counter - 1;
        }
    }

    pub fn questions(&mut self, q: Vec<QuestionaireEntry>) -> &mut Self {
        self.questions = Some(q);
        self.init_positions();
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
            pos_count: Some(self.question_count),
            init_block,
        }
    }
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum AnswerEntry {
    Block(BlockAnswer),
    Question(QuestionAnswerInput),
    RepeatedQuestion(Vec<QuestionAnswerInput>),
}


#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub enum QuestionAnswerInput {
    String(Option<String>),
    Int(Option<i32>),
    Float(Option<f32>),
    Bool(Option<bool>),
    Option(Option<String>)
}

impl Display for QuestionAnswerInput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            QuestionAnswerInput::String(value) => match value {
                Some(val) => write!(f, "{}", val),
                None => write!(f, ""),
            },
            QuestionAnswerInput::Int(value) => match value {
                Some(val) => write!(f, "{}", val),
                None => write!(f, ""),
            },
            QuestionAnswerInput::Float(value) => match value {
                Some(val) => write!(f, "{}", val),
                None => write!(f, ""),
            },
            QuestionAnswerInput::Bool(value) => match value {
                Some(val) => write!(f, "{}", val),
                None => write!(f, ""),
            },
            QuestionAnswerInput::Option(value) => match value {
                Some(val) => write!(f, "Some({})", val),
                None => write!(f, ""),
            },
        }
    }
}
#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct BlockAnswer {
    pub id: String,

    /// Vector of itereations, with the answers of each iteration in its own vector
    pub iterations: Vec<Vec<AnswerEntry>>,
}

#[cfg(test)]
mod tests {
    use crate::questionaire;
    use super::*;
    use crate::test_helper;

    #[test]
    fn test_pos_init_01() {
        let questionaire = test_helper::create_small_questionaire();
        assert_eq!(2, questionaire.pos_count.unwrap());
        assert_eq!(None, questionaire.init_block.pos);
        let mut counter: usize = 1;
        for e in questionaire.init_block.entries {
            match e {
                QuestionaireEntry::Question(q) => {
                    assert_eq!(counter, q.pos);
                    counter += 1;
                },
                QuestionaireEntry::Block(b) => {
                    assert_eq!(counter, b.pos.unwrap());
                    counter += 1;
                },
                _ => panic!("Unexpected input"),
            };
        }
    }

    #[test]
    fn test_pos_init_02() {
        let questionaire = test_helper::create_complex_questionaire();
        assert_eq!(16, questionaire.pos_count.unwrap());
        assert_eq!(None, questionaire.init_block.pos);
        // let mut counter: usize = 1;
        // for e in questionaire.init_block.entries {
        //     match e {
        //         QuestionaireEntry::Question(q) => {
        //             assert_eq!(counter, q.pos.unwrap());
        //             counter += 1;
        //         },
        //         QuestionaireEntry::Block(b) => {
        //             assert_eq!(counter, b.pos.unwrap());
        //             counter += 1;
        //         },
        //     };
        // }
    }

    #[test]
    fn test_string_validate_min_length() {
        let entry = StringEntry {
            default_value: None,
            min_length: Some(5),
            max_length: None,
            regexp: None,
        };
        assert!(entry.validate("12345", true).is_ok());
        assert!(entry.validate("1234", true).is_err());
    }

    #[test]
    fn test_string_validate_max_length() {
        let entry = StringEntry {
            default_value: None,
            min_length: None,
            max_length: Some(5),
            regexp: None,
        };
        assert!(entry.validate("12345", true).is_ok());
        assert!(entry.validate("123456", true).is_err());
    }

    #[test]
    fn test_string_validate_regex() {
        let entry = StringEntry {
            default_value: None,
            min_length: None,
            max_length: None,
            regexp: Some(r"^\d+$".to_string()),
        };
        assert!(entry.validate("12345", true).is_ok());
        assert!(entry.validate("1234a", true).is_err());
    }

    #[test]
    fn test_string_validate_all_constraints_met() {
        let entry = StringEntry {
            default_value: None,
            min_length: Some(3),
            max_length: Some(5),
            regexp: Some(r"^\d+$".to_string()),
        };
        assert!(entry.validate("123", true).is_ok());
        assert!(entry.validate("1234", true).is_ok());
        assert!(entry.validate("12345", true).is_ok());
        assert!(entry.validate("12", true).is_err());
        assert!(entry.validate("123456", true).is_err());
        assert!(entry.validate("12a34", true).is_err());
    }

    #[test]
    fn test_string_validate_no_constraints() {
        let entry = StringEntry {
            default_value: None,
            min_length: None,
            max_length: None,
            regexp: None,
        };
        assert!(entry.validate("any string", true).is_ok());
    }

    #[test]
    fn test_string_validate_combined_constraints() {
        let entry = StringEntry {
            default_value: None,
            min_length: Some(3),
            max_length: Some(5),
            regexp: Some(r"^\d+$".to_string()),
        };

        // Valid input that meets all constraints
        assert_eq!(entry.validate("123", true).unwrap(), QuestionAnswerInput::String(Some("123".to_string())));
        assert_eq!(entry.validate("1234", true).unwrap(), QuestionAnswerInput::String(Some("1234".to_string())));
        assert_eq!(entry.validate("12345", true).unwrap(), QuestionAnswerInput::String(Some("12345".to_string())));

        // Invalid inputs
        assert!(entry.validate("12", true).is_err()); // Too short
        assert!(entry.validate("123456", true).is_err()); // Too long
        assert!(entry.validate("12a34", true).is_err()); // Invalid format (non-digit character)
    }

    #[test]
    fn test_int_validate_with_empty_input_with_default_value() {
        let entry = IntEntry {
            default_value: Some(10),
            max: None,
            min: None,
        };
        let result = entry.validate("", true);
        assert_eq!(result.unwrap(), QuestionAnswerInput::Int(Some(10)));
    }

    #[test]
    fn test_int_validate_with_empty_input_without_default_value() {
        let entry = IntEntry {
            default_value: None,
            max: None,
            min: None,
        };
        let result = entry.validate("", true);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "No default value is set. Input is needed.");
    }

    #[test]
    fn test_int_validate_with_non_empty_input_non_integer() {
        let entry = IntEntry {
            default_value: Some(10),
            max: None,
            min: None,
        };
        let result = entry.validate("abc", true);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Input can not be cast into an int value.");
    }

    #[test]
    fn test_int_validate_with_non_empty_input_integer_within_bounds() {
        let entry = IntEntry {
            default_value: Some(10),
            max: Some(100),
            min: Some(1),
        };
        let result = entry.validate("50", true);
        assert_eq!(result.unwrap(), QuestionAnswerInput::Int(Some(50)));
    }

    #[test]
    fn test_int_validate_with_non_empty_input_integer_below_min() {
        let entry = IntEntry {
            default_value: Some(10),
            max: Some(100),
            min: Some(20),
        };
        let result = entry.validate("10", true);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Input doesn't respect min value constraint.");
    }

    #[test]
    fn test_int_validate_with_non_empty_input_integer_above_max() {
        let entry = IntEntry {
            default_value: Some(10),
            max: Some(100),
            min: Some(1),
        };
        let result = entry.validate("150", true);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Input doesn't respect max value constraint.");
    }

    #[test]
    fn test_float_validate_with_valid_input() {
        let entry = FloatEntry { default_value: None, max: None, min: None };
        let result = entry.validate("42.0", true);
        assert_eq!(result.unwrap(), QuestionAnswerInput::Float(Some(42.0)));
    }

    #[test]
    fn test_float_validate_with_empty_input_and_default() {
        let entry = FloatEntry { default_value: Some(3.14), max: None, min: None };
        let result = entry.validate("", true);
        assert_eq!(result.unwrap(), QuestionAnswerInput::Float(Some(3.14)));
    }

    #[test]
    fn test_float_validate_with_empty_input_no_default() {
        let entry = FloatEntry { default_value: None, max: None, min: None };
        let result = entry.validate("", true);
        assert!(result.is_err());
    }

    #[test]
    fn test_float_validate_with_invalid_input() {
        let entry = FloatEntry { default_value: None, max: None, min: None };
        let result = entry.validate("not_a_float", true);
        assert!(result.is_err());
    }

    #[test]
    fn test_float_validate_with_input_less_than_min() {
        let entry = FloatEntry { default_value: None, max: None, min: Some(10.0) };
        let result = entry.validate("5.0", true);
        assert!(result.is_err());
    }

    #[test]
    fn test_float_validate_with_input_greater_than_max() {
        let entry = FloatEntry { default_value: None, max: Some(10.0), min: None };
        let result = entry.validate("15.0", true);
        assert!(result.is_err());
    }

    #[test]
    fn test_float_validate_with_input_within_min_max() {
        let entry = FloatEntry { default_value: None, max: Some(10.0), min: Some(1.0) };
        let result = entry.validate("5.0", true);
        assert_eq!(result.unwrap(), QuestionAnswerInput::Float(Some(5.0)));
    }

    #[test]
    fn test_bool_validate_with_empty_input_and_default() {
        let entry = BoolEntry { default_value: Some(true) };
        let result = entry.validate("", true);
        assert_eq!(result.unwrap(), QuestionAnswerInput::Bool(Some(true)));
    }

    #[test]
    fn test_bool_validate_with_empty_input_and_no_default() {
        let entry = BoolEntry { default_value: None };
        let result = entry.validate("", true);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "No default value is set. Input is needed.");
    }

    #[test]
    fn test_bool_validate_with_valid_true_inputs() {
        let entry = BoolEntry { default_value: None };
        let inputs = ["y", "yes", "true"];
        for &input in inputs.iter() {
            let result = entry.validate(input, true);
            assert_eq!(result.unwrap(), QuestionAnswerInput::Bool(Some(true)));
        }
    }

    #[test]
    fn test_bool_validate_with_valid_false_inputs() {
        let entry = BoolEntry { default_value: None };
        let inputs = ["n", "no", "false"];
        for &input in inputs.iter() {
            let result = entry.validate(input, true);
            assert_eq!(result.unwrap(), QuestionAnswerInput::Bool(Some(false)));
        }
    }

    #[test]
    fn test_bool_validate_with_invalid_input() {
        let entry = BoolEntry { default_value: None };
        let result = entry.validate("invalid", true);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Input can not be cast into a bool value.");
    }

    #[test]
    fn test_option_validate_with_valid_input() {
        let options = vec!["Option1".to_string(), "Option2".to_string(), "Option3".to_string()];
        let option_entry = OptionEntry {
            default_value: None,
            options,
        };

        let result = option_entry.validate("0", true);
        assert_eq!(result.unwrap(), QuestionAnswerInput::Option(Some("Option1".to_string())));
    }


    #[test]
    fn test_option_validate_with_empty_input_and_default_value() {
        let options = vec!["Option1".to_string(), "Option2".to_string(), "Option3".to_string()];
        let option_entry = OptionEntry {
            default_value: Some(1),
            options,
        };

        let result = option_entry.validate("1", true);
        assert_eq!(result.unwrap(), QuestionAnswerInput::Option(Some("Option2".to_string())));
    }

    #[test]
    fn test_option_validate_with_empty_input_and_invalid_default_value() {
        let options = vec!["Option1".to_string(), "Option2".to_string()];
        let option_entry = OptionEntry {
            default_value: Some(5),
            options,
        };

        let result = option_entry.validate("2", true);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Default value index is bigger than the options list");
    }

    #[test]
    fn test_option_validate_with_empty_input_and_no_default_value() {
        let options = vec!["Option1".to_string(), "Option2".to_string()];
        let option_entry = OptionEntry {
            default_value: None,
            options,
        };

        let result = option_entry.validate("", true);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "No default value is set. Input is needed.");
    }}
