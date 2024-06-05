use crate::controller::{ControllerResult, QuestionaireResult};
use crate::questionaire::{AnswerEntry, BlockAnswer, QuestionAnswerInput, QuestionEntry, QuestionaireEntry, RepeatedQuestionEntry, SubBlock} ;
use anyhow::{anyhow, Result};
use colored::Colorize;
use serde::{Deserialize, Serialize};

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::io::{BufRead, BufReader};


pub trait QuestionairePersistence {
    fn store_block(&mut self, entry: &SubBlock, data: &BlockAnswer) -> Result<()>;
    fn store_question(&mut self, entry: &QuestionEntry, data: &QuestionAnswerInput) -> Result<()>;
    fn store_repeated_question(&mut self, entry: &RepeatedQuestionEntry, data: &Vec<QuestionAnswerInput>) -> Result<()>;
    fn load(&mut self) -> Result<()>;
}

pub struct FileQuestionairePersistence  {
    file: String,
    data: Vec<(String, AnswerEntry)>,
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

    fn store<T: Serialize>(&mut self, id: &str, answer: &T, type_marker: &str) -> Result<()> {
        let json_string = serde_json::to_string(answer).unwrap();
        let txt = format!("{}:{}={}",type_marker, id, json_string);
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
        self.data.push((entry.id.to_string(), AnswerEntry::Block(data.clone())));
        self.store(&entry.id, data, "B")
    }

    fn store_question(&mut self, entry: &QuestionEntry, data: &QuestionAnswerInput) -> Result<()> {
        self.data.push((entry.id.to_string(), AnswerEntry::Question(data.clone())));
        self.store(&entry.id, data, "Q")
    }

    fn store_repeated_question(&mut self, entry: &RepeatedQuestionEntry, data: &Vec<QuestionAnswerInput>) -> Result<()> {
        self.data.push((entry.id.to_string(), AnswerEntry::RepeatedQuestion(data.clone())));
        self.store(&entry.id, data, "R")
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
    fn store_block(&mut self, _entry: &SubBlock, _data: &BlockAnswer) -> Result<()> {
        Ok(())
    }

    fn store_question(&mut self, _entry: &QuestionEntry, _data: &QuestionAnswerInput) -> Result<()> {
        Ok(())
    }

    fn store_repeated_question(&mut self, _entry: &RepeatedQuestionEntry, _data: &Vec<QuestionAnswerInput>) -> Result<()> {
        Ok(())
    }

    fn load(&mut self) -> Result<()> {
        Err(anyhow!("Not supported"))
    }
}

pub fn load_tmp_file(file_path: &str) -> Result<Vec<(String, AnswerEntry)>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut ret: Vec<(String, AnswerEntry)> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let prefix = &line[..2];
        let content = &line[3..];
        let (id, json_str) = if let Some(index) = line.find("=") {
            (&content[..index], &content[index+1..])
        } else {
            continue;
        };
        match prefix {
            "Q:" => {
                if let Ok(o) = serde_json::from_str::<QuestionAnswerInput>(&json_str) {
                    ret.push((id.to_string(), AnswerEntry::Question(o)));
                }
            },
            "R:" => {
                if let Ok(o) = serde_json::from_str::<Vec<QuestionAnswerInput>>(&json_str) {
                    ret.push((id.to_string(), AnswerEntry::RepeatedQuestion(o)));
                }
            },
            "B:" => {
                if let Ok(o) = serde_json::from_str::<BlockAnswer>(&json_str) {
                    ret.push((id.to_string(), AnswerEntry::Block(o)));
                }
            },
            _ => {
                break;
            }
        }
    }
    Ok(ret)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_load_tmp_file() {
        if let Ok(v) = load_tmp_file("res/tquest.tmp") {
            assert_eq!(19, v.len());
            // checking that the ids are unique
            // checking that the content can be deserialized
        } else {
            panic!("error while loading test file");
        }
    }
}