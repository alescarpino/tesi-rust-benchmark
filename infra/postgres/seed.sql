INSERT INTO items (name, price)
SELECT 'item ' || gs, round((random() * 1000)::numeric, 2)
FROM generate_series(1, 10000) AS gs;
