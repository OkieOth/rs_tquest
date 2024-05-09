use std::borrow::BorrowMut;

use crate::{
    questionaire::{QuestionAnswer, Questionaire, SubBlock},
    ui::QuestionaireView,
    QuestionaireEntry,
};
use anyhow::Result;

pub struct QController<V: QuestionaireView> {
    questionaire: Questionaire,
    view: V,
}

impl<V: QuestionaireView> QController<V> {
    pub fn new(questionaire: Questionaire, view: V) -> Self {
        Self { questionaire, view }
    }


    pub fn run(&mut self) -> Result<(bool, Option<Vec<QuestionAnswer>>)> {
        let mut ret: Vec<QuestionAnswer> = Vec::new();
        run_sub_block(&mut self.view, &self.questionaire.init_block, &mut ret);
        Ok((false, Some(ret)))
    }
}

fn run_sub_block<V: QuestionaireView> (
    view: &mut V,
    sub_block: &SubBlock,
    answers: &mut Vec<QuestionAnswer>,
) -> Result<bool> {
    //self.view.show_proceed_screen("dummy id", "dummy query", "dummy help");
    let answer_start = view.show_proceed_screen(
        &sub_block.id,
        &sub_block.start_text,
        sub_block.help_text.as_deref(),
    )?;

    if answer_start.0 {
        // cancel the questionaire
        return Ok(true);
    }
    if answer_start.1 {
        // the proceed screen was left with 'okay' ...
        loop {
            for e in &sub_block.entries {
                // ask the sub-queries ...
                match e {
                    QuestionaireEntry::Question(q) => {
                        view.show_question_screen(&q)?;
                    }
                    QuestionaireEntry::Block(b) => {
                        run_sub_block(view, b, answers)?;
                    }
                }
            }
            if let Some(end_text) = sub_block.end_text.as_deref() {
                let answer_end = view.show_proceed_screen(
                    &sub_block.id,
                    end_text,
                    sub_block.help_text.as_deref(),
                )?;
                if answer_end.0 {
                    return Ok(true);
                };
                if !answer_end.1 {
                    return Ok(false);
                }
            } else {
                break;
            }
        }
    }
    Ok(false)
}
