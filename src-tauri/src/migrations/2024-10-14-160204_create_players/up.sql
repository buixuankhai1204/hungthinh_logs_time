-- Your SQL goes here
CREATE TABLE players
(
    id            SERIAL PRIMARY KEY ,
    team_id       INTEGER NOT NULL,
    full_name     VARCHAR NOT NULL,
    nick_name     VARCHAR NOT NULL,
    age           integer NOT NULL,
    role_position BOOLEAN NOT NULL
)