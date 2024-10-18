-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE teams
(
    id        SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    size INTEGER NOT NULL,
    current_size       INT NOT NULL
)