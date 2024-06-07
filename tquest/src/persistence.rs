use crate::questionaire::{QuestionAnswerInput, QuestionEntry} ;
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
    fn next_answer(&mut self) -> Option<QuestionAnswerInput>;
    fn next_answer_id(&mut self) -> Option<String>;
}

pub struct FileQuestionairePersistence  {
    file: String,
    data: Vec<(String, QuestionAnswerInput)>,
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

    fn next_answer(&mut self) -> Option<QuestionAnswerInput> {
        if self.current_pos < self.data.len() {
            let e = self.data.get(self.current_pos);
            self.current_pos += 1;
            if let Some((_, a)) = e {
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
            if let Some((id, _)) = e {
                Some(id.to_string())
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

    fn next_answer(&mut self) -> Option<QuestionAnswerInput> {
        None
    }

    fn next_answer_id(&mut self) -> Option<String> {
        None
    }

}

pub fn load_tmp_file(file_path: &str) -> Result<Vec<(String, QuestionAnswerInput)>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut ret: Vec<(String, QuestionAnswerInput)> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let (id, json_str) = if let Some(index) = line.find("=") {
            (&line[..index], &line[index+1..])
        } else {
            continue;
        };
        if let Ok(o) = serde_json::from_str::<QuestionAnswerInput>(&json_str) {
            ret.push((id.to_string(), o));
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
            v.iter().enumerate().for_each(|(index, (id, _))| {
                match index {
                    0 => assert_eq!("id01".to_string(), *id),
                    1 => assert_eq!("id02".to_string(), *id),
                    2 => assert_eq!("id03_01_01".to_string(), *id),
                    3 => assert_eq!("id03_01_02".to_string(), *id),
                    4 => assert_eq!("id03_01_01".to_string(), *id),
                    5 => assert_eq!("id03_01_02".to_string(), *id),
                    6 => assert_eq!("id04_01".to_string(), *id),
                    7 => assert_eq!("id04_02".to_string(), *id),
                    8 => assert_eq!("id04_03".to_string(), *id),
                    9 => assert_eq!("id04_04_01".to_string(), *id),
                    10 => assert_eq!("id04_04_02".to_string(), *id),
                    11 => assert_eq!("id04_01".to_string(), *id),
                    12 => assert_eq!("id04_02".to_string(), *id),
                    13 => assert_eq!("id04_03".to_string(), *id),
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