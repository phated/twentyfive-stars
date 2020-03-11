CREATE TABLE cards (
  id SERIAL PRIMARY KEY,
  external_id UUID UNIQUE NOT NULL DEFAULT gen_random_uuid(),
  tcg_id VARCHAR NOT NULL,
  rarity CARD_RARITY NOT NULL,
  number VARCHAR NOT NULL,
  category CARD_CATEGORY NOT NULL,
  wave_id INTEGER NOT NULL REFERENCES waves (id),
  UNIQUE (tcg_id, wave_id)
);
