CREATE TABLE battle_cards (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  card_id UUID NOT NULL REFERENCES cards (id),
  title VARCHAR NOT NULL,
  type BATTLE_TYPE NOT NULL,
  stars SMALLINT CHECK (stars >= 0),
  icons BATTLE_ICON[],
  attack_modifier SMALLINT,
  defense_modifier SMALLINT
);
