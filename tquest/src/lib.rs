mod ui;

mod questionaire;

pub use questionaire::{Questionaire, QuestionaireBuilder, StringEntry, IntEntry, FloatEntry, BoolEntry, 
    OptionEntry};

pub use ui::Ui;


#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
    }
}
