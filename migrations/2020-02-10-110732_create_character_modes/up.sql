CREATE TABLE character_modes (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  card_id UUID NOT NULL REFERENCES cards (id),
  title VARCHAR NOT NULL,
  subtitle VARCHAR,
  traits CHARACTER_TRAIT[] NOT NULL DEFAULT '{}',
  type MODE_TYPE NOT NULL,
  stars INT NOT NULL CHECK (stars >= 0),
  health INT,
  attack INT,
  defense INT,
  attack_modifier INT,
  defense_modifier INT
);
