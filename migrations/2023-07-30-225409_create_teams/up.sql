-- Your SQL goes here
CREATE TABLE teams_tb (
  id BIGINT GENERATED ALWAYS AS IDENTITY,
  score SMALLINT NOT NULL DEFAULT 0,
  table_entity_id BIGINT NOT NULL,
  PRIMARY KEY(id),
  CONSTRAINT fk_tables FOREIGN KEY(table_entity_id) REFERENCES tables_tb(id)
);