use crate::questionaire::{QuestionAnswerInput, QuestionEntry, QuestionAnswer} ;
use anyhow::{anyhow, Result};
use colored::Colorize;
use serde::Serialize;

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::io::{BufRead, BufReader};


pub trait QuestionairePersistence {
    fn store_question(&mut self, entry: &QuestionEntry, data: &QuestionAnswerInput) -> Result<()>;
    fn load(&mut self, source: Option<&str>) -> Result<()>;
    fn import(&mut self, data_to_import: &Vec<QuestionAnswer>);
    fn next_answer(&mut self) -> Option<QuestionAnswer>;
    fn next_answer_id(&mut self) -> Option<String>;
}

pub struct FileQuestionairePersistence  {
    file: String,
    data: Vec<QuestionAnswer>,
    pub debug: bool,
    current_pos: usize,
}

impl FileQuestionairePersistence  {
    pub fn new(file: &str) -> Result<FileQuestionairePersistence> {
        let ret = FileQuestionairePersistence {
            file: file.to_string(),
            data: vec![],
            debug: false,
            current_pos: 0,
        };
        Ok(ret)
    }

    fn store<T: Serialize>(&mut self, id: &str, answer: &T) -> Result<()> {
        let json_string = serde_json::to_string(answer).unwrap();
        let txt = format!("{}={}",id, json_string);
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
    fn store_question(&mut self, entry: &QuestionEntry, data: &QuestionAnswerInput) -> Result<()> {
        // TODO: remove, that mess up the load and fast-forward mode ... and it's not needed
        //self.data.push((entry.id.to_string(), data.clone()));
        self.store(&entry.id, data)
    }

    fn load(&mut self, source: Option<&str>) -> Result<()> {
        if let Some(file_path) = source {
            self.data = load_tmp_file(&file_path)?;
            Ok(())
        } else {
            Err(anyhow!("No source for loading given"))
        }
    }

    fn import (&mut self, data_to_import: &Vec<QuestionAnswer>) {
        for i in data_to_import {
            self.data.push(i.clone());
        }
    }

    fn next_answer(&mut self) -> Option<QuestionAnswer> {
        if self.current_pos < self.data.len() {
            let e = self.data.get(self.current_pos);
            self.current_pos += 1;
            if let Some(a) = e {
                return Some(a.clone())
            }
            None
        } else {
            None
        }
    }

    fn next_answer_id(&mut self) -> Option<String> {
        if self.current_pos < self.data.len() {
            let e = self.data.get(self.current_pos);
            if let Some(a) = e {
                Some(a.id.to_string())
            } else {
                None
            }
        } else {
            None
        }
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
    fn store_question(&mut self, _entry: &QuestionEntry, _data: &QuestionAnswerInput) -> Result<()> {
        Ok(())
    }

    fn load(&mut self, _s: Option<&str>) -> Result<()> {
        Err(anyhow!("Not supported"))
    }

    fn import(&mut self, _data_to_import: &Vec<QuestionAnswer>) {
    }

    fn next_answer(&mut self) -> Option<QuestionAnswer> {
        None
    }

    fn next_answer_id(&mut self) -> Option<String> {
        None
    }

}

pub fn load_tmp_file(file_path: &str) -> Result<Vec<QuestionAnswer>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut ret: Vec<QuestionAnswer> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let (id, json_str) = if let Some(index) = line.find("=") {
            (&line[..index], &line[index+1..])
        } else {
            continue;
        };
        if let Ok(o) = serde_json::from_str::<QuestionAnswerInput>(&json_str) {
            let qa = QuestionAnswer {
                id: id.to_string(),
                answer: o,
            };
            ret.push(qa);
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
            assert_eq!(14, v.len());
            v.iter().enumerate().for_each(|(index, a)| {
                match index {
                    0 => assert_eq!("id01".to_string(), *a.id),
                    1 => assert_eq!("id02".to_string(), *a.id),
                    2 => assert_eq!("id03_01_01".to_string(), *a.id),
                    3 => assert_eq!("id03_01_02".to_string(), *a.id),
                    4 => assert_eq!("id03_01_01".to_string(), *a.id),
                    5 => assert_eq!("id03_01_02".to_string(), *a.id),
                    6 => assert_eq!("id04_01".to_string(), *a.id),
                    7 => assert_eq!("id04_02".to_string(), *a.id),
                    8 => assert_eq!("id04_03".to_string(), *a.id),
                    9 => assert_eq!("id04_04_01".to_string(), *a.id),
                    10 => assert_eq!("id04_04_02".to_string(), *a.id),
                    11 => assert_eq!("id04_01".to_string(), *a.id),
                    12 => assert_eq!("id04_02".to_string(), *a.id),
                    13 => assert_eq!("id04_03".to_string(), *a.id),
                    _ => panic!("more elements than expected"),
                };
            });
        } else {
            panic!("error while loading test file");
        }
    }

    #[test]
    fn test_next() {
        let mut persistence = FileQuestionairePersistence::new("tmp/tquest.tmp").unwrap();
        persistence.load(Some("res/tquest.tmp")).expect("error while loading old persistence file");
        assert_eq!("id01".to_string(), persistence.next_answer_id().unwrap());
        let _ = persistence.next_answer().unwrap();
        assert_eq!("id02".to_string(), persistence.next_answer_id().unwrap());
        let _ = persistence.next_answer().unwrap();
        assert_eq!("id03_01_01".to_string(), persistence.next_answer_id().unwrap());
        let _ = persistence.next_answer().unwrap();
        assert_eq!("id03_01_02".to_string(), persistence.next_answer_id().unwrap());
        let _ = persistence.next_answer().unwrap();
        assert_eq!("id03_01_01".to_string(), persistence.next_answer_id().unwrap());
        let _ = persistence.next_answer().unwrap();
        assert_eq!("id03_01_02".to_string(), persistence.next_answer_id().unwrap());
        let _ = persistence.next_answer().unwrap();
        assert_eq!("id04_01".to_string(), persistence.next_answer_id().unwrap());
        let _ = persistence.next_answer().unwrap();
        assert_eq!("id04_02".to_string(), persistence.next_answer_id().unwrap());
        let _ = persistence.next_answer().unwrap();
        assert_eq!("id04_03".to_string(), persistence.next_answer_id().unwrap());
        let _ = persistence.next_answer().unwrap();
        assert_eq!("id04_04_01".to_string(), persistence.next_answer_id().unwrap());
        let _ = persistence.next_answer().unwrap();
        assert_eq!("id04_04_02".to_string(), persistence.next_answer_id().unwrap());
        let _ = persistence.next_answer().unwrap();
        assert_eq!("id04_01".to_string(), persistence.next_answer_id().unwrap());
        let _ = persistence.next_answer().unwrap();
        assert_eq!("id04_02".to_string(), persistence.next_answer_id().unwrap());
        let _ = persistence.next_answer().unwrap();
        assert_eq!("id04_03".to_string(), persistence.next_answer_id().unwrap());
        let _ = persistence.next_answer().unwrap();
        assert_eq!(None, persistence.next_answer_id());
        assert_eq!(None, persistence.next_answer());
        assert_eq!(None, persistence.next_answer_id());
        assert_eq!(None, persistence.next_answer());
    }
}