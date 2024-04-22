use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct StringEntry {
    pub reqexp: Option<String>,
    pub max_length: Option<usize>,
}

#[derive(Debug)]
pub struct IntEntry {
    pub max: Option<u32>,
    pub min: Option<u32>,
}

#[derive(Debug)]
pub struct FloatEntry {
    pub max: Option<f32>,
    pub min: Option<f32>,
}

#[derive(Debug)]
pub struct BoolEntry {
    pub input: Option<bool>,
}

#[derive(Debug)]
pub struct ProceedEntry {
    pub input: Option<bool>,
}

#[derive(Debug)]
pub struct OptionEntry {
    pub options: Vec<String>,
}

#[derive(Debug)]
pub enum EntryType {
    String(StringEntry),
    Int(IntEntry),
    Float(FloatEntry),
    Bool(BoolEntry),
    Option(OptionEntry),
    ProceedQuery(ProceedEntry),
}

#[derive(Debug)]
pub struct QuestionEntry {
    pub query_text: String,
    pub entry_type: EntryType,
    pub id: String,
    pub prev: Option<Rc<RefCell<QuestionEntry>>>,
    pub next: Option<Rc<RefCell<QuestionEntry>>>,
}

#[derive(Debug, Default)]
pub struct Questionaire {
    pub questions: Vec<Rc<RefCell<QuestionEntry>>>,
}

impl Questionaire {
    pub fn builder() -> QuestionaireBuilder {
        QuestionaireBuilder::default()
    }
    pub fn run(&mut self) -> Option<Vec<QuestionAnswer>>{
        // TODO
        println!("Run the questionaire");
        None // TODO
    }
}

#[derive(Debug, Default)]
pub struct QuestionaireBuilder {
    pub questions: Vec<QuestionEntry>,
}

impl QuestionaireBuilder {
    pub fn add_boolean_question(&mut self, id: &str, query_text: &str, level: u8) -> &mut Self {
        self
    }
    
    pub fn add_string_question(&mut self, id: &str, query_text: &str, level: u8) -> &mut Self {
        self
    }

    pub fn add_int_question(&mut self, id: &str, query_text: &str, level: u8) -> &mut Self {
        self
    }

    pub fn add_float_question(&mut self, id: &str, query_text: &str, level: u8) -> &mut Self {
        self
    }

    pub fn add_option_question(&mut self, id: &str, query_text: &str, level: u8) -> &mut Self {
        self
    }

    pub fn add_proceed_question(&mut self, id: &str, first_query_text: &str, additional_query_text: &str, level: u8) -> &mut Self {
        self
    }


    pub fn build(&self) -> Questionaire {
        // TODO
        Questionaire::default()
    }
}


#[derive(Debug)]
pub struct QuestionAnswer {
    pub id: String,
    pub level: u8,
    pub answer: EntryInput,
}


#[derive(Debug)]
pub enum EntryInput {
    String(String),
    Int(i32),
    Float(f32),
    Bool(bool),
    Option(String),
    ProceedQuery(bool),
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
