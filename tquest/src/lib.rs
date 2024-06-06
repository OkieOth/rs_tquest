mod ui;

mod questionaire;

mod controller;

mod persistence;

use controller::{QuestionaireController, QuestionaireResult};
use anyhow::{anyhow, Result};
use ui::{ProceedScreenResult, QuestionaireView};

use std::{fs, path::Path};

use persistence::{FileQuestionairePersistence, QuestionairePersistence};
pub use questionaire::{Questionaire, QuestionaireBuilder, QuestionaireEntry, QuestionEntry, RepeatedQuestionEntry, 
    SubBlock, EntryType, StringEntry, IntEntry, FloatEntry, BoolEntry, 
    OptionEntry};

pub use ui::Ui;

const PERSISTENCE_FILE_NAME: &str = "tquest.tmp";


pub fn run_questionaire(title: &str, questionaire: Questionaire) -> Result<QuestionaireResult> {
    fn check_for_old_persistence_file() -> bool {
        let p = Path::new(PERSISTENCE_FILE_NAME);
        p.is_file()
    }

    fn remove_persistence_file() {
        let p = Path::new(PERSISTENCE_FILE_NAME);
        if p.is_file() {
            let _ = fs::remove_file(p);
        }
    }

    let mut ui: Ui = Ui::new()?;
    if title.len() > 0 {
        ui.print_title(title);
    }
    let mut persistence = FileQuestionairePersistence::new(PERSISTENCE_FILE_NAME)?;

    let persistence_file_exists = check_for_old_persistence_file();

    if persistence_file_exists {
        let r = ui.show_proceed_screen("00", "Found persistence file, for a questionaire. Do you want to load it to proceed where you stopped last time?", None, 0, 0, None);
        match r {
            Ok(res) => {
                match res {
                    ProceedScreenResult::Canceled => {
                        return Err(anyhow!("Canceled by user"));
                    },
                    ProceedScreenResult::Proceeded(p) => {
                        if p {
                            let _ = persistence.load(Some(PERSISTENCE_FILE_NAME));
                        }
                    },
                }
            },
            Err(_) => {
                return Err(anyhow!("error while processing"));
            },
        };
    }

    let mut c: QuestionaireController<Ui, FileQuestionairePersistence> = QuestionaireController::new(questionaire, ui, persistence);
    if persistence_file_exists {
        remove_persistence_file();
    }
    c.run()

}


#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
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
                        .help_text("Provide the date of birth in YYYY-MM-DD format".to_string())
                        .entry_type(EntryType::String(
                            StringEntry::builder()
                            .regexp("\\d\\d\\d\\d-\\d\\d-\\d\\d".to_string())
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
                    .help_text("Provide the date of birth in YYYY-MM-DD format".to_string())
                    .entry_type(EntryType::String(
                        StringEntry::builder()
                        .regexp("\\d\\d\\d\\d-\\d\\d-\\d\\d".to_string())
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
                    .help_text("Provide the date of birth in YYYY-MM-DD format".to_string())
                    .entry_type(EntryType::String(
                        StringEntry::builder()
                        .regexp("\\d\\d\\d\\d-\\d\\d-\\d\\d".to_string())
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
                    .end_text("Do you have another sister?".to_string())
                    .entries(get_sister_questions(&id_block_1))
                    .loop_over_entries(true)
                    .build()
                ),
                QuestionaireEntry::Block(
                    SubBlock::builder()
                    .id(&id_block_2)
                    .start_text("Do you have a brother?")
                    .end_text("Do you have another brother?".to_string())
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
                    .help_text("Provide the year and optional month in 'YYYY-MM' or 'YYYY' format.".to_string())
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
                    .help_text("Provide the main reason for leaving".to_string())
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
                    .help_text("Provide the year and optional month in 'YYYY-MM' or 'YYYY' format".to_string())
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
                    .help_text("Provide the date of birth in YYYY-MM-DD format".to_string())
                    .entry_type(EntryType::String(
                        StringEntry::builder()
                        .regexp("\\d\\d\\d\\d-\\d\\d-\\d\\d".to_string())
                        .build()
                    ))
                    .build()
                ),
                QuestionaireEntry::Block(
                    SubBlock::builder()
                    .id("id03")
                    .start_text("Do you have brothers or sisters?")
                    .end_text("Do you have more brothers and sisters?".to_string())
                    .entries(get_sibling_entries("id03"))
                    .loop_over_entries(true)
                    .build()
                ),
                QuestionaireEntry::Block(
                    SubBlock::builder()
                    .id("id04")
                    .start_text("Have you already worked in a job?")
                    .end_text("Have you worked in another job?".to_string())
                    .entries(get_job_entries("id04"))
                    .loop_over_entries(true)
                    .build()
                )
            ]
        )
        .build()
    }
}