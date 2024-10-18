-- Your SQL goes here
CREATE TABLE games
(
    id      SERIAL PRIMARY KEY,

    stadium VARCHAR not null DEFAULT 'Hung Thinh',
    s1      INTEGER REFERENCES teams (id),
    s2      INTEGER REFERENCES teams (id),
    date    VARCHAR    not null,
    result  VARCHAR not null default 'win',
    is_win  BOOLEAN not null default true
);