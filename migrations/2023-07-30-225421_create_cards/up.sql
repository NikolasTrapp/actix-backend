-- Your SQL goes here
CREATE TYPE Suit AS ENUM (
    'Diamonds', 'Spades', 'Hearts', 'Clubs'
);

CREATE TABLE cards_tb (
  id BIGINT GENERATED ALWAYS AS IDENTITY,
  suit Suit NOT NULL,
  card_value SMALLINT NOT NULL,
  is_manilha BOOLEAN NOT NULL,
  player_entity_id BIGINT NOT NULL,
  PRIMARY KEY(id),
  CONSTRAINT fk_player FOREIGN KEY(player_entity_id) REFERENCES players_tb(id)
);