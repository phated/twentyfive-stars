CREATE TABLE nodes (
  id SERIAL PRIMARY KEY,
  -- This is our external ID
  node_id UUID UNIQUE DEFAULT gen_random_uuid(),
  -- This allows us to lookup the table
  node_type NODE_TYPE NOT NULL
);
