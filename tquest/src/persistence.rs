use crate::controller::QuestionaireResult;
use anyhow::{anyhow, Result};

pub trait QuestionairePersistence {
    fn store(&mut self, data: QuestionaireResult) -> Result<()>;
    fn load(&mut self) -> Result<QuestionaireResult>;
}

pub struct FileQuestionairePersistence {
}

impl FileQuestionairePersistence {
    pub fn new(file: &str) -> Result<FileQuestionairePersistence> {
        Err(anyhow!("TODO")) // TODO
    }
}

impl QuestionairePersistence for FileQuestionairePersistence {
    fn store(&mut self, data: QuestionaireResult) -> Result<()> {
        Err(anyhow!("TODO")) // TODO
    }

    fn load(&mut self) -> Result<QuestionaireResult> {
        Err(anyhow!("TODO")) // TODO
    }
}