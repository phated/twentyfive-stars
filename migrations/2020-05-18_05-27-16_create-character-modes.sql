CREATE TABLE character_modes (
  id INT REFERENCES nodes (id),
  character_id INT NOT NULL REFERENCES character_cards (id),
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
  defense_modifier INT,

  image_id INT REFERENCES images (id),

  PRIMARY KEY (id)
);
