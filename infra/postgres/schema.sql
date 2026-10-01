CREATE TABLE items (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    price NUMERIC NOT NULL
);

CREATE TABLE branches (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL
);

CREATE TABLE customers (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL
);

CREATE TABLE contracts (
  id SERIAL PRIMARY KEY,
  customer_id INT NOT NULL REFERENCES customers(id),
  branch_id INT NOT NULL REFERENCES branches(id),
  status TEXT NOT NULL,          -- es. 'approved' | 'ongoing' | 'closed'
  start_date DATE NOT NULL,
  end_date DATE
);

CREATE TABLE contract_rows (
  id SERIAL PRIMARY KEY,
  contract_id INT NOT NULL REFERENCES contracts(id),
  item_id INT NOT NULL REFERENCES items(id),
  quantity INT NOT NULL,
  price NUMERIC NOT NULL
);