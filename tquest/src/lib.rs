mod ui;

mod questionaire;

mod controller;

mod persistence;

use controller::QuestionaireController;
use anyhow::{anyhow, Result};
use ui::{ProceedScreenResult, QuestionaireView};

use std::{fs, path::Path};

pub use persistence::{FileQuestionairePersistence, QuestionairePersistence};
pub use questionaire::{Questionaire, QuestionaireBuilder, QuestionaireEntry, QuestionEntry, RepeatedQuestionEntry, 
    SubBlock, EntryType, StringEntry, IntEntry, FloatEntry, BoolEntry, 
    OptionEntry, BlockAnswer, QuestionAnswerInput, AnswerEntry, RepeatedQuestionAnswers, QuestionAnswer};
pub use controller::QuestionaireResult;
pub use ui::Ui;

const PERSISTENCE_FILE_NAME: &str = "tquest.tmp";
const TITLE: &str = "A short questionaire";

pub struct QuestionaireRunner {
    persistence_file: String,
    imported_data: Option<Vec<QuestionAnswer>>,
    title: String,
    autofil: bool,
    questionaire: Questionaire,
}

impl QuestionaireRunner {
    pub fn builder() -> QuestionaireRunnerBuilder {
        QuestionaireRunnerBuilder::default()
    }

    fn check_for_old_persistence_file(&self) -> bool {
        let p = Path::new(&self.persistence_file);
        p.is_file()
    }
    
    fn remove_persistence_file(&self) {
        let p = Path::new(&self.persistence_file);
        if p.is_file() {
            let _ = fs::remove_file(p);
        }
    }

    fn handle_persistence_file(&self, persistence: &mut FileQuestionairePersistence, ui: &mut Ui) -> Result<bool> {
        if self.check_for_old_persistence_file() {
            let r = ui.show_proceed_screen("00", "Found persistence file, for a questionaire. Do you want to load it to proceed where you stopped last time?", None, 0, 0, None);
            match r {
                Ok(res) => {
                    match res {
                        ProceedScreenResult::Canceled => {
                            return Err(anyhow!("Canceled by user"));
                        },
                        ProceedScreenResult::Proceeded(p) => {
                            if p {
                                let _ = persistence.load(Some(&self.persistence_file));
                            }
                        },
                    }
                },
                Err(_) => {
                    return Err(anyhow!("error while processing"));
                },
            };
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn run(&self) -> Result<QuestionaireResult> {    
        let mut ui: Ui = Ui::new()?;
        ui.print_title(&self.title);
        let mut persistence = FileQuestionairePersistence::new(&self.persistence_file)?;
    
        let mut persistence_file_exists: bool = false;
        if self.imported_data.is_some() {
            persistence.import(self.imported_data.as_ref().unwrap());
        } else {
            persistence_file_exists = self.handle_persistence_file(&mut persistence, &mut ui)?;
        }
    
        ui.fast_forward = self.autofil;
        let mut c: QuestionaireController<Ui, FileQuestionairePersistence> = QuestionaireController::new(&self.questionaire, ui, persistence);
        if persistence_file_exists {
            self.remove_persistence_file();
        }
        c.run()
    }
}

#[derive(Default)]
pub struct QuestionaireRunnerBuilder {
    persistence_file: Option<String>,
    title: Option<String>,
    autofil: bool,
    imported_data: Option<Vec<QuestionAnswer>>,
}

impl QuestionaireRunnerBuilder {
    pub fn persistence_file(&mut self, v: &str) -> &mut Self {
        self.persistence_file = Some(v.to_string());
        self
    }
    pub fn title(&mut self, v: &str) -> &mut Self {
        self.title = Some(v.to_string());
        self
    }
    pub fn imported_data(&mut self, v: Option<Vec<QuestionAnswer>>) -> &mut Self {
        self.imported_data = v;
        self
    }
    pub fn autofil(&mut self, v: bool) -> &mut Self {
        self.autofil = v;
        self
    }
    pub fn build(&self, questionaire: Questionaire) -> Result<QuestionaireRunner> {
        let persistence_file = if let Some (pf) = self.persistence_file.as_ref() {
            pf.to_string()
        } else {
            PERSISTENCE_FILE_NAME.to_string()
        };
        let title = if let Some (t) = self.title.as_ref() {
            t.to_string()
        } else {
            TITLE.to_string()
        };
        let imported_data = if let Some(id) = self.imported_data.as_ref() {
            Some(id.clone())
        } else {
            None
        };
        Ok(QuestionaireRunner {
            persistence_file,
            title,
            autofil: self.autofil,
            imported_data,
            questionaire,
        })
    }

}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    #[ignore]
    fn test_fast_forward() {
        use test_helper::create_complex_questionaire;
        // Thread start - here
        let tmp_file = "tmp/test_persistence.tmp";
        let source_file = "res/tquest.tmp";
        let mut persistence = FileQuestionairePersistence::new(tmp_file).unwrap();
        persistence.load(Some(source_file)).expect("error while loading 'res/tquest.tmp'");
        let mut ui: Ui = Ui::new().expect("error while crating UI");
        ui.fast_forward =  true;

        let p = Path::new(tmp_file);
        if p.is_file() {
            let _ = fs::remove_file(p);
        }

        // Channel for communication
        let (tx, rx) = std::sync::mpsc::channel::<()>();
        std::thread::spawn(move || {
            let questionaire = create_complex_questionaire();
            let mut c: QuestionaireController<Ui, FileQuestionairePersistence> = QuestionaireController::new(&questionaire, ui, persistence);
            let _ = c.run();
            tx.send(()).expect("Error sending termination signal"); // Signal thread termination
        });

    // Wait for thread or timeout
        let result = rx.recv_timeout(std::time::Duration::from_secs(2));

        // Handle results
        match result {
            Ok(_) => panic!("Thread finished execution within timeout"),
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                // TODO compare the input with the output file ...
                if let Ok(v) = test_helper::is_same_file(tmp_file, source_file) {
                    assert!(v);
                } else {
                    panic!("something went wrong");
                }
            },
            Err(_) => panic!("Error receiving termination signal"),
        }
        
    }


}

#[cfg(test)]
mod test_helper {
    use crate::{
        questionaire::{Questionaire, SubBlock, StringEntry, QuestionEntry, 
            EntryType, OptionEntry},
        QuestionaireEntry,
    };

    use std::{io::Write, path::Path};
    use std::fs::{File, remove_file};
    use std::io::{BufReader, Read};



    #[test]
    fn test_json() {
        let p = Path::new("tmp");
        if ! p.is_dir() {
            let _ = std::fs::create_dir(p);
        }
        let pf = Path::new("tmp/test.json");
        if p.is_file() {
            let _ = remove_file(pf);
        }

        assert!(!p.is_file());
    
        let q = create_complex_questionaire();
        let json_string = serde_json::to_string(&q).unwrap();

        let mut file = File::create("tmp/test.json").unwrap();
        let _ = file.write_all(json_string.as_bytes());

        assert!(pf.is_file());

        let q2: Questionaire = serde_json::from_str(&json_string).unwrap();
        assert_eq!(q, q2);
    }
    
    pub fn create_small_questionaire() -> Questionaire {
        Questionaire::builder()
            .id("id00")
            .start_text("In the following questionaire you will be asked about your family and things. Do you want to proceed?")
            .end_text("All data are collected. Do you want to process them?")
            .questions(vec![
                    QuestionaireEntry::Question (
                        QuestionEntry::builder()
                        .id("id01")
                        .query_text("What's your name?")
                        .entry_type(EntryType::String(
                            StringEntry::builder()
                            .min_length(2)
                            .max_length(100)
                            .build()
                        ))
                        .build()
                    ),
                    QuestionaireEntry::Question (
                        QuestionEntry::builder()
                        .query_text("What's your date of birth?")
                        .id("id01")
                        .help_text("Provide the date of birth in YYYY-MM-DD format")
                        .entry_type(EntryType::String(
                            StringEntry::builder()
                            .regexp("\\d\\d\\d\\d-\\d\\d-\\d\\d")
                            .build()
                        ))
                        .build()
                    )
                ])
            .build()
    }

