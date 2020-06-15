CREATE TABLE battle_cards (
  id INT REFERENCES nodes (id),
  -- Common card traits
  tcg_id VARCHAR NOT NULL,
  rarity CARD_RARITY NOT NULL,
  number VARCHAR NOT NULL,
  category CARD_CATEGORY NOT NULL,
  wave_id INT NOT NULL REFERENCES waves (id),
  -- Battle card specific traits
  title VARCHAR NOT NULL,
  type BATTLE_TYPE NOT NULL,
  faction FACTION,
  stars INT CHECK (stars >= 0),
  icons BATTLE_ICON[] NOT NULL,
  attack_modifier INT,
  defense_modifier INT,

  image_id INT REFERENCES images (id),

  PRIMARY KEY (id),
  UNIQUE (tcg_id, wave_id)
);
