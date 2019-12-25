CREATE TABLE cards (
  id VARCHAR PRIMARY KEY,
  wave_id VARCHAR NOT NULL REFERENCES waves (id),
  title VARCHAR NOT NULL,
  UNIQUE (id, wave_id)
)