    pub fn create_complex_questionaire() -> Questionaire {
        fn get_brother_questions(id_pre: &str) -> Vec<QuestionaireEntry> {
            vec![
                QuestionaireEntry::Question (
                    QuestionEntry::builder()
                    .id(&format!("{}_01", id_pre))
                    .query_text("What's his name?")
                    .entry_type(EntryType::String(
                        StringEntry::builder()
                        .min_length(2)
                        .max_length(50)
                        .build()
                    ))
                    .build()
                ),
                QuestionaireEntry::Question (
                    QuestionEntry::builder()
                    .id(&format!("{}_02", id_pre))
                    .query_text("What's his date of birth?")
                    .help_text("Provide the date of birth in YYYY-MM-DD format")
                    .entry_type(EntryType::String(
                        StringEntry::builder()
                        .regexp("\\d\\d\\d\\d-\\d\\d-\\d\\d")
                        .build()
                    ))
                    .build()
                ),
            ]
        }
        
        fn get_sister_questions(id_pre: &str) -> Vec<QuestionaireEntry> {
            vec![
                QuestionaireEntry::Question (
                    QuestionEntry::builder()
                    .id(&format!("{}_01", id_pre))
                    .query_text("What's her name?")
                    .entry_type(EntryType::String(
                        StringEntry::builder()
                        .min_length(2)
                        .max_length(50)
                        .build()
                    ))
                    .build()
                ),
                QuestionaireEntry::Question (
                    QuestionEntry::builder()
                    .id(&format!("{}_02", id_pre))
                    .query_text("What's her date of birth?")
                    .help_text("Provide the date of birth in YYYY-MM-DD format")
                    .entry_type(EntryType::String(
                        StringEntry::builder()
                        .regexp("\\d\\d\\d\\d-\\d\\d-\\d\\d")
                        .build()
                    ))
                    .build()
                ),
            ]
        }
        
        fn get_sibling_entries(id_pre: &str) -> Vec<QuestionaireEntry> {
            let id_block_1 = format!("{}_01", id_pre);
            let id_block_2 = format!("{}_02", id_pre);
            vec![
                QuestionaireEntry::Block(
                    SubBlock::builder()
                    .id(&id_block_1)
                    .start_text("Do you have a sister?")
                    .end_text("Do you have another sister?")
                    .entries(get_sister_questions(&id_block_1))
                    .loop_over_entries(true)
                    .build()
                ),
                QuestionaireEntry::Block(
                    SubBlock::builder()
                    .id(&id_block_2)
                    .start_text("Do you have a brother?")
                    .end_text("Do you have another brother?")
                    .entries(get_brother_questions(&id_block_2))
                    .loop_over_entries(true)
                    .build()
                )
            ]
        }
        
        fn get_job_end_entries(id_pre: &str) -> Vec<QuestionaireEntry> {
            vec![
                QuestionaireEntry::Question(
                    QuestionEntry::builder()
                    .id(&format!("{}_01", id_pre))
                    .query_text("What was your end date there?")
                    .help_text("Provide the year and optional month in 'YYYY-MM' or 'YYYY' format.")
                    .entry_type(EntryType::String(
                        StringEntry::builder()
                        .min_length(2)
                        .max_length(100)
                        .build()
                    ))
                    .build()
                ),
                QuestionaireEntry::Question(
                    QuestionEntry::builder()
                    .id(&format!("{}_02", id_pre))
                    .query_text("Why did you leave the job?")
                    .help_text("Provide the main reason for leaving")
                    .entry_type(EntryType::Option(
                        OptionEntry::builder()
                        .options(vec![
                            "I left by my own".to_string(),
                            "I was laid off".to_string(),
                            "Other reason".to_string(),
                        ])
                        .build()
                    ))
                    .build()
                )
            ]
        }
        
        fn get_job_entries(id_pre: &str) -> Vec<QuestionaireEntry> {
            vec![
                QuestionaireEntry::Question(
                    QuestionEntry::builder()
                    .id(&format!("{}_01", id_pre))
                    .query_text("What was the name of the company you worked for?")
                    .entry_type(EntryType::String(
                        StringEntry::builder()
                        .min_length(2)
                        .max_length(200)
                        .build()
                    ))
                    .build()
                ),
                QuestionaireEntry::Question(
                    QuestionEntry::builder()
                    .id(&format!("{}_02", id_pre))
                    .query_text("What was your job title?")
                    .entry_type(EntryType::String(
                        StringEntry::builder()
                        .min_length(2)
                        .max_length(100)
                        .build()
                    ))
                    .build()
                ),
                QuestionaireEntry::Question(
                    QuestionEntry::builder()
                    .id(&format!("{}_03", id_pre))
                    .query_text("What was your start date there?")
                    .help_text("Provide the year and optional month in 'YYYY-MM' or 'YYYY' format")
                    .entry_type(EntryType::String(
                        StringEntry::builder()
                        .min_length(2)
                        .max_length(100)
                        .build()
                    ))
                    .build()
                ),
                QuestionaireEntry::Block(
                    SubBlock::builder()
                    .id(&format!("{}_04", id_pre))
                    .start_text("Have you finished your job there?")
                    .entries(get_job_end_entries(&format!("{}_04", id_pre)))
                    .build()
                )
            ]
        }
        
        Questionaire::builder()
        .id("id00")
        .title("Fun Questionaire")
        .start_text("In the following questionaire you will be asked about your family and things. Do you want to proceed?")
        .end_text("All data are collected. Do you want to process them?")
        .questions(
            vec![
                QuestionaireEntry::Question (
                    QuestionEntry::builder()
                    .id("id01")
                    .query_text("What's your name?")
                    .entry_type(EntryType::String(
                        StringEntry::builder()
                        .min_length(2)
                        .max_length(100)
                        .build()
                    ))
                    .build()
                ),
                QuestionaireEntry::Question (
                    QuestionEntry::builder()
                    .id("id02")
                    .query_text("What's your date of birth?")
                    .help_text("Provide the date of birth in YYYY-MM-DD format")
                    .entry_type(EntryType::String(
                        StringEntry::builder()
                        .regexp("\\d\\d\\d\\d-\\d\\d-\\d\\d")
                        .build()
                    ))
                    .build()
                ),
                QuestionaireEntry::Block(
                    SubBlock::builder()
                    .id("id03")
                    .start_text("Do you have brothers or sisters?")
                    .end_text("Do you have more brothers and sisters?")
                    .entries(get_sibling_entries("id03"))
                    .loop_over_entries(true)
                    .build()
                ),
                QuestionaireEntry::Block(
                    SubBlock::builder()
                    .id("id04")
                    .start_text("Have you already worked in a job?")
                    .end_text("Have you worked in another job?")
                    .entries(get_job_entries("id04"))
                    .loop_over_entries(true)
                    .build()
                )
            ]
        )
        .build()
    }

    pub fn is_same_file(file1: &str, file2: &str) -> Result<bool, std::io::Error> {
      let mut f1 = BufReader::new(File::open(file1)?);
      let mut f2 = BufReader::new(File::open(file2)?);
      // Check file sizes first
      if f1.get_ref().metadata()?.len() != f2.get_ref().metadata()?.len() {
        return Ok(false);
      }
      let mut buf1 = [0; 4096]; // Read in chunks of 4096 bytes
      let mut buf2 = [0; 4096];
      loop {
        let n1 = f1.read(&mut buf1)?;
        let n2 = f2.read(&mut buf2)?;
        if n1 != n2 || &buf1[..n1] != &buf2[..n2] {
          return Ok(false);
        }
        if n1 == 0 {
          return Ok(true);
        }
      }
    }
}