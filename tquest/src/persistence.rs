use crate::controller::{ControllerResult, QuestionaireResult};
use crate::questionaire::{BlockAnswer, QuestionAnswerInput, QuestionEntry, QuestionaireEntry, RepeatedQuestionEntry, SubBlock} ;
use anyhow::{anyhow, Result};
use colored::Colorize;
use serde::{Deserialize, Serialize};

use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub trait QuestionairePersistence {
    fn store_block(&mut self, entry: &SubBlock, data: &BlockAnswer) -> Result<()>;
    fn store_question(&mut self, entry: &QuestionEntry, data: &QuestionAnswerInput) -> Result<()>;
    fn store_repeated_question(&mut self, entry: &RepeatedQuestionEntry, data: &Vec<QuestionAnswerInput>) -> Result<()>;
    fn load(&mut self) -> Result<()>;
}

pub struct FileQuestionairePersistence  {
    file: String,
    data: Vec<(String, String)>,
    pub debug: bool,
}

impl FileQuestionairePersistence  {
    pub fn new(file: &str) -> Result<FileQuestionairePersistence> {
        let ret = FileQuestionairePersistence {
            file: file.to_string(),
            data: vec![],
            debug: true,
        };
        Ok(ret)
    }

    fn store<T: Serialize>(&mut self, id: &str, answer: &T) -> Result<()> {
        let json_string = serde_json::to_string(answer).unwrap();
        self.data.push((id.to_string(), json_string.clone()));
        let txt = format!("{}={}", id, json_string);
        if self.debug {
            println!("{}", txt.blue().italic());
        }
        self.write_to_file(&txt)
    }


    fn write_to_file(&mut self, txt: &str) -> Result<()> {
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
        self.store(&entry.id, data)
    }

    fn store_question(&mut self, entry: &QuestionEntry, data: &QuestionAnswerInput) -> Result<()> {
        self.store(&entry.id, data)
    }

    fn store_repeated_question(&mut self, entry: &RepeatedQuestionEntry, data: &Vec<QuestionAnswerInput>) -> Result<()> {
        self.store(&entry.id, data)
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
