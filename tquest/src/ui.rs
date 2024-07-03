
use anyhow::Result;
use colored::Colorize;
use std::io;
use std::process;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use crate::questionaire::{QuestionAnswerInput, QuestionEntry, StringEntry, 
    IntEntry, FloatEntry, BoolEntry, OptionEntry, EntryType};


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

pub enum MsgLevel {
    Normal,
    Urgent,
    Critical,
}

pub trait QuestionaireView {
    fn print_title<'a>(&mut self, _title: &str) {}
    fn show_proceed_screen<'a, T: Into<Option<&'a str>>>(&mut self, id: &str, text: &str, help_text: T, question_count: usize, current: usize, preferred: Option<bool>) -> Result<ProceedScreenResult>;
    fn show_question_screen(&mut self, question_entry: &QuestionEntry, question_count: usize, preferred: Option<QuestionAnswerInput>) -> Result<QuestionScreenResult>;
    fn show_msg(&mut self, _msg: &str, _level: MsgLevel) {}
}

trait ViewHelper {
    fn get_input_hint(&self) -> String;
}


pub struct Ui {
    pub fast_forward: bool,
}


impl Ui  {
    pub fn new() -> Result<Self> {
        Ok(Self {
            fast_forward: false,
        }) 
    }
}


impl QuestionaireView for Ui {
    fn print_title<'a>(&mut self, title: &str) {
        println!("\n________________________________________________________________________________");
        println!("\n{}\n", title.bold().underline());
    }

    fn show_msg<'a>(&mut self, msg: &str, level: MsgLevel) {
        match level {
            MsgLevel::Normal => {
                println!("\n{}\n", msg.italic());
            },
            _ => {
                println!("\n{}\n", msg.yellow().italic());
            }
        }
    }

    fn show_proceed_screen<'a, T: Into<Option<&'a str>>>(&mut self, _id: &str, text: &str, help_text: T, question_count: usize, current: usize, preferred: Option<bool>) -> Result<ProceedScreenResult> {
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
            let msg = format!("Wrong input! {}", get_valid_input_hint(has_help));
            println!("\n{}\n",msg.yellow());
        }

        fn print_result_and_return(input: bool) -> Result<ProceedScreenResult> {
            if input {
                println!(">>> {}", format!("{}", YES).green());

            } else {
                println!(">>> {}", format!("{}", NO).green());
            }
            Ok(ProceedScreenResult::Proceeded(input))
        }

        let ht = help_text.into();
        let text_to_display = if current != 0 {
            format!("[{}/{}] {}", current, question_count, text)
        } else {
            text.to_string()
        };
        println!("\n{}\n({})", text_to_display.bold(), get_valid_input_hint(ht.is_some()).dimmed());
        if let Some(a) = preferred {
            let preferred_txt = format!("{}", a).yellow().italic();
            println!("last input, take it with ⏎: {}", preferred_txt);
        }
        if self.fast_forward  && preferred.is_some() {
            // fast forward mode
            if let Some(a) = preferred {
                return print_result_and_return(a);
            } else {
                return print_result_and_return(true);
            }
        }
        let mut rl = DefaultEditor::new()?;
        loop {
            let readline = rl.readline(">> ");
            match readline {
                Ok(line) => {
                    match line.to_lowercase().as_str().trim() {
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
                                if let Some(a) = preferred {
                                    return print_result_and_return(a);
                                } else {
                                    return print_result_and_return(true);
                                }
                            } else {
                                print_wrong_input(ht.is_some());
                            }
                        }
                    }        
                },
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    process::exit(1);
                },
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    process::exit(1);
                },
                Err(err) => {
                    panic!("Error: {:?}", err);
                }
            }
        }
    }

    fn show_question_screen(&mut self, question_entry: &QuestionEntry, question_count: usize, preferred: Option<QuestionAnswerInput>) -> Result<QuestionScreenResult>{
        fn get_valid_input_hint(question_entry: &QuestionEntry) -> String {
            let mut s: String = match &question_entry.entry_type {
                EntryType::String (s) => {
                    s.get_input_hint()
                },
                EntryType::Int(s) => {
                    s.get_input_hint()
                },
                EntryType::Float(s) => {
                    s.get_input_hint()
                },
                EntryType::Bool(s) => {
                    s.get_input_hint()
                },
                EntryType::Option(s) => {
                    s.get_input_hint()
                },
                _ => {
                    "".to_string()
                },
            };
            if question_entry.help_text.is_some() {
                s.push_str(" (for more info type 'h')");
            };
            s
        }

        fn print_result_and_return(ret: QuestionAnswerInput) -> Result<QuestionScreenResult> {
            match &ret {
                QuestionAnswerInput::String(x) => {
                    if let Some(v) =x {
                        println!(">>> {}", format!("{}", v).green());
                    }
                },
                QuestionAnswerInput::Int(x) => {
                    if let Some(v) =x {
                        println!(">>> {}", format!("{}", v).green());
                    }
                },
                QuestionAnswerInput::Float(x) => {
                    if let Some(v) =x {
                        println!(">>> {}", format!("{}", v).green());
                    }
                },
                QuestionAnswerInput::Bool(x) => {
                    if let Some(v) =x {
                        println!(">>> {}", format!("{}", v).green());
                    }
                },
                QuestionAnswerInput::Option(x) => {
                    if let Some(v) =x {
                        println!(">>> {}", format!("{}", v).green());
                    }
                },
                QuestionAnswerInput::None => {
                        println!(">>> ???");
                }
            }
            return Ok(QuestionScreenResult::Proceeded(ret));
        }

        fn print_wrong_input(question_entry: &QuestionEntry) {
            let msg = format!("Wrong input! {}", get_valid_input_hint(question_entry));
            println!("{}",msg.yellow());
        }

        fn print_help_text(question_entry: &QuestionEntry) {
            let msg = if let Some(help_text) = &question_entry.help_text {
                format!("Help: {}", help_text)
            } else {
                "no help available".to_string()
            };
            println!("\n{}\n",msg.italic());
        }

        fn validate_input(str: &str, entry_type: &EntryType, required: bool) -> Result<QuestionAnswerInput> {
            match entry_type {
                EntryType::String (s) => {
                    s.validate(&str, required)
                },
                EntryType::Int(s) => {
                    s.validate(&str, required)
                },
                EntryType::Float(s) => {
                    s.validate(&str, required)
                },
                EntryType::Bool(s) => {
                    s.validate(&str, required)
                },
                EntryType::Option(s) => {
                    s.validate(&str, required)
                },
                _ => {
                    panic!("unexpected EntryType for question screen");
                }
            }
        }

        let text_to_display = format!("[{}/{}] {}", question_entry.pos, question_count, question_entry.query_text);
        println!("\n{}\n({})", text_to_display.bold(), get_valid_input_hint(&question_entry).dimmed());
        let preferred_txt = if let Some(a) = preferred {
            let s = format!("{}", a).yellow().italic();
            if self.fast_forward {
                // fast forward mode
                let input_txt: String = a.to_string();
                if let Ok(_ret) = validate_input(&input_txt, &question_entry.entry_type, question_entry.required) {
                    // validate was ok ...
                    return print_result_and_return(a);
                }
            }
    
            println!("last input, take it w/ ⏎: {}", s);
            format!("{}", a)
        } else {
            "".to_string()
        };
        let mut rl = DefaultEditor::new()?;
        loop {
            let readline = rl.readline(">> ");
            let mut input = String::new(); 

            match readline {
                Ok(line) => {
                    let mut str: String = line.trim().to_string();

                    if (str.len() == 0) && (preferred_txt.len() > 0) {
                        if preferred_txt.len() > 0 {
                            str = preferred_txt.clone();
                        };
                    };
        
                    if ((str == "h") || (str == "?")) && (question_entry.help_text.is_some()){
                        print_help_text(&question_entry);
                    } else {                
                        if let Ok(ret) = validate_input(&str, &question_entry.entry_type, question_entry.required) {
                            // validate was ok ...
                            return print_result_and_return(ret);
                        } else {
                            print_wrong_input(question_entry);
                        }
                    }        
                },
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    process::exit(1);
                },
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    process::exit(1);
                },
                Err(err) => {
                    panic!("Error: {:?}", err);
                }
            }
        }
    }
}


