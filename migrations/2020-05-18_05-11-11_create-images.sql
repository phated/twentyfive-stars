CREATE TABLE images (
  id INT REFERENCES nodes (id),
  original_url VARCHAR NOT NULL,

  PRIMARY KEY (id)
);
