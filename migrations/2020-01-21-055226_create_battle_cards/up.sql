CREATE TABLE battle_cards (
  id SERIAL PRIMARY KEY,
  external_id UUID UNIQUE NOT NULL DEFAULT gen_random_uuid(),
  card_id INTEGER NOT NULL REFERENCES cards (id),
  title VARCHAR NOT NULL,
  type BATTLE_TYPE NOT NULL,
  faction FACTION,
  stars INT CHECK (stars >= 0),
  icons BATTLE_ICON[] NOT NULL,
  attack_modifier INT,
  defense_modifier INT
);
