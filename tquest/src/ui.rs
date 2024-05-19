
use anyhow::Result;
use colored::Colorize;
use std::io;

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

    fn show_question_screen(&mut self, _question_entry: &QuestionEntry) -> Result<QuestionScreenResult>{
        Ok(QuestionScreenResult::Canceled)
    }
}

