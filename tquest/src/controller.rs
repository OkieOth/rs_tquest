use crate::{
    persistence::QuestionairePersistence, questionaire::{AnswerEntry, BlockAnswer, QuestionAnswer, QuestionAnswerInput, Questionaire, RepeatedQuestionAnswers, RepeatedQuestionEntry, SubBlock}, ui::{MsgLevel, ProceedScreenResult, QuestionScreenResult, QuestionaireView}, QuestionEntry, QuestionaireEntry
};
use anyhow::Result;
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub enum QuestionaireResult {
    Canceled,
    Finished(BlockAnswer),
}

pub enum ControllerResult {
    Canceled,
    Finished(AnswerEntry),
}


pub struct QuestionaireController<'a, V: QuestionaireView, P: QuestionairePersistence> {
    questionaire: &'a Questionaire,
    view: V,
    persistence: P,
}

impl<'a, V: QuestionaireView, P: QuestionairePersistence> QuestionaireController<'a, V, P> {
    pub fn new(questionaire: &'a Questionaire, view: V, persistence: P) -> Self {
        Self { questionaire, view, persistence }
    }

    pub fn run(&mut self) -> Result<QuestionaireResult> {   
        let pos_count = if let Some(pc) = self.questionaire.pos_count {
            pc
        } else {
            0
        };
        match run_sub_block(&mut self.view, &mut self.persistence, &self.questionaire.init_block, true, pos_count)? {
            ControllerResult::Canceled => return Ok(QuestionaireResult::Canceled),
            ControllerResult::Finished(answers) => {
                match answers {
                    AnswerEntry::Block(ba) => {
                        return Ok(QuestionaireResult::Finished(ba));
                    },
                    _ => panic!("receive wrong result for init-block"),
                }
            },
        }
    }
}

fn enter_sub_block<V: QuestionaireView, P: QuestionairePersistence> (
    view: &mut V,
    persistence: &mut P,
    sub_block: &SubBlock,
    init: bool,
    question_count: usize
) -> Result<ControllerResult> {
    let mut block_answer: BlockAnswer = BlockAnswer {
        id: sub_block.id.clone(),
        iterations: Vec::new(),
    };
    let mut has_preferred = false;
    loop {
        let mut iteration_answers: Vec<AnswerEntry> = Vec::new();
        for e in &sub_block.entries {
            // ask the sub-queries ...
            match e {
                QuestionaireEntry::Question(q) => {
                    let preferred = get_preferred(&q.id, persistence);
                    if ! has_preferred {
                        has_preferred = true;
                    }
                    match view.show_question_screen(&q, question_count, preferred)? {
                        QuestionScreenResult::Canceled => return Ok(ControllerResult::Canceled),
                        QuestionScreenResult::Proceeded(answer) => {
                            let _ = persistence.store_question(&q , &answer);
                            let qa = QuestionAnswer {
                                id: q.id.to_string(),
                                answer: answer.clone(),
                            };
                            iteration_answers.push(AnswerEntry::Question(qa));
                        }
                    }
                }
                QuestionaireEntry::Block(b) => {
                    match run_sub_block(view, persistence, b, false, question_count)? {
                        ControllerResult::Canceled => return Ok(ControllerResult::Canceled),
                        ControllerResult::Finished(answer) => {
                            iteration_answers.push(answer);
                        }
                    }
                },
                QuestionaireEntry::RepeatedQuestion(rq) => {
                    match run_repeated_question(view, persistence,  rq, question_count)? {
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
            let mut preferred = match has_preferred_block_answer(&sub_block.id, persistence) {
                PreferredBlockAnswer::Exist => {
                    Some(true)
                },
                PreferredBlockAnswer::NextHasWrongId => {
                    Some(false)
                },
                _ => {
                    None
                },
            };
            if init {
                preferred = None;
            }

            let current: usize = if init {
                let dummy_entry = QuestionEntry::builder()
                .id("00000000")
                .build();
                let final_data = QuestionAnswerInput::String(Some("done".to_string()));
                let _ = persistence.store_question(&dummy_entry, &final_data); // this is included to show that the questionary was finished
                question_count
            } else {
                0
            };

            match view.show_proceed_screen(
                &sub_block.id,
                end_text,
                sub_block.help_text.as_deref(),
                question_count, current, preferred
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
        if ! sub_block.loop_over_entries {
            break;
        }
    }
    Ok(ControllerResult::Finished(
        AnswerEntry::Block(block_answer)
    ))
}

enum PreferredBlockAnswer {
    NoMoreAnswers,
    NextHasWrongId,
    Exist,
}

fn has_preferred_block_answer<P: QuestionairePersistence>(id: &str, persistence: &mut P) -> PreferredBlockAnswer {
    if let Some(i) = persistence.next_answer_id() {
        let pre = format!("{}_", id);
        if i.starts_with(&pre) {
            return PreferredBlockAnswer::Exist;
        } else {
            return PreferredBlockAnswer::NextHasWrongId;
        }
    }
    PreferredBlockAnswer::NoMoreAnswers
}

fn run_sub_block<V: QuestionaireView, P: QuestionairePersistence> (
    view: &mut V,
    persistence: &mut P,
    sub_block: &SubBlock,
    init: bool, 
    question_count: usize) -> Result<ControllerResult> {        

    let current = if let Some(p) = sub_block.pos {
        p
    } else {
        0
    };

    let mut preferred = match has_preferred_block_answer(&sub_block.id, persistence) {
        PreferredBlockAnswer::Exist => {
            Some(true)
        },
        PreferredBlockAnswer::NextHasWrongId => {
            Some(false)
        },
        _ => {
            None
        },
    };
    if init && persistence.next_answer_id().is_some() {
        preferred = Some(true)
    }

    match view.show_proceed_screen(
        &sub_block.id,
        &sub_block.start_text,
        sub_block.help_text.as_deref(),
        question_count,
        current,
        preferred,
    )? {
        ProceedScreenResult::Canceled => {
            return Ok(ControllerResult::Canceled);
        },
        ProceedScreenResult::Proceeded(b) => {
            if b {
                return enter_sub_block(view, persistence, sub_block, init, question_count);
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


fn run_repeated_question<V: QuestionaireView, P: QuestionairePersistence> (
    view: &mut V,
    persistence: &mut P,
    repeated_question: &RepeatedQuestionEntry, question_count: usize) -> Result<ControllerResult> {
        fn push_result<P: QuestionairePersistence>(q: &QuestionEntry, answers: &mut Vec<QuestionAnswerInput>, a: &QuestionAnswerInput, persistence: &mut P) {
            let _ = persistence.store_question(&q , a);
            answers.push(a.clone());
        }
    

        fn check_for_min_input<V: QuestionaireView, T> (repeated_question: &RepeatedQuestionEntry, loop_count: usize, view: &mut V, a: &Option<T>) -> bool {
            if (repeated_question.min_count > 0) && (loop_count <= repeated_question.min_count) && (a.is_none()) {
                let m = format!("Input is needed. Minimal number of elements ({}) isn't reached yet.", repeated_question.min_count);
                view.show_msg(&m, MsgLevel::Critical);
                false
            } else {
                true
            }
        }
    


    let mut loop_count: usize = 0;
    let mut answers: Vec<QuestionAnswerInput> = Vec::new();
    let mut has_preferred = false; // this is needed to skip in fast-forward mode
    loop {
        loop_count += 1;
        if (repeated_question.max_count>0) && (loop_count > repeated_question.max_count) {
            view.show_msg("Reached maximum number of input entries. Go on with the next topic ...", MsgLevel::Normal);
            break;
        }

        let question_txt = if loop_count == 1 {
            repeated_question.query_text.clone()
        } else {
            if let Some(t) = repeated_question.secondary_query_text.as_ref() {
                t.clone()
            } else {
                repeated_question.query_text.clone()
            }
        };
        let q = QuestionEntry::builder()
            .id(&repeated_question.id)
            .pos(repeated_question.pos)
            .query_text(&question_txt)
            .entry_type(repeated_question.entry_type.clone())
            .build();
        let mut preferred = get_preferred(&repeated_question.id, persistence);
        if preferred.is_some() {
            has_preferred = true
        } else {
            if has_preferred {
                preferred = Some(QuestionAnswerInput::String(None));
            }
        }
        match view.show_question_screen(&q, question_count, preferred)? {
            QuestionScreenResult::Canceled => return Ok(ControllerResult::Canceled),
            QuestionScreenResult::Proceeded(answer) => {
                match &answer {
                    QuestionAnswerInput::String(a) => {
                        if check_for_min_input::<V, String>(&repeated_question, loop_count, view, &a) {
                            if a.is_none() {
                                break;
                            }
                            push_result(&q, &mut answers, &answer, persistence);
                        }
                    },
                    QuestionAnswerInput::Int(a) => {
                        if check_for_min_input::<V, i32>(&repeated_question, loop_count, view, &a) {
                            if a.is_none() {
                                break;
                            }
                            push_result(&q, &mut answers, &answer, persistence);
                        }
                    },
                    QuestionAnswerInput::Float(a) => {
                        if check_for_min_input::<V, f32>(&repeated_question, loop_count, view, &a) {
                            if a.is_none() {
                                break;
                            }
                            push_result(&q, &mut answers, &answer, persistence);
                        }
                    },
                    QuestionAnswerInput::Bool(a) => {
                        if check_for_min_input::<V, bool>(&repeated_question, loop_count, view, &a) {
                            if a.is_none() {
                                break;
                            }
                            push_result(&q, &mut answers, &answer, persistence);
                        }                        
                    },
                    QuestionAnswerInput::Option(a) => {
                        if check_for_min_input::<V, String>(&repeated_question, loop_count, view, &a) {
                            if a.is_none() {
                                break;
                            }
                            push_result(&q, &mut answers, &answer, persistence);
                        }                        
                    },
                    QuestionAnswerInput::None => {
                        push_result(&q, &mut answers, &answer, persistence);
                    },
                };
            }
        }
    }
    let r = RepeatedQuestionAnswers {
        id: repeated_question.id.to_string(),
        answers,
    };
Ok(ControllerResult::Finished(
        AnswerEntry::RepeatedQuestion(r)
    ))
}

fn get_preferred<P: QuestionairePersistence>(id: &str, persistence: &mut P) -> Option<QuestionAnswerInput> {
    if let Some(i) = persistence.next_answer_id() {
        if i == id {
            if let Some(a) = persistence.next_answer() {
                Some(a.answer.clone())
            } else {
                None
            }
        } else {
            Some(QuestionAnswerInput::String(None))
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::persistence::NoPersistence;

    use crate::questionaire::{QuestionAnswerInput, QuestionEntry};
    use crate::test_helper;
    use super::*;

    fn validate_question_string_input(ae: &AnswerEntry, expected_input: &str) {
        match ae {
            AnswerEntry::Block(_) => panic!("unexpected block answer"),
            AnswerEntry::Question(qa) => {
                if let QuestionAnswerInput::String(qai) = &qa.answer {
                    assert_eq!(Some(expected_input.to_string()), qai.clone());
                } else {
                    panic!("expected StringInput, but got something different")
                }
            },
            _ => {
                panic!("expected Input, but got something different")
            }
        }
    }

    #[test]
    fn it_travers_01() {
        #[derive(Default)]
        struct UiMock {
            current_step: usize,
        }
    
        impl QuestionaireView for UiMock {
            fn show_proceed_screen<'a, T: Into<Option<&'a str>>>(&mut self, _id: &str, text: &str, _help_text: T, _question_count: usize, _current: usize, _p: Option<bool>) -> Result<ProceedScreenResult> {
                if (self.current_step != 0) && (self.current_step != 3) {
                    panic!("unexpected proceed screen: {}: {}", self.current_step, text);
                }
                println!("proceed question: {}", text);
                self.current_step += 1; 
                Ok(ProceedScreenResult::Proceeded(true))
            }
            fn show_question_screen(&mut self, question_entry: &QuestionEntry, _question_count: usize, _p: Option<QuestionAnswerInput>) -> Result<QuestionScreenResult>{
                if (self.current_step != 1) && (self.current_step != 2) {
                    panic!("unexpected question screen: {}: {}", self.current_step, question_entry.query_text);
                } 
                println!("normal question: {}", question_entry.query_text);
                self.current_step += 1; 
                Ok(QuestionScreenResult::Proceeded(QuestionAnswerInput::String(Some(format!("step: {}", self.current_step)))))
            }
        }
    
     
        let ui = UiMock::default();
        let questionaire = crate::test_helper::create_small_questionaire();
    
        let np = NoPersistence::new();
        let mut c: QuestionaireController<UiMock, NoPersistence> = QuestionaireController::new(&questionaire, ui, np);
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
    fn it_travers_w_persistence() {
        use crate::persistence::FileQuestionairePersistence;

        #[derive(Default)]
        struct UiMock {
            current_step: usize,
        }
    
        impl QuestionaireView for UiMock {
            fn show_proceed_screen<'a, T: Into<Option<&'a str>>>(&mut self, _id: &str, text: &str, _help_text: T, _question_count: usize, _current: usize, _p: Option<bool>) -> Result<ProceedScreenResult> {
                if (self.current_step != 0) && (self.current_step != 3) {
                    panic!("unexpected proceed screen: {}: {}", self.current_step, text);
                }
                println!("proceed question: {}", text);
                self.current_step += 1; 
                Ok(ProceedScreenResult::Proceeded(true))
            }
            fn show_question_screen(&mut self, question_entry: &QuestionEntry, _question_count: usize, _p: Option<QuestionAnswerInput>) -> Result<QuestionScreenResult>{
                if (self.current_step != 1) && (self.current_step != 2) {
                    panic!("unexpected question screen: {}: {}", self.current_step, question_entry.query_text);
                } 
                println!("normal question: {}", question_entry.query_text);
                self.current_step += 1; 
                Ok(QuestionScreenResult::Proceeded(QuestionAnswerInput::String(Some(format!("step: {}", self.current_step)))))
            }
        }
    
     
        let ui = UiMock::default();
        let questionaire = crate::test_helper::create_small_questionaire();
    
        let mut persistence = FileQuestionairePersistence::new("tmp/tquest.tmp").unwrap();
        persistence.load(Some("res/tquest.tmp")).expect("fail to load temp file");
        let mut c: QuestionaireController<UiMock, FileQuestionairePersistence> = QuestionaireController::new(&questionaire, ui, persistence);
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
            fn show_proceed_screen<'a, T: Into<Option<&'a str>>>(&mut self, _id: &str, text: &str, _help_text: T, _question_count: usize, _current: usize, _p: Option<bool>) -> Result<ProceedScreenResult> {
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
            fn show_question_screen(&mut self, question_entry: &QuestionEntry, _question_count: usize, _p: Option<QuestionAnswerInput>) -> Result<QuestionScreenResult>{
                if (self.current_step != 1) && (self.current_step != 2) {
                    panic!("unexpected question screen: {}: {}", self.current_step, question_entry.query_text);
                } 
                println!("normal question: {}", question_entry.query_text);
                self.current_step += 1; 
                Ok(QuestionScreenResult::Proceeded(QuestionAnswerInput::String(Some(format!("step: {}", self.current_step)))))
            }
        }
    

        let ui = UiMock2::default();
        let questionaire = test_helper::create_small_questionaire();
        let np = NoPersistence::new();
        let mut c: QuestionaireController<UiMock2, NoPersistence> = QuestionaireController::new(&questionaire, ui, np);
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
            fn show_proceed_screen<'a, T: Into<Option<&'a str>>>(&mut self, _id: &str, _text: &str, _help_text: T, _question_count: usize, _current: usize, _p: Option<bool>) -> Result<ProceedScreenResult> {
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
            fn show_question_screen(&mut self, _question_entry: &QuestionEntry, _question_count: usize, _p: Option<QuestionAnswerInput>) -> Result<QuestionScreenResult>{
                let ret = match self.current_step {
                    1 => QuestionScreenResult::Proceeded(QuestionAnswerInput::String(Some("Homer".to_string()))),
                    2 => QuestionScreenResult::Proceeded(QuestionAnswerInput::String(Some("1956-03-12".to_string()))),
                    5 => QuestionScreenResult::Proceeded(QuestionAnswerInput::String(Some("Springfield Nuclear Power Plant".to_string()))),
                    6 => QuestionScreenResult::Proceeded(QuestionAnswerInput::String(Some("Nuclear safety inspector".to_string()))),
                    7 => QuestionScreenResult::Proceeded(QuestionAnswerInput::String(Some("1986".to_string()))),
                    _ => panic!("unexpected question screen: step={}", self.current_step)
                };
                self.current_step += 1;
                Ok(ret)
            }
        }
    
     
        let ui = UiMock::default();
        let questionaire = test_helper::create_complex_questionaire();
    
        let np = NoPersistence::new();
        let mut c: QuestionaireController<UiMock, NoPersistence> = QuestionaireController::new(&questionaire, ui, np);
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
            fn show_proceed_screen<'a, T: Into<Option<&'a str>>>(&mut self, _id: &str, _text: &str, _help_text: T, _question_count: usize, _current: usize, _p: Option<bool>) -> Result<ProceedScreenResult> {
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
            fn show_question_screen(&mut self, _question_entry: &QuestionEntry, _question_count: usize, _p: Option<QuestionAnswerInput>) -> Result<QuestionScreenResult>{
                let ret = match self.current_step {
                    1 => QuestionScreenResult::Proceeded(QuestionAnswerInput::String(Some("Homer".to_string()))),
                    2 => QuestionScreenResult::Proceeded(QuestionAnswerInput::String(Some("1956-03-12".to_string()))),
                    5 => QuestionScreenResult::Proceeded(QuestionAnswerInput::String(Some("Springfield Nuclear Power Plant".to_string()))),
                    6 => QuestionScreenResult::Proceeded(QuestionAnswerInput::String(Some("Nuclear safety inspector".to_string()))),
                    7 => QuestionScreenResult::Proceeded(QuestionAnswerInput::String(Some("1986".to_string()))),
                    _ => panic!("unexpected question screen: step={}", self.current_step)
                };
                self.current_step += 1;
                Ok(ret)
            }
        }
    
     
        let ui = UiMock::default();
        let questionaire = test_helper::create_complex_questionaire();
        let np = NoPersistence::new();
        let mut c: QuestionaireController<UiMock, NoPersistence> = QuestionaireController::new(&questionaire, ui, np);
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
