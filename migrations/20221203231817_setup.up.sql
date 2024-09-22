-- Add up migration script here

CREATE TABLE IF NOT EXISTS questions (
    question_uuid INTEGER PRIMARY KEY DEFAULT (ABS(RANDOM())),
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS answers (
    answer_uuid INTEGER PRIMARY KEY DEFAULT (ABS(RANDOM())),
    question_uuid INTEGER not null,
    content TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(question_uuid) REFERENCES questions(question_uuid)
);