table! {
    all_questions (id) {
        id -> Int4,
        label -> Varchar,
        option_one -> Varchar,
        option_two -> Varchar,
        option_three -> Varchar,
        option_four -> Varchar,
        correct_answer -> Varchar,
    }
}

table! {
    questions (id) {
        id -> Int4,
        label -> Varchar,
        option_one -> Varchar,
        option_two -> Varchar,
        option_three -> Varchar,
        option_four -> Varchar,
        correct_answer -> Varchar,
        kind -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    all_questions,
    questions,
);
