use rs_tquest::{QuestionaireBuilder, Questionaire};


fn build_questionaire() -> Questionaire {
    Questionaire::builder()
    .add_string_question("1", "What's your name?", 0)
    .add_string_question("2", "What's your date of birth?", 0)
    .add_proceed_question("3","Do you have brothers or sisters?", "Do you have more brothers and sisters?", 0)
        .add_proceed_question("4", "Do you have a brother?", "Do you have another brother?", 1)
            .add_string_question("5","What's the name of your brother?", 2)
            .add_string_question("6","What's the date of birth of your brother?", 2)
        .add_proceed_question("7","Do you have a sister?", "Do you have another Sister?", 1)
            .add_string_question("8","What's the name of your sister?", 2)
            .add_string_question("9","What's the date of birth of your sister?", 2)
    .add_proceed_question("10","Have you worked in a job?", "Have you worked in another job?", 0)
        .add_string_question("11","For what company have you worked?", 1)
        .add_string_question("12","What was your job title?", 1)
        .add_string_question("13","What was your start date?", 1)
        .add_string_question("14","What was your end date?", 1)
    .add_proceed_question("15","Should we store the answers?", "", 0)
    .build()
}

fn main() {
    let mut questionaire = build_questionaire();

    let results =questionaire.run();

    println!("Results: {:?}", results);
}
