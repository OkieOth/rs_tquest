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
                    } else {
                        // TODO ... it's some kind of critical. What's happen if the last question
                        // is answered with 'No' but it should not be looped
                        if ! sub_block.loop_over_entries {
                            break;
                        }
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
    sub_block: &SubBlock
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

#[cfg(test)]
mod tests {
    use crate::questionaire::{QuestionEntry, EntryType, StringEntry, QuestionAnswerInput};

    use super::*;    

    #[test]
    fn it_travers_01() {
        enum UiMockAnswer {

        }

        #[derive(Default)]
        struct UiMock {
            current_step: usize,
            answers: Vec<UiMockAnswer>,
        }
    
        impl QuestionaireView for UiMock {
            fn show_proceed_screen<'a, T: Into<Option<&'a str>>>(&mut self, id: &str, text: &str, help_text: T) -> Result<ProceedScreenResult> {
                if (self.current_step != 0) && (self.current_step != 3) {
                    panic!("unexpected proceed screen: {}: {}", self.current_step, text);
                }
                println!("proceed question: {}", text);
                self.current_step += 1; 
                Ok(ProceedScreenResult::Proceeded(true))
            }
            fn show_question_screen(&mut self, question_entry: &QuestionEntry) -> Result<QuestionScreenResult>{
                if (self.current_step != 1) && (self.current_step != 2) {
                    panic!("unexpected question screen: {}: {}", self.current_step, question_entry.query_text);
                } 
                println!("normal question: {}", question_entry.query_text);
                self.current_step += 1; 
                Ok(QuestionScreenResult::Proceeded(QuestionAnswerInput::String(format!("step: {}", self.current_step))))
            }
        }    

        let ui = UiMock::default();
        let questionaire = Questionaire::builder().add_init_block_and_build (
            "id00",
            "In the following questionaire you will be asked about your family and things. Do you want to proceed?", 
            Some("All data are collected. Do you want to process them?"), 
            None, 
            Some(
                vec![
                    QuestionaireEntry::Question (
                        QuestionEntry::builder()
                        .id("id01")
                        .query_text("What's your name?")
                        .entry_type(EntryType::String(
                            StringEntry::builder()
                            .min_length(2)
                            .max_length(100)
                            .build().unwrap()
                        ))
                        .build().unwrap(),
                    ),
                    QuestionaireEntry::Question (
                        QuestionEntry::builder()
                        .query_text("What's your date of birth?")
                        .id("id01")
                        .help_text("Provide the date of birth in YYYY-MM-DD format".to_string())
                        .entry_type(EntryType::String(
                            StringEntry::builder()
                            .reqexp("\\d\\d\\d\\d-\\d\\d-\\d\\d".to_string())
                            .build().unwrap()
                        ))
                        .build().unwrap()
                    )
                ]),
                );
    
        let mut c: QController<UiMock> = QController::new(questionaire, ui);
        match c.run() {
            Ok(r) => {
                println!("result: {:?}", r);
            },
            Err(e) => panic!("received Err as questionaire result"),
        }
        
    }
}
