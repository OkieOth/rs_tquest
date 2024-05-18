mod ui;

mod questionaire;

mod controller;

use controller::{QController, QuestionaireResult};
use anyhow::Result;

pub use questionaire::{Questionaire, QuestionaireBuilder, QuestionaireEntry, QuestionEntry, 
    SubBlock, EntryType, StringEntry, IntEntry, FloatEntry, BoolEntry, 
    OptionEntry};

use ui::QuestionaireView;
pub use ui::Ui;

pub fn run_questionaire(title: &str, questionaire: Questionaire) -> Result<QuestionaireResult> {
    let mut ui: Ui = Ui::new()?;
    if title.len() > 0 {
        ui.print_title(title);
    }
    let mut c: QController<Ui> = QController::new(questionaire, ui);
    c.run()
}


#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
    }
}
