use crate::{
    questionaire::{AnswerEntry, BlockAnswer, Questionaire, SubBlock},
    ui::{ProceedScreenResult, QuestionScreenResult, QuestionaireView},
    QuestionaireEntry,
};
use anyhow::Result;


#[derive(Debug)]
pub enum QuestionaireResult {
    Canceled,
    Finished(BlockAnswer),
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
        let pos_count = if let Some(pc) = self.questionaire.pos_count {
            pc
        } else {
            0
        };
        match run_sub_block(&mut self.view, &self.questionaire.init_block, true, pos_count)? {
            ControllerResult::Canceled => return Ok(QuestionaireResult::Canceled),
            ControllerResult::Finished(answers) => {
                match answers {
                    AnswerEntry::Block(ba) => {
                        return Ok(QuestionaireResult::Finished(ba));
                    },
                    AnswerEntry::Question(_) => panic!("receive wrong result for init-block"),
                }
            },
        }
    }
}

fn enter_sub_block<V: QuestionaireView> (
    view: &mut V,
    sub_block: &SubBlock,
    init: bool,
    question_count: usize
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
                    match view.show_question_screen(&q, question_count)? {
                        QuestionScreenResult::Canceled => return Ok(ControllerResult::Canceled),
                        QuestionScreenResult::Proceeded(answer) => {
                            iteration_answers.push(AnswerEntry::Question(answer));
                        }
                    }
                }
                QuestionaireEntry::Block(b) => {
                    match run_sub_block(view, b, false, question_count)? {
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
            let current: usize = if init {
                question_count
            } else {
                0
            };
            match view.show_proceed_screen(
                &sub_block.id,
                end_text,
                sub_block.help_text.as_deref(),
                question_count, current
            )? {
                ProceedScreenResult::Canceled => {
                    //return Ok(ControllerResult::Canceled)
                    break;
                },
                ProceedScreenResult::Proceeded(b) => {
                    if ! b {
                        if init {
                            return Ok(ControllerResult::Canceled);
                        } else {
                            break;
                        }
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
    sub_block: &SubBlock, init: bool, question_count: usize) -> Result<ControllerResult> {
    //self.view.show_proceed_screen("dummy id", "dummy query", "dummy help");
    let current = if let Some(p) = sub_block.pos {
        p
    } else {
        0
    };

    match view.show_proceed_screen(
        &sub_block.id,
        &sub_block.start_text,
        sub_block.help_text.as_deref(),
        question_count,
        current
    )? {
        ProceedScreenResult::Canceled => {
            return Ok(ControllerResult::Canceled);
        },
        ProceedScreenResult::Proceeded(b) => {
            if b {
                return enter_sub_block(view, sub_block, init, question_count);
            } else {
                if init {
                    return Ok(ControllerResult::Canceled);
                } else {
                    let mut ret: BlockAnswer = BlockAnswer::default();
                    ret.id = sub_block.id.clone();
                    return Ok(ControllerResult::Finished(AnswerEntry::Block(ret)));
                }
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::questionaire::{QuestionEntry, EntryType, StringEntry, QuestionAnswerInput};
    use crate::test_helper;
    use super::*;

    fn validate_question_string_input(ae: &AnswerEntry, expected_input: &str) {
        match ae {
            AnswerEntry::Block(_) => panic!("unexpected block answer"),
            AnswerEntry::Question(qa) => {
                if let QuestionAnswerInput::String(qai) = qa {
                    assert_eq!(expected_input, qai);
                } else {
                    panic!("expected StringInput, but got something different")
                }
            },
        }
    }

    #[test]
    fn it_travers_01() {
        #[derive(Default)]
        struct UiMock {
            current_step: usize,
        }
    
        impl QuestionaireView for UiMock {
            fn print_title<'a>(&mut self, _title: &str) {
            }

            fn show_proceed_screen<'a, T: Into<Option<&'a str>>>(&mut self, _id: &str, text: &str, _help_text: T, _question_count: usize, _current: usize) -> Result<ProceedScreenResult> {
                if (self.current_step != 0) && (self.current_step != 3) {
                    panic!("unexpected proceed screen: {}: {}", self.current_step, text);
                }
                println!("proceed question: {}", text);
                self.current_step += 1; 
                Ok(ProceedScreenResult::Proceeded(true))
            }
            fn show_question_screen(&mut self, question_entry: &QuestionEntry, _question_count: usize) -> Result<QuestionScreenResult>{
                if (self.current_step != 1) && (self.current_step != 2) {
                    panic!("unexpected question screen: {}: {}", self.current_step, question_entry.query_text);
                } 
                println!("normal question: {}", question_entry.query_text);
                self.current_step += 1; 
                Ok(QuestionScreenResult::Proceeded(QuestionAnswerInput::String(format!("step: {}", self.current_step))))
            }
        }
    
     
        let ui = UiMock::default();
        let questionaire = Questionaire::builder()
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
                            .reqexp("\\d\\d\\d\\d-\\d\\d-\\d\\d".to_string())
                            .build()
                        ))
                        .build()
                    )
                ])
            .build();
    
        let mut c: QController<UiMock> = QController::new(questionaire, ui);
        match c.run() {
            Ok(r) => {
                match r {
                    QuestionaireResult::Finished(ba) => {
                        println!("result: {:?}", &ba);
                        assert_eq!(1,ba.iterations.len());
                        let a = ba.iterations.get(0).unwrap();
                        assert_eq!(2, a.len());
                        let a0 = a.get(0).unwrap();
                        validate_question_string_input(&a0, "step: 2");
                        let a1 = a.get(1).unwrap();
                        validate_question_string_input(&a1, "step: 3");
                    },
                    QuestionaireResult::Canceled => {
                        panic!("received cancel from a valid questionaire flow");
                    }
                }
            },
            Err(_) => panic!("received Err as questionaire result"),
        }
        
    }

    #[test]
    fn it_travers_cancel_end() {
        
        #[derive(Default)]
        struct UiMock2 {
            current_step: usize,
        }

        impl QuestionaireView for UiMock2 {
            fn print_title<'a>(&mut self, _title: &str) {}
            fn show_proceed_screen<'a, T: Into<Option<&'a str>>>(&mut self, _id: &str, text: &str, _help_text: T, _question_count: usize, _current: usize) -> Result<ProceedScreenResult> {
                if (self.current_step != 0) && (self.current_step != 3) {
                    panic!("unexpected proceed screen: {}: {}", self.current_step, text);
                }
                println!("proceed question: {}", text);
                self.current_step += 1;
                if self.current_step == 4 {
                    Ok(ProceedScreenResult::Proceeded(false))

                } else {
                    Ok(ProceedScreenResult::Proceeded(true))
                }
            }
            fn show_question_screen(&mut self, question_entry: &QuestionEntry, _question_count: usize) -> Result<QuestionScreenResult>{
                if (self.current_step != 1) && (self.current_step != 2) {
                    panic!("unexpected question screen: {}: {}", self.current_step, question_entry.query_text);
                } 
                println!("normal question: {}", question_entry.query_text);
                self.current_step += 1; 
                Ok(QuestionScreenResult::Proceeded(QuestionAnswerInput::String(format!("step: {}", self.current_step))))
            }
        }
    

        let ui = UiMock2::default();
        let questionaire = test_helper::create_small_questionaire();
        let mut c: QController<UiMock2> = QController::new(questionaire, ui);
        let canceled: bool;
        match c.run() {
            Ok(r) => {
                match r {
                    QuestionaireResult::Finished(_ba) => {
                        panic!("received finished instead of canceled");
                    },
                    QuestionaireResult::Canceled => {
                        canceled = true;
                    }
                }
            },
            Err(_) => panic!("received Err as questionaire result"),
        }
        assert_eq!(true, canceled);
        
    }

    #[test]
    fn it_travers_jobs_only() {


        #[derive(Default)]
        struct UiMock {
            current_step: usize,
        }
    
        impl QuestionaireView for UiMock {
            fn print_title<'a>(&mut self, _title: &str) {
            }

            fn show_proceed_screen<'a, T: Into<Option<&'a str>>>(&mut self, _id: &str, _text: &str, _help_text: T, _question_count: usize, _current: usize) -> Result<ProceedScreenResult> {
                let ret = match self.current_step {
                    0 => ProceedScreenResult::Proceeded(true),  // Start
                    3 => ProceedScreenResult::Proceeded(false), // Siblings
                    4 => ProceedScreenResult::Proceeded(true),  // Worked in a job
                    8 => ProceedScreenResult::Proceeded(false), // Finished the job
                    9 => ProceedScreenResult::Proceeded(false), // Worked in another job
                    10 => ProceedScreenResult::Proceeded(true), // finish ... should proceed?
                    _ => panic!("unexpected proceed screen: step={}", self.current_step)
                };
                self.current_step += 1;
                Ok(ret)
            }
            fn show_question_screen(&mut self, _question_entry: &QuestionEntry, _question_count: usize) -> Result<QuestionScreenResult>{
                let ret = match self.current_step {
                    1 => QuestionScreenResult::Proceeded(QuestionAnswerInput::String("Homer".to_string())),
                    2 => QuestionScreenResult::Proceeded(QuestionAnswerInput::String("1956-03-12".to_string())),
                    5 => QuestionScreenResult::Proceeded(QuestionAnswerInput::String("Springfield Nuclear Power Plant".to_string())),
                    6 => QuestionScreenResult::Proceeded(QuestionAnswerInput::String("Nuclear safety inspector".to_string())),
                    7 => QuestionScreenResult::Proceeded(QuestionAnswerInput::String("1986".to_string())),
                    _ => panic!("unexpected question screen: step={}", self.current_step)
                };
                self.current_step += 1;
                Ok(ret)
            }
        }
    
     
        let ui = UiMock::default();
        let questionaire = test_helper::build_complex_questionaire();
    
        let mut c: QController<UiMock> = QController::new(questionaire, ui);
        match c.run() {
            Ok(r) => {
                match r {
                    QuestionaireResult::Finished(ba) => {
                        println!("result: {:?}", &ba);
                        // assert_eq!(1,ba.iterations.len());
                        // let a = ba.iterations.get(0).unwrap();
                        // assert_eq!(2, a.len());
                        // let a0 = a.get(0).unwrap();
                        // validate_question_string_input(&a0, "step: 2");
                        // let a1 = a.get(1).unwrap();
                        // validate_question_string_input(&a1, "step: 3");
                    },
                    QuestionaireResult::Canceled => {
                        panic!("received cancel from a valid questionaire flow");
                    }
                }
            },
            Err(_) => panic!("received Err as questionaire result"),
        }
    }

    #[test]
    fn it_travers_jobs_only_cancel() {
        #[derive(Default)]
        struct UiMock {
            current_step: usize,
        }
    
        impl QuestionaireView for UiMock {
            fn print_title<'a>(&mut self, _title: &str) {
            }

            fn show_proceed_screen<'a, T: Into<Option<&'a str>>>(&mut self, _id: &str, _text: &str, _help_text: T, _question_count: usize, _current: usize) -> Result<ProceedScreenResult> {
                let ret = match self.current_step {
                    0 => ProceedScreenResult::Proceeded(true),  // Start
                    3 => ProceedScreenResult::Proceeded(false), // Siblings
                    4 => ProceedScreenResult::Proceeded(true),  // Worked in a job
                    8 => ProceedScreenResult::Proceeded(false), // Finished the job
                    9 => ProceedScreenResult::Proceeded(false), // Worked in another job
                    10 => ProceedScreenResult::Proceeded(false), // finish ... should proceed?
                    _ => panic!("unexpected proceed screen: step={}", self.current_step)
                };
                self.current_step += 1;
                Ok(ret)
            }
            fn show_question_screen(&mut self, _question_entry: &QuestionEntry, _question_count: usize) -> Result<QuestionScreenResult>{
                let ret = match self.current_step {
                    1 => QuestionScreenResult::Proceeded(QuestionAnswerInput::String("Homer".to_string())),
                    2 => QuestionScreenResult::Proceeded(QuestionAnswerInput::String("1956-03-12".to_string())),
                    5 => QuestionScreenResult::Proceeded(QuestionAnswerInput::String("Springfield Nuclear Power Plant".to_string())),
                    6 => QuestionScreenResult::Proceeded(QuestionAnswerInput::String("Nuclear safety inspector".to_string())),
                    7 => QuestionScreenResult::Proceeded(QuestionAnswerInput::String("1986".to_string())),
                    _ => panic!("unexpected question screen: step={}", self.current_step)
                };
                self.current_step += 1;
                Ok(ret)
            }
        }
    
     
        let ui = UiMock::default();
        let questionaire = test_helper::build_complex_questionaire();
    
        let mut c: QController<UiMock> = QController::new(questionaire, ui);
        match c.run() {
            Ok(r) => {
                match r {
                    QuestionaireResult::Finished(_ba) => {
                        panic!("received cancel from a valid questionaire flow");
                    },
                    QuestionaireResult::Canceled => {
                        println!("Questionaire was canceled");
                    }
                }
            },
            Err(_) => panic!("received Err as questionaire result"),
        }
        
    }

}
