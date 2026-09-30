CREATE TABLE items (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  price NUMERIC NOT NULL
);
INSERT INTO items (name, price)
SELECT 'item ' || g, (random()*100)::numeric(10,2)
FROM generate_series(1, 10000) g;
