--------------------------------------------------------------------------------

CREATE TYPE public.suit AS ENUM (
	'Diamonds',
	'Spades',
	'Hearts',
	'Clubs'
);

CREATE TABLE IF NOT EXISTS public.tables_tb (
	id int8 NOT NULL GENERATED ALWAYS AS IDENTITY,
	last_played int8 NULL,
	maquina int8 NULL,
	turn int8 NOT NULL DEFAULT 0,
    round_points NOT NULL int2 DEFAULT 1,
    winner_team int8 NULL,
	CONSTRAINT tables_tb_pkey PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS teams_tb (
	id int8 NOT NULL GENERATED ALWAYS AS IDENTITY,
	score int2 NOT NULL DEFAULT 0,
	table_entity_id int8 NOT NULL,
	CONSTRAINT teams_tb_pkey PRIMARY KEY (id),
    CONSTRAINT teams_tb_fk FOREIGN KEY (table_entity_id) REFERENCES tables_tb (id)
);


CREATE TABLE IF NOT EXISTS players_tb (
	id int8 NOT NULL GENERATED ALWAYS AS IDENTITY,
	name varchar NOT NULL,
	victories int8 NOT NULL DEFAULT 0,
	team_entity_id int8 NULL,
	CONSTRAINT players_tb_pkey PRIMARY KEY (id),
    CONSTRAINT teams_tb_fk FOREIGN KEY (team_entity_id) REFERENCES teams_tb (id)
);

CREATE TABLE IF NOT EXISTS cards_tb (
	id int8 NOT NULL GENERATED ALWAYS AS IDENTITY,
	suit public.suit NOT NULL,
	card_value int2 NOT NULL,
	is_manilha bool NOT NULL,
	player_entity_id int8 NULL,
	CONSTRAINT cards_tb_pkey PRIMARY KEY (id),
	CONSTRAINT players_tb_fk FOREIGN KEY (player_entity_id) REFERENCES players_tb (id)
);

CREATE TABLE IF NOT EXISTS rounds_tb (
    table_id int8 NOT NULL,
    player_id int8 NOT NULL,
    card_id int8 NOT NULL,
    turn int8 NOT NULL,
    CONSTRAINT tables_tb_fk FOREIGN KEY (table_id) REFERENCES tables_tb (id),
    CONSTRAINT players_tb_fk FOREIGN KEY (player_id) REFERENCES players_tb (id),
    CONSTRAINT cards_tb_fk FOREIGN KEY (card_id) REFERENCES cards_tb (id)
);


--------------------------------------------------------------------------------
