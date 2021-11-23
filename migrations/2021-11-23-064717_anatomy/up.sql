-- Your SQL goes here
CREATE TABLE questions(
      id SERIAL PRIMARY KEY,
      label VARCHAR NOT NULL,
      option_one VARCHAR NOT NULL,
      option_two VARCHAR NOT NULL,
      option_three VARCHAR NOT NULL,
      option_four VARCHAR NOT NULL,
      correct_answers VARCHAR NOT NULL
);