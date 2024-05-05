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

fn question_03(builder: &mut QuestionaireBuilder, level: u8, id: &str) {
    builder.add_proceed_question(
        level,
        id,
        "Do you have brothers or sisters?", 
        Some("Do you have more brothers and sisters?"));
}

fn question_04(builder: &mut QuestionaireBuilder, level: u8, id: &str) {
    builder.add_proceed_question(
        level,
        id,
        "Do you have a brother?", 
        Some("Do you have another brother?"));
}

fn question_05(builder: &mut QuestionaireBuilder, level: u8, id: &str) {
    builder.add_string_question(level, id, 
        "what's the name of your brother?", None, None);
}

fn question_06(builder: &mut QuestionaireBuilder, level: u8, id: &str) {
    builder.add_string_question(level, 
        id,
        "What's his date of birth?", 
        Some("Provide the date of birth in YYYY-MM-DD format"), 
    Some(StringEntry::builder()
                    .reqexp("\\d\\d\\d\\d-\\d\\d-\\d\\d".to_string())
                    .build().unwrap()));
}

fn question_07(builder: &mut QuestionaireBuilder, level: u8, id: &str) {
    builder.add_proceed_question(
        level,
        id,
        "Do you have a sister?", 
        Some("Do you have another sister?"));
}

fn question_08(builder: &mut QuestionaireBuilder, level: u8, id: &str) {
    builder.add_string_question(level, id, 
        "what's the name of your sister?", None, None);
}

fn question_09(builder: &mut QuestionaireBuilder, level: u8, id: &str) {
    builder.add_string_question(level, 
        id,
        "What's her date of birth?", 
        Some("Provide the date of birth in YYYY-MM-DD format"), 
    Some(StringEntry::builder()
                    .reqexp("\\d\\d\\d\\d-\\d\\d-\\d\\d".to_string())
                    .build().unwrap()));
}

fn question_10(builder: &mut QuestionaireBuilder, level: u8, id: &str) {
    builder.add_proceed_question(
        level,
        id,
        "Have you already worked in a job?", 
        Some("Have you worked in another job?"));
}

fn question_11(builder: &mut QuestionaireBuilder, level: u8, id: &str) {
    builder.add_string_question(level, 
        id,
        "What was the name of the company you worked for?", 
        None, 
    Some(StringEntry::builder()
                    .min_length(2)
                    .max_length(200)
                    .build().unwrap()));
}

fn question_12(builder: &mut QuestionaireBuilder, level: u8, id: &str) {
    builder.add_string_question(level, 
        id,
        "What was your job title?", 
        None, 
    Some(StringEntry::builder()
                    .min_length(2)
                    .max_length(200)
                    .build().unwrap()));
}

fn question_13(builder: &mut QuestionaireBuilder, level: u8, id: &str) {
    builder.add_string_question(level, 
        id,
        "What was your start date there?", 
        Some("Provide the year and optional month in YYYY-MM format"), 
    Some(StringEntry::builder()
                    .min_length(0)
                    .reqexp("\\d\\d\\d\\d-\\d\\d".to_string())
                    .build().unwrap()));
}

fn question_14(builder: &mut QuestionaireBuilder, level: u8, id: &str) {
    builder.add_string_question(level, 
        id,
        "What was your end date there?", 
        Some("Provide the year and optional month in YYYY-MM format"), 
    Some(StringEntry::builder()
                    .min_length(0)
                    .reqexp("\\d\\d\\d\\d-\\d\\d".to_string())
                    .build().unwrap()));
}

fn question_15(builder: &mut QuestionaireBuilder, level: u8, id: &str) {
    builder.add_proceed_question(
        level,
        id,
        "Should your answers be stored?", 
        None);
}

fn info_txt_01(builder: &mut QuestionaireBuilder, level: u8, id: &str) {
    builder.add_info_text(
        level,
        id,
        "In the following questionaire you will be asked about your family and things?");
}

fn info_txt_02(builder: &mut QuestionaireBuilder, level: u8, id: &str) {
    builder.add_info_text(
        level,
        id,
        "Your input will be now processed and stored");
}

fn build_questionaire() -> Questionaire {
    let mut builder = Questionaire::builder();
    info_txt_01(&mut builder, 0, "id0");
    question_01(&mut builder, 0, "id1");
    question_02(&mut builder, 0, "id2");
    question_03(&mut builder, 0, "id3");
        question_04(&mut builder, 1, "id4");
            question_05(&mut builder, 2, "id5");
            question_06(&mut builder, 2, "id6");
        question_07(&mut builder, 1, "id7");
            question_08(&mut builder, 2, "id8");
            question_09(&mut builder, 2, "id9");
    question_10(&mut builder, 0, "id10");
        question_11(&mut builder, 1, "id11");
        question_12(&mut builder, 1, "id12");
        question_13(&mut builder, 1, "id13");
        question_14(&mut builder, 1, "id14");
    question_15(&mut builder, 0, "id15");
    info_txt_02(&mut builder, 0, "id16");
    builder.build()

}

fn main() {
    let mut questionaire = build_questionaire();

    let results =questionaire.run();

    //generate_json_and_send_to_eiko(results);
    println!("Results: {:?}", results);
}
