CREATE TABLE stratagem_cards (
  id SERIAL PRIMARY KEY,
  external_id UUID UNIQUE NOT NULL DEFAULT gen_random_uuid(),
  card_id INTEGER NOT NULL REFERENCES cards (id),
  title VARCHAR NOT NULL,
  requirement VARCHAR NOT NULL,
  faction FACTION,
  stars INT NOT NULL CHECK (stars >= 0)
);
