mod ui;

mod questionaire;

mod controller;

use controller::{QController, QuestionaireResult};
use anyhow::Result;

pub use questionaire::{Questionaire, QuestionaireBuilder, QuestionaireEntry, QuestionEntry, 
    SubBlock, EntryType, StringEntry, IntEntry, FloatEntry, BoolEntry, 
    OptionEntry};

pub use ui::Ui;

pub fn run_questionaire(questionaire: Questionaire) -> Result<QuestionaireResult> {
    let ui: Ui = Ui::new()?;
    let mut c: QController<Ui> = QController::new(questionaire, ui);
    c.run()
}


#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
    }
}
