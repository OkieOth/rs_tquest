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
        match run_sub_block(&mut self.view, &self.questionaire.init_block, true)? {
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
                    match run_sub_block(view, b, false)? {
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
                ProceedScreenResult::Canceled => {
                    //return Ok(ControllerResult::Canceled)
                    break;
                },
                ProceedScreenResult::Proceeded(b) => {
                    if ! b {
                        return Ok(ControllerResult::Canceled);
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
    sub_block: &SubBlock, init: bool) -> Result<ControllerResult> {
    //self.view.show_proceed_screen("dummy id", "dummy query", "dummy help");
    match view.show_proceed_screen(
        &sub_block.id,
        &sub_block.start_text,
        sub_block.help_text.as_deref(),
    )? {
        ProceedScreenResult::Canceled => {
            return Ok(ControllerResult::Canceled);
        },
        ProceedScreenResult::Proceeded(b) => {
            if b {
                return enter_sub_block(view, sub_block);
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
    use crate::questionaire::{QuestionEntry, EntryType, StringEntry, OptionEntry, QuestionAnswerInput};

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

            fn show_proceed_screen<'a, T: Into<Option<&'a str>>>(&mut self, _id: &str, text: &str, _help_text: T) -> Result<ProceedScreenResult> {
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
            fn show_proceed_screen<'a, T: Into<Option<&'a str>>>(&mut self, _id: &str, text: &str, _help_text: T) -> Result<ProceedScreenResult> {
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
            fn show_question_screen(&mut self, question_entry: &QuestionEntry) -> Result<QuestionScreenResult>{
                if (self.current_step != 1) && (self.current_step != 2) {
                    panic!("unexpected question screen: {}: {}", self.current_step, question_entry.query_text);
                } 
                println!("normal question: {}", question_entry.query_text);
                self.current_step += 1; 
                Ok(QuestionScreenResult::Proceeded(QuestionAnswerInput::String(format!("step: {}", self.current_step))))
            }
        }
    

        let ui = UiMock2::default();
        let questionaire = Questionaire::builder()
            .id("id00")
            .start_text("In the following questionaire you will be asked about your family and things. Do you want to proceed?")
            .end_text("All data are collected. Do you want to process them?")
            .title("Dummy title")
            .questions(vec![
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
            ])
            .build();
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


    fn build_complex_questionaire() -> Questionaire {
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
                        .build().unwrap()
                    ))
                    .build().unwrap(),
                ),
                QuestionaireEntry::Question (
                    QuestionEntry::builder()
                    .id(&format!("{}_02", id_pre))
                    .query_text("What's his date of birth?")
                    .help_text("Provide the date of birth in YYYY-MM-DD format".to_string())
                    .entry_type(EntryType::String(
                        StringEntry::builder()
                        .reqexp("\\d\\d\\d\\d-\\d\\d-\\d\\d".to_string())
                        .build().unwrap()
                    ))
                    .build().unwrap()
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
                        .build().unwrap()
                    ))
                    .build().unwrap(),
                ),
                QuestionaireEntry::Question (
                    QuestionEntry::builder()
                    .id(&format!("{}_02", id_pre))
                    .query_text("What's her date of birth?")
                    .help_text("Provide the date of birth in YYYY-MM-DD format".to_string())
                    .entry_type(EntryType::String(
                        StringEntry::builder()
                        .reqexp("\\d\\d\\d\\d-\\d\\d-\\d\\d".to_string())
                        .build().unwrap()
                    ))
                    .build().unwrap()
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
                        .build().unwrap()
                    ))
                    .build().unwrap()
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
                        .build().unwrap()
                    ))
                    .build().unwrap()
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
                        .build().unwrap()
                    ))
                    .build().unwrap()
                ),
                QuestionaireEntry::Question(
                    QuestionEntry::builder()
                    .id(&format!("{}_02", id_pre))
                    .query_text("What was your job title?")
                    .entry_type(EntryType::String(
                        StringEntry::builder()
                        .min_length(2)
                        .max_length(100)
                        .build().unwrap()
                    ))
                    .build().unwrap()
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
                        .build().unwrap()
                    ))
                    .build().unwrap()
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
                        .build().unwrap()
                    ))
                    .build().unwrap(),
                ),
                QuestionaireEntry::Question (
                    QuestionEntry::builder()
                    .id("id02")
                    .query_text("What's your date of birth?")
                    .help_text("Provide the date of birth in YYYY-MM-DD format".to_string())
                    .entry_type(EntryType::String(
                        StringEntry::builder()
                        .reqexp("\\d\\d\\d\\d-\\d\\d-\\d\\d".to_string())
                        .build().unwrap()
                    ))
                    .build().unwrap()
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

    #[test]
    fn it_travers_02() {


        #[derive(Default)]
        struct UiMock {
            current_step: usize,
        }
    
        impl QuestionaireView for UiMock {
            fn print_title<'a>(&mut self, _title: &str) {
            }

            fn show_proceed_screen<'a, T: Into<Option<&'a str>>>(&mut self, _id: &str, text: &str, _help_text: T) -> Result<ProceedScreenResult> {
                let ret = match self.current_step {
                    0 => ProceedScreenResult::Proceeded(true),
                    3 => ProceedScreenResult::Proceeded(false),
                    _ => panic!("unexpected proceed screen: step={}", self.current_step)
                };
                self.current_step += 1;
                Ok(ret)
            }
            fn show_question_screen(&mut self, question_entry: &QuestionEntry) -> Result<QuestionScreenResult>{
                let ret = match self.current_step {
                    1 => QuestionScreenResult::Proceeded(QuestionAnswerInput::String("Homer".to_string())),
                    2 => QuestionScreenResult::Proceeded(QuestionAnswerInput::String("1956-03-12".to_string())),
                    _ => panic!("unexpected question screen: step={}", self.current_step)
                };
                self.current_step += 1;
                Ok(ret)
            }
        }
    
     
        let ui = UiMock::default();
        let questionaire = build_complex_questionaire();
    
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

}
