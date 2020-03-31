CREATE TABLE cards (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  tcg_id VARCHAR NOT NULL,
  rarity CARD_RARITY NOT NULL,
  number VARCHAR NOT NULL,
  category CARD_CATEGORY NOT NULL,
  wave_id UUID NOT NULL REFERENCES waves (id),
  sort_order SERIAL,
  UNIQUE (tcg_id, wave_id)
);
