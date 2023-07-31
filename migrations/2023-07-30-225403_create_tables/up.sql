-- Your SQL goes here
CREATE TABLE tables_tb (
  id BIGINT GENERATED ALWAYS AS IDENTITY,
  last_played BIGINT,
  PRIMARY KEY(id)
);