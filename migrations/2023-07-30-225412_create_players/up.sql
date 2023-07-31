-- Your SQL goes here
CREATE TABLE players_tb (
  id BIGINT GENERATED ALWAYS AS IDENTITY,
  name VARCHAR NOT NULL,
  victories BIGINT NOT NULL DEFAULT 0,
  team_entity_id BIGINT NOT NULL,
  PRIMARY KEY(id),
  CONSTRAINT fk_team FOREIGN KEY(team_entity_id) REFERENCES teams_tb(id)
);