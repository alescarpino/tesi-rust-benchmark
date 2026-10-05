INSERT INTO items (name, price)
SELECT 'item ' || gs, round((random() * 1000)::numeric, 2)
FROM generate_series(1, 10000) AS gs;

-- 10 filiali
INSERT INTO branches (name)
SELECT 'Filiale ' || g
FROM generate_series(1, 10) AS g;

-- 500 clienti
INSERT INTO customers (name)
SELECT 'Cliente ' || g
FROM generate_series(1, 500) AS g;

-- 2000 contratti, ognuno assegnato a un cliente e una filiale casuali
INSERT INTO contracts (customer_id, branch_id, status, start_date, end_date)
SELECT
    (floor(random() * 500) + 1)::int,
    (floor(random() * 10) + 1)::int,
    (ARRAY['approved', 'ongoing', 'closed'])[floor(random() * 3) + 1],
    DATE '2025-01-01' + (floor(random() * 365))::int,
    NULL
FROM generate_series(1, 2000) AS g;

-- ogni contratto ha 3 righe, ciascuna su un item casuale
INSERT INTO contract_rows (contract_id, item_id, quantity, price)
SELECT
    c.id,
    (floor(random() * 10000) + 1)::int,
    (floor(random() * 5) + 1)::int,
    round((random() * 500)::numeric, 2)
FROM contracts c
CROSS JOIN generate_series(1, 3) AS g;
