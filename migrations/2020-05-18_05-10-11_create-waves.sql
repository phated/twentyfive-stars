CREATE TABLE waves (
  id INT REFERENCES nodes (id),
  tcg_id VARCHAR UNIQUE NOT NULL,
  name VARCHAR UNIQUE NOT NULL,
  released DATE NOT NULL,

  PRIMARY KEY (id)
);