impl ViewHelper for StringEntry {
    fn get_input_hint(&self) -> String {
        let mut s = "Please enter a string and take it with ⏎".to_string();
        if let Some(def) = self.default_value.as_ref() {
            s.push_str(&format!(", default: {}", def));
        }
        if let Some(min) = self.min_length {
            s.push_str(&format!(", min-length: {}", min));
        }
        if let Some(max) = self.max_length {
            s.push_str(&format!(", max-length: {}", max));
        }
        if let Some(regexp) = self.regexp.as_ref() {
            s.push_str(&format!(", regexp: {}", regexp));
        }
        s
    }
}

impl ViewHelper for IntEntry {
    fn get_input_hint(&self) -> String {
        let mut s = "Please enter an integer and take it with ⏎".to_string();
        if let Some(def) = self.default_value {
            s.push_str(&format!(", default: {}", def));
        }
        if let Some(min) = self.min {
            s.push_str(&format!(", min: {}", min));
        }
        if let Some(max) = self.max {
            s.push_str(&format!(", max: {}", max));
        }
        s
    }
}

impl ViewHelper for FloatEntry {
    fn get_input_hint(&self) -> String {
        let mut s = "Please enter a floating point number (e.g. 1.123) and take it with ⏎".to_string();
        if let Some(def) = self.default_value {
            s.push_str(&format!(", default: {}", def));
        }
        if let Some(min) = self.min {
            s.push_str(&format!(", min: {}", min));
        }
        if let Some(max) = self.max {
            s.push_str(&format!(", max: {}", max));
        }
        s
    }
}

impl ViewHelper for BoolEntry {
    fn get_input_hint(&self) -> String {
        let mut s = "Please enter 'y' for 'yes' or 'n' for 'no'. Take it with ⏎".to_string();
        if let Some(def) = self.default_value {
            let def_str = if def { "[y]es" } else { "[n]o" };
            s.push_str(&format!(", default: {}", def_str));
        }
        s
    }
}

impl ViewHelper for OptionEntry {
    fn get_input_hint(&self) -> String {
        let mut s = "Please enter the number for one of this options and take it with ⏎".to_string();

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
