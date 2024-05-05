use tquest::{Questionaire, QuestionEntry, EntryType,QuestionaireEntry, 
    StringEntry, OptionEntry, SubBlock};


fn get_brother_questions() -> Vec<QuestionaireEntry> {
    vec![
        QuestionaireEntry::Question (
            QuestionEntry::builder()
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

fn get_sister_questions() -> Vec<QuestionaireEntry> {
    vec![
        QuestionaireEntry::Question (
            QuestionEntry::builder()
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

fn get_sibling_entries() -> Vec<QuestionaireEntry> {
    vec![
        QuestionaireEntry::Block(
            SubBlock::builder()
            .start_text("Do you have a sister?")
            .end_text("Do you have another sister?".to_string())
            .entries(get_sister_questions())
            .build().unwrap()
        ),
        QuestionaireEntry::Block(
            SubBlock::builder()
            .start_text("Do you have a brother?")
            .end_text("Do you have another brother?".to_string())
            .entries(get_brother_questions())
            .build().unwrap()
        )
    ]
}

fn get_job_end_entries() -> Vec<QuestionaireEntry> {
    vec![
        QuestionaireEntry::Question(
            QuestionEntry::builder()
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

fn get_job_entries() -> Vec<QuestionaireEntry> {
    vec![
        QuestionaireEntry::Question(
            QuestionEntry::builder()
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
            .start_text("Have you finished your job there?")
            .entries(get_job_end_entries())
            .build().unwrap()
        )
    ]
}

fn build_questionaire() -> Questionaire {
    let mut builder = Questionaire::builder();
    builder.add_init_block_and_build (
        "In the following questionaire you will be asked about your family and things. Do you want to proceed?", 
        Some("All data are collected. Do you want to process them?"), 
        None, 
        Some(
            vec![
                QuestionaireEntry::Question (
                    QuestionEntry::builder()
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
                    .start_text("Do you have brothers or sisters?")
                    .end_text("Do you have more brothers and sisters?".to_string())
                    .entries(get_sibling_entries())
                    .build().unwrap()
                ),
                QuestionaireEntry::Block(
                    SubBlock::builder()
                    .start_text("Have you already worked in a job?")
                    .end_text("Have you worked in another job?".to_string())
                    .entries(get_job_entries())
                    .build().unwrap()
                )
            ]
        ))
}

fn main() {
    let mut questionaire = build_questionaire();

    let results =questionaire.run();

    //generate_json_and_send_to_eiko(results);
    println!("Results: {:?}", results);
}
