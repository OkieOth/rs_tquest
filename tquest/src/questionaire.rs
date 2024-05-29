//! Main types of the questionaire implementation
//! 
use std::str::FromStr;

use builder_m4cro::{Builder, BuilderFromDefault};
use anyhow::{Result, anyhow};
use regex::{self, Regex};

/// Expected string entry
#[derive(Debug, Builder, Clone)]
pub struct StringEntry {
    pub default_value: Option<String>,
    pub reqexp: Option<String>,
    pub max_length: Option<usize>,
    pub min_length: Option<usize>,
}

impl StringEntry {
    pub fn validate<'a>(&self, input: &'a str) -> Result<QuestionAnswerInput> {
        if input.len() == 0 {
            if let Some(def_value) = self.default_value.as_ref() {
                return Ok(QuestionAnswerInput::String(def_value.clone()));
            } else {
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
        if let Some(regex) = self.reqexp.as_ref() {
            let re = Regex::from_str(regex).unwrap();
            if ! re.is_match(input) {
                return Err(anyhow!("Max input len not respected"));
            }
        }
        Ok(QuestionAnswerInput::String(input.to_string()))
    }
}


/// Expected int entry
#[derive(Debug, Builder, Clone)]
pub struct IntEntry {
    pub default_value: Option<i32>,
    pub max: Option<i32>,
    pub min: Option<i32>,
}

impl IntEntry {
    pub fn validate<'a>(&self, input: &'a str) -> Result<QuestionAnswerInput> {
        if input.len() == 0 {
            if let Some(def_value) = self.default_value {
                return Ok(QuestionAnswerInput::Int(def_value));
            } else {
                return Err(anyhow!("No default value is set. Input is needed."));
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
        Ok(QuestionAnswerInput::Int(input_value))
    }
}

/// Expected floating point entry
#[derive(Debug, Builder, Clone)]
pub struct FloatEntry {
    pub default_value: Option<f32>,
    pub max: Option<f32>,
    pub min: Option<f32>,
}

impl FloatEntry {
    pub fn validate<'a>(&self, input: &'a str) -> Result<QuestionAnswerInput> {
        if input.len() == 0 {
            if let Some(def_value) = self.default_value {
                return Ok(QuestionAnswerInput::Float(def_value));
            } else {
                return Err(anyhow!("No default value is set. Input is needed."));
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
        Ok(QuestionAnswerInput::Float(input_value))
    }
}

/// Expected bool entry
#[derive(Debug, Builder, Clone)]
pub struct BoolEntry {
    pub default_value: Option<bool>,
}

impl BoolEntry {
    pub fn validate<'a>(&self, input: &'a str) -> Result<QuestionAnswerInput> {
        if input.len() == 0 {
            if let Some(def_value) = self.default_value {
                return Ok(QuestionAnswerInput::Bool(def_value));
            } else {
                return Err(anyhow!("No default value is set. Input is needed."));
            }
        }
        match input.to_lowercase().as_str() {
            "y" | "yes" | "true" => Ok(QuestionAnswerInput::Bool(true)),
            "n" | "no" | "false" => Ok(QuestionAnswerInput::Bool(false)),
            _ => return Err(anyhow!("Input can not be cast into a bool value.")),
        }
    }
}

/// Expected String entry based on a number of predefined options
#[derive(Debug, Builder, Clone)]
pub struct OptionEntry {
    /// represents the index of the array, used as default value
    pub default_value: Option<u32>,

    /// valid input options
    pub options: Vec<String>,
}

impl OptionEntry {
    pub fn validate<'a>(&self, input: &'a str) -> Result<QuestionAnswerInput> {
        if input.len() == 0 {
            if let Some(def_value) = self.default_value {
                if def_value < self.options.len() as u32 {
                    return Ok(QuestionAnswerInput::Option(self.options.get(def_value as usize).unwrap().clone()));
                } else {
                    return Err(anyhow!("Default value index is bigger than the options list"));
                }
            } else {
                return Err(anyhow!("No default value is set. Input is needed."));
            }
        }
        if let Ok(i) = input.parse::<usize>() {
            if i < self.options.len() {
                return Ok(QuestionAnswerInput::Option(self.options.get(i as usize).unwrap().clone()));
            } else {
                return Err(anyhow!("Default value index is bigger than the options list"));
            }
        } else {
            return Err(anyhow!("Input can't be cast into the option index."));
        };
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
    Option(String),
}

#[derive(Debug, Default)]
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

    #[test]
    fn test_int_validate_with_empty_input_with_default_value() {
        let entry = IntEntry {
            default_value: Some(10),
            max: None,
            min: None,
        };
        let result = entry.validate("");
        assert_eq!(result.unwrap(), QuestionAnswerInput::Int(10));
    }

    #[test]
    fn test_int_validate_with_empty_input_without_default_value() {
        let entry = IntEntry {
            default_value: None,
            max: None,
            min: None,
        };
        let result = entry.validate("");
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
        let result = entry.validate("abc");
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
        let result = entry.validate("50");
        assert_eq!(result.unwrap(), QuestionAnswerInput::Int(50));
    }

    #[test]
    fn test_int_validate_with_non_empty_input_integer_below_min() {
        let entry = IntEntry {
            default_value: Some(10),
            max: Some(100),
            min: Some(20),
        };
        let result = entry.validate("10");
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
        let result = entry.validate("150");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Input doesn't respect max value constraint.");
    }

    #[test]
    fn test_float_validate_with_valid_input() {
        let entry = FloatEntry { default_value: None, max: None, min: None };
        let result = entry.validate("42.0");
        assert_eq!(result.unwrap(), QuestionAnswerInput::Float(42.0));
    }

    #[test]
    fn test_float_validate_with_empty_input_and_default() {
        let entry = FloatEntry { default_value: Some(3.14), max: None, min: None };
        let result = entry.validate("");
        assert_eq!(result.unwrap(), QuestionAnswerInput::Float(3.14));
    }

    #[test]
    fn test_float_validate_with_empty_input_no_default() {
        let entry = FloatEntry { default_value: None, max: None, min: None };
        let result = entry.validate("");
        assert!(result.is_err());
    }

    #[test]
    fn test_float_validate_with_invalid_input() {
        let entry = FloatEntry { default_value: None, max: None, min: None };
        let result = entry.validate("not_a_float");
        assert!(result.is_err());
    }

    #[test]
    fn test_float_validate_with_input_less_than_min() {
        let entry = FloatEntry { default_value: None, max: None, min: Some(10.0) };
        let result = entry.validate("5.0");
        assert!(result.is_err());
    }

    #[test]
    fn test_float_validate_with_input_greater_than_max() {
        let entry = FloatEntry { default_value: None, max: Some(10.0), min: None };
        let result = entry.validate("15.0");
        assert!(result.is_err());
    }

    #[test]
    fn test_float_validate_with_input_within_min_max() {
        let entry = FloatEntry { default_value: None, max: Some(10.0), min: Some(1.0) };
        let result = entry.validate("5.0");
        assert_eq!(result.unwrap(), QuestionAnswerInput::Float(5.0));
    }

    #[test]
    fn test_bool_validate_with_empty_input_and_default() {
        let entry = BoolEntry { default_value: Some(true) };
        let result = entry.validate("");
        assert_eq!(result.unwrap(), QuestionAnswerInput::Bool(true));
    }

    #[test]
    fn test_bool_validate_with_empty_input_and_no_default() {
        let entry = BoolEntry { default_value: None };
        let result = entry.validate("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "No default value is set. Input is needed.");
    }

    #[test]
    fn test_bool_validate_with_valid_true_inputs() {
        let entry = BoolEntry { default_value: None };
        let inputs = ["y", "yes", "true"];
        for &input in inputs.iter() {
            let result = entry.validate(input);
            assert_eq!(result.unwrap(), QuestionAnswerInput::Bool(true));
        }
    }

    #[test]
    fn test_bool_validate_with_valid_false_inputs() {
        let entry = BoolEntry { default_value: None };
        let inputs = ["n", "no", "false"];
        for &input in inputs.iter() {
            let result = entry.validate(input);
            assert_eq!(result.unwrap(), QuestionAnswerInput::Bool(false));
        }
    }

    #[test]
    fn test_bool_validate_with_invalid_input() {
        let entry = BoolEntry { default_value: None };
        let result = entry.validate("invalid");
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

        let result = option_entry.validate("0");
        assert_eq!(result.unwrap(), QuestionAnswerInput::Option("Option1".to_string()));
    }


    #[test]
    fn test_option_validate_with_empty_input_and_default_value() {
        let options = vec!["Option1".to_string(), "Option2".to_string(), "Option3".to_string()];
        let option_entry = OptionEntry {
            default_value: Some(1),
            options,
        };

        let result = option_entry.validate("1");
        assert_eq!(result.unwrap(), QuestionAnswerInput::Option("Option2".to_string()));
    }

    #[test]
    fn test_option_validate_with_empty_input_and_invalid_default_value() {
        let options = vec!["Option1".to_string(), "Option2".to_string()];
        let option_entry = OptionEntry {
            default_value: Some(5),
            options,
        };

        let result = option_entry.validate("2");
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

        let result = option_entry.validate("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "No default value is set. Input is needed.");
    }}
