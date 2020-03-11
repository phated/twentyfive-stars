CREATE VIEW cards_with_pageinfo AS
SELECT
  cards.*,
  lag(true, 1, false) OVER () AS has_previous,
  lead(true, 1, false) OVER () AS has_next
FROM cards;

CREATE VIEW battle_cards_with_pageinfo AS
SELECT
  cards.*,
  lag(true, 1, false) OVER () AS has_previous,
  lead(true, 1, false) OVER () AS has_next
FROM cards
WHERE category = 'BATTLE';

CREATE VIEW character_cards_with_pageinfo AS
SELECT
  cards.*,
  lag(true, 1, false) OVER () AS has_previous,
  lead(true, 1, false) OVER () AS has_next
FROM cards
WHERE category = 'CHARACTER';

CREATE VIEW stratagem_cards_with_pageinfo AS
SELECT
  cards.*,
  lag(true, 1, false) OVER () AS has_previous,
  lead(true, 1, false) OVER () AS has_next
FROM cards
WHERE category = 'STRATAGEM';
