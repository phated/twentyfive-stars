CREATE TABLE stratagem_cards (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  card_id UUID NOT NULL REFERENCES cards (id),
  title VARCHAR NOT NULL,
  requirement VARCHAR NOT NULL,
  faction FACTION,
  stars INT NOT NULL CHECK (stars >= 0),
  sort_order SERIAL
);
