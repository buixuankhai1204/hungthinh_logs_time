-- Your SQL goes here
CREATE TABLE player_team (
                             player_id INT NOT NULL,
                             team_id INT NOT NULL,
                             start_date DATE,
                             end_date DATE,
                             PRIMARY KEY (player_id, team_id),
                             FOREIGN KEY (player_id) REFERENCES players(id),
                             FOREIGN KEY (team_id) REFERENCES teams(id)
);