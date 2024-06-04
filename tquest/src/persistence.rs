use crate::controller::{ControllerResult, QuestionaireResult};
use crate::questionaire::{BlockAnswer, QuestionAnswerInput, QuestionEntry, QuestionaireEntry, RepeatedQuestionEntry, SubBlock} ;
use anyhow::{anyhow, Result};
use colored::Colorize;

use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub trait QuestionairePersistence {
    fn store_block(&mut self, entry: &SubBlock, data: &BlockAnswer) -> Result<()>;
    fn store_question(&mut self, entry: &QuestionEntry, data: &QuestionAnswerInput) -> Result<()>;
    fn store_repeated_question(&mut self, entry: &RepeatedQuestionEntry, data: &Vec<QuestionAnswerInput>) -> Result<()>;
    fn load(&mut self) -> Result<()>;
}

#[derive(Default)]
pub struct FileQuestionairePersistence  {
    file: String,
    pub debug: bool,
}

impl FileQuestionairePersistence  {
    pub fn new(file: &str) -> Result<FileQuestionairePersistence> {
        let ret = FileQuestionairePersistence {
            file: file.to_string(),
            debug: true,
        };
        Ok(ret)
    }

    fn write_to_file(&mut self, txt: &str) -> Result<()> {
        if self.debug {
            println!("{}", txt.blue().italic());
        }
        let p = Path::new(&self.file);
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(p)?;
        write!(file, "{}\n", txt)?;
        Ok(())
    }
}

impl QuestionairePersistence for FileQuestionairePersistence {
    fn store_block(&mut self, entry: &SubBlock, data: &BlockAnswer) -> Result<()> {
        let json_string = serde_json::to_string(&data).unwrap();
        let output = format!("{}={}", entry.id, json_string);
        self.write_to_file(&output)?;
        Ok(()) // TODO
    }

    fn store_question(&mut self, entry: &QuestionEntry, data: &QuestionAnswerInput) -> Result<()> {
        let json_string = serde_json::to_string(&data).unwrap();
        let output = format!("{}={}", entry.id, json_string);
        self.write_to_file(&output)?;
        Ok(()) // TODO
    }

    fn store_repeated_question(&mut self, entry: &RepeatedQuestionEntry, data: &Vec<QuestionAnswerInput>) -> Result<()> {
        let json_string = serde_json::to_string(&data).unwrap();
        let output = format!("{}={}", entry.id, json_string);
        self.write_to_file(&output)?;
        Ok(()) // TODO
    }

    fn load(&mut self) -> Result<()> {
        Ok(()) // TODO
    }
}

pub struct NoPersistence {
}

impl NoPersistence {
    pub fn new() -> Self {
        NoPersistence{}
    }
}

impl QuestionairePersistence for NoPersistence {
    fn store_block(&mut self, entry: &SubBlock, data: &BlockAnswer) -> Result<()> {
        Ok(())
    }

    fn store_question(&mut self, entry: &QuestionEntry, data: &QuestionAnswerInput) -> Result<()> {
        Ok(())
    }

    fn store_repeated_question(&mut self, entry: &RepeatedQuestionEntry, data: &Vec<QuestionAnswerInput>) -> Result<()> {
        Ok(())
    }

    fn load(&mut self) -> Result<()> {
        Err(anyhow!("Not supported"))
    }
}
