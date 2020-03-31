CREATE TABLE waves (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  tcg_id VARCHAR UNIQUE NOT NULL,
  name VARCHAR UNIQUE NOT NULL,
  released DATE NOT NULL,
  sort_order SERIAL
);
