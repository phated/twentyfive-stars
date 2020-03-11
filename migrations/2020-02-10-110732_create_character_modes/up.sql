CREATE TABLE character_modes (
  id SERIAL PRIMARY KEY,
  external_id UUID UNIQUE NOT NULL DEFAULT gen_random_uuid(),
  card_id INTEGER NOT NULL REFERENCES cards (id),
  title VARCHAR NOT NULL,
  subtitle VARCHAR,
  faction FACTION NOT NULL,
  traits CHARACTER_TRAIT[] NOT NULL DEFAULT '{}',
  type MODE_TYPE NOT NULL,
  stars INT NOT NULL CHECK (stars >= 0),
  health INT,
  attack INT,
  defense INT,
  attack_modifier INT,
  defense_modifier INT
);
