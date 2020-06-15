CREATE TABLE character_cards (
  id INT REFERENCES nodes (id),
  -- Common card traits
  tcg_id VARCHAR NOT NULL,
  rarity CARD_RARITY NOT NULL,
  number VARCHAR NOT NULL,
  category CARD_CATEGORY NOT NULL,
  wave_id INT NOT NULL REFERENCES waves (id),

  PRIMARY KEY (id),
  UNIQUE (tcg_id, wave_id)
);
