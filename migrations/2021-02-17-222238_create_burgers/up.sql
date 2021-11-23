-- Your SQL goes here
CREATE TABLE all_questions(
    id SERIAL PRIMARY KEY,
    label VARCHAR NOT NULL,
    option_one VARCHAR NOT NULL,
    option_two VARCHAR NOT NULL,
    option_three VARCHAR NOT NULL,
    option_four VARCHAR NOT NULL,
    correct_answer VARCHAR NOT NULL
);