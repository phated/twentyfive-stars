CREATE TABLE cards (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  card_number VARCHAR NOT NULL,
  card_type CARD_TYPE NOT NULL,
  title VARCHAR NOT NULL,
  subtitle VARCHAR,
  rarity CARD_RARITY NOT NULL,
  wave_id VARCHAR NOT NULL REFERENCES waves (id),
  UNIQUE (card_number, wave_id)
);
