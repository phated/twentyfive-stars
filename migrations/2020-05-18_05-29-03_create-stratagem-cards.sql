CREATE TABLE stratagem_cards (
  id INT REFERENCES nodes (id),
  -- Common card traits
  tcg_id VARCHAR NOT NULL,
  rarity CARD_RARITY NOT NULL,
  number VARCHAR NOT NULL,
  category CARD_CATEGORY NOT NULL,
  wave_id INT NOT NULL REFERENCES waves (id),
  -- Stratagem card specific traits
  title VARCHAR NOT NULL,
  requirement VARCHAR NOT NULL,
  faction FACTION,
  stars INT NOT NULL CHECK (stars >= 0),

  PRIMARY KEY (id),
  UNIQUE (tcg_id, wave_id)
);
