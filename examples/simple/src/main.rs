use tquest::{Questionaire, QuestionaireBuilder, StringEntry};

fn question_01(builder: &mut QuestionaireBuilder, level: u8, id: &str) {
    builder.add_string_question(level, id, "what's your name?", None, None);
}

fn question_02(builder: &mut QuestionaireBuilder, level: u8, id: &str) {
    builder.add_string_question(level, 
        id,
        "What's your date of birth?", 
        Some("Provide the date of birth in YYYY-MM-DD format"), 
    Some(StringEntry::builder()
                    .reqexp("\\d\\d\\d\\d-\\d\\d-\\d\\d".to_string())
                    .build().unwrap()));
}


fn build_questionaire() -> Questionaire {
    let mut builder = Questionaire::builder();
    question_01(&mut builder, 0, "id1");
    question_02(&mut builder, 0, "id2");
    builder.build()



    // .add_string_question("2", "What's your date of birth?", None, 0)
    // .add_proceed_question("3","Do you have brothers or sisters?", "Do you have more brothers and sisters?", 0)
    //     .add_proceed_question("4", "Do you have a brother?", "Do you have another brother?", 1)
    //         .add_string_question("5","What's the name of your brother?", 2)
    //         .add_string_question("6","What's the date of birth of your brother?", 2)
    //     .add_proceed_question("7","Do you have a sister?", "Do you have another Sister?", 1)
    //         .add_string_question("8","What's the name of your sister?", 2)
    //         .add_string_question("9","What's the date of birth of your sister?", 2)
    // .add_proceed_question("10","Have you worked in a job?", "Have you worked in another job?", 0)
    //     .add_string_question("11","For what company have you worked?", 1)
    //     .add_string_question("12","What was your job title?", 1)
    //     .add_string_question("13","What was your start date?", 1)
    //     .add_string_question("14","What was your end date?", 1)
    // .add_proceed_question("15","Should we store the answers?", "", 0)
    // .build()

}

fn main() {
    let mut questionaire = build_questionaire();

    let results =questionaire.run();

    //generate_json_and_send_to_eiko(results);
    println!("Results: {:?}", results);
}
