use std::borrow::BorrowMut;

use crate::{
    questionaire::{AnswerEntry, BlockAnswer, Questionaire, SubBlock},
    ui::{ProceedScreenResult, QuestionScreenResult, QuestionaireView},
    QuestionaireEntry,
};
use anyhow::Result;


#[derive(Debug)]
pub enum QuestionaireResult {
    Canceled,
    Finished(Vec<AnswerEntry>),
}

pub enum ControllerResult {
    Canceled,
    Finished(AnswerEntry),
}


pub struct QController<V: QuestionaireView> {
    questionaire: Questionaire,
    view: V,
}

impl<V: QuestionaireView> QController<V> {
    pub fn new(questionaire: Questionaire, view: V) -> Self {
        Self { questionaire, view }
    }

    pub fn run(&mut self) -> Result<QuestionaireResult> {
        match run_sub_block(&mut self.view, &self.questionaire.init_block)? {
            ControllerResult::Canceled => return Ok(QuestionaireResult::Canceled),
            ControllerResult::Finished(answers) => return Ok(
                QuestionaireResult::Finished(vec![answers])
            ),
        }
    }
}

fn enter_sub_block<V: QuestionaireView> (
    view: &mut V,
    sub_block: &SubBlock,
) -> Result<ControllerResult> {
    //let block_answers: Vec<AnswerEntry> = Vec::new();
    let mut block_answer: BlockAnswer = BlockAnswer {
        id: sub_block.id.clone(),
        iterations: Vec::new(),
    };
    loop {
        let mut iteration_answers: Vec<AnswerEntry> = Vec::new();
        for e in &sub_block.entries {
            // ask the sub-queries ...
            match e {
                QuestionaireEntry::Question(q) => {
                    match view.show_question_screen(&q)? {
                        QuestionScreenResult::Canceled => return Ok(ControllerResult::Canceled),
                        QuestionScreenResult::Proceeded(answer) => {
                            iteration_answers.push(AnswerEntry::Question(answer));
                        }
                    }
                }
                QuestionaireEntry::Block(b) => {
                    match run_sub_block(view, b)? {
                        ControllerResult::Canceled => return Ok(ControllerResult::Canceled),
                        ControllerResult::Finished(answer) => {
                            iteration_answers.push(answer);
                        }
                    }
                }
            }
        }
        block_answer.iterations.push(iteration_answers);
        if let Some(end_text) = sub_block.end_text.as_deref() {
            match view.show_proceed_screen(
                &sub_block.id,
                end_text,
                sub_block.help_text.as_deref(),
            )? {
                ProceedScreenResult::Canceled => return Ok(ControllerResult::Canceled),
                ProceedScreenResult::Proceeded(b) => {
                    if ! b {
                        return Ok(ControllerResult::Finished(
                            AnswerEntry::Block(block_answer)
                        ));
                    }
                }
            }
        } else {
            break;
        }
    }
    Ok(ControllerResult::Finished(
        AnswerEntry::Block(block_answer)
    ))
}


fn run_sub_block<V: QuestionaireView> (
    view: &mut V,
    sub_block: &SubBlock,
) -> Result<ControllerResult> {
    //self.view.show_proceed_screen("dummy id", "dummy query", "dummy help");
    match view.show_proceed_screen(
        &sub_block.id,
        &sub_block.start_text,
        sub_block.help_text.as_deref(),
    )? {
        ProceedScreenResult::Canceled => return Ok(ControllerResult::Canceled),
        ProceedScreenResult::Proceeded(b) => return enter_sub_block(view, sub_block),
    }
}
