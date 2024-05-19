
use anyhow::Result;
use colored::Colorize;
use std::io;

use crate::questionaire::{QuestionAnswerInput, QuestionEntry, StringEntry, IntEntry, FloatEntry, BoolEntry, OptionEntry};


/// This is returned for normal question entries.
pub enum QuestionScreenResult {
    Canceled,
    Proceeded(QuestionAnswerInput)
}

/// This type is returned for questions, where a decision how to
/// proceed is needed
pub enum ProceedScreenResult {
    Canceled,
    Proceeded(bool)
}


pub trait QuestionaireView {
    fn print_title<'a>(&mut self, title: &str);
    fn show_proceed_screen<'a, T: Into<Option<&'a str>>>(&mut self, id: &str, text: &str, help_text: T) -> Result<ProceedScreenResult>;
    fn show_question_screen(&mut self, question_entry: &QuestionEntry) -> Result<QuestionScreenResult>;
}

trait ViewHelper {
    fn get_input_hint(&self) -> String;
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
        println!("\n________________________________________________________________________________");
        println!("\n{}\n", title.bold().underline());
    }

    fn show_proceed_screen<'a, T: Into<Option<&'a str>>>(&mut self, _id: &str, text: &str, help_text: T) -> Result<ProceedScreenResult> {
        const YES: &str = "yes";
        const NO: &str = "no";

        fn get_valid_input_hint<'b>(has_help: bool) -> &'b str{
            if has_help {
                "type [y|n] or only ENTER for yes (for more info type 'h')"
            } else {
                "type [y|n] or only ENTER for yes"
            }
        }

        fn print_wrong_input(has_help: bool) {
            let msg = format!("Wrong input! Allowed are: {}", get_valid_input_hint(has_help));
            println!("\n{}\n",msg.red());
        }

        fn print_result_and_return(input: bool) -> Result<ProceedScreenResult> {
            if input {
                println!(">>> {}\n", format!("{}", YES).green());

            } else {
                println!(">>> {}\n", format!("{}", NO).green());
            }
            Ok(ProceedScreenResult::Proceeded(input))
        }

        let ht = help_text.into();
        println!("\n{}", text.bold());
        println!("\n{}",get_valid_input_hint(ht.is_some()).dimmed());

        loop {
            let mut input = String::new(); 
            io::stdin().read_line(&mut input).expect("error while reading from stdin");
            match input.to_lowercase().as_str().trim() {
                "y" | "yes" => return print_result_and_return(true),
                "n" | "no" => return print_result_and_return(false),
                "h" | "help" | "?" => {
                    if let Some(help_text_str) = ht {
                        println!("\n{}\n", help_text_str);
                        println!("\n{}\n",get_valid_input_hint(ht.is_some()).dimmed());
                    } else {
                        print_wrong_input(ht.is_some());
                    }
                },
                other => {
                    if other.len() == 0 {
                        return print_result_and_return(true);
                    } else {
                        print_wrong_input(ht.is_some());
                    }
                }
            }
        }
    }

    fn show_question_screen(&mut self, question_entry: &QuestionEntry) -> Result<QuestionScreenResult>{
        fn get_valid_input_hint<'b>(question_entry: &QuestionEntry) -> &'b str{
            if question_entry.help_text.is_some() {
                "type [y|n] or only ENTER for yes (for more info type 'h')"
            } else {
                "type [y|n] or only ENTER for yes"
            }
        }

        println!("\n{}", question_entry.query_text.bold());
        println!("\n{}",get_valid_input_hint(&question_entry).dimmed());

        Ok(QuestionScreenResult::Canceled)
    }
}


impl ViewHelper for StringEntry {
    fn get_input_hint(&self) -> String {
        let mut s = "Please enter a string and take it with ENTER".to_string();
        if let Some(def) = self.default_value.as_ref() {
            s.push_str(&format!(", default: {}", def));
        }
        if let Some(min) = self.min_length {
            s.push_str(&format!(", min-length: {}", min));
        }
        if let Some(max) = self.max_length {
            s.push_str(&format!(", max-length: {}", max));
        }
        if let Some(regexp) = self.reqexp.as_ref() {
            s.push_str(&format!(", regexp: {}", regexp));
        }
        s
    }
}

impl ViewHelper for IntEntry {
    // #[derive(Debug, Builder, Clone)]
    // pub struct IntEntry {
    //     pub default_value: Option<i32>,
    //     pub max: Option<i32>,
    //     pub min: Option<i32>,
    // }
    
    fn get_input_hint(&self) -> String {
        let mut s = "Please enter an integer and take it with ENTER".to_string();
        todo!()
    }
}

impl ViewHelper for FloatEntry {
    // #[derive(Debug, Builder, Clone)]
    // pub struct FloatEntry {
    //     pub default_value: Option<i32>,
    //     pub max: Option<f32>,
    //     pub min: Option<f32>,
    // }
    
    fn get_input_hint(&self) -> String {
        let mut s = "Please enter a floating point number (e.g. 1.123) and take it with ENTER.".to_string();
        todo!()
    }
}

impl ViewHelper for BoolEntry {
    fn get_input_hint(&self) -> String {
        let mut s = "Please enter 'y' for 'yes' or 'n' for 'no'. Take it with ENTER.".to_string();
        if let Some(def) = self.default_value {
            let def_str = if def { "[y]es" } else { "[n]o" };
            s.push_str(&format!(", default: {}", def_str));
        }
        s
    }
}

impl ViewHelper for OptionEntry {
    fn get_input_hint(&self) -> String {
        let mut s = "Please enter the number for one of this options and take it with ENTER.".to_string();

        let default_index = if let Some(def) = self.default_value {
            def
        } else {
            0
        };

        for (i, o) in self.options.iter().enumerate() {
            if i == default_index as usize {
                s.push_str(&format!("\n  [{}] {} (default)", i, o));
            } else {
                s.push_str(&format!("\n  [{}] {}", i, o));
            }
        }
        s
    }
}
