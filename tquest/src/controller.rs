use crate::{questionaire::{QuestionAnswer, Questionaire}, ui::QuestionaireView, QuestionaireEntry};
use anyhow::Result;

pub struct QController<V: QuestionaireView> {
    questionaire: Questionaire,
    view: V,
}

impl<V: QuestionaireView> QController<V> {
    pub fn new(questionaire: Questionaire, view: V) -> Self {
        Self {
            questionaire,
            view,
        }
    }

    pub fn run(&mut self) -> Result<Option<Vec<QuestionAnswer>>> {
        let mut ret: Vec<QuestionAnswer> = Vec::new();
        for e in &self.questionaire {
            match e {
                QuestionaireEntry::Question(q) => {
                    self.view.show_question_screen(&q);
                },
                QuestionaireEntry::Block(b) => {
                    //self.view.show_proceed_screen("dummy id", "dummy query", "dummy help");
                    self.view.show_proceed_screen("dummy id", "dummy query", None);
                }
            }
        }
        Ok(Some(ret))
    }
}