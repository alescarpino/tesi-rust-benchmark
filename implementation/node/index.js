import Fastify from 'fastify';
import pg from 'pg';

const { Pool } = pg;

const pool = new Pool({
  connectionString: process.env.DATABASE_URL,
});

const fastify = Fastify();

fastify.get('/items/:id', async (request, reply) => {
  const { id } = request.params;
  const result = await pool.query('SELECT id, name, price FROM items WHERE id = $1', [id]);
  if (result.rows.length === 0) {
    return reply.code(404).send();
  }
  return result.rows[0];
});

fastify.post('/items', async (request, reply) => {
  const { name, price } = request.body;
  const result = await pool.query(
    'INSERT INTO items (name, price) VALUES ($1, $2) RETURNING id, name, price',
    [name, price]
  );
  return result.rows[0];
});

fastify.put('/items/:id', async (request, reply) => {
  const { id } = request.params;
  const { name, price } = request.body;
  await pool.query('UPDATE items SET name = $1, price = $2 WHERE id = $3', [name, price, id]);
  return reply.code(200).send();
});

fastify.delete('/items/:id', async (request, reply) => {
  const { id } = request.params;
  await pool.query('DELETE FROM items WHERE id = $1', [id]);
  return reply.code(204).send();
});

fastify.get('/reports/revenue-by-branch', async (request, reply) => {
  const result = await pool.query(`
    SELECT
        b.name AS branch_name,
        i.name AS item_name,
        SUM(cr.quantity * cr.price) AS total_revenue,
        COUNT(*) AS line_count
    FROM contract_rows cr
    JOIN contracts c ON cr.contract_id = c.id
    JOIN branches b ON c.branch_id = b.id
    JOIN items i ON cr.item_id = i.id
    WHERE c.status IN ('approved', 'ongoing')
      AND c.start_date BETWEEN '2025-01-01' AND '2025-12-31'
    GROUP BY b.name, i.name
    ORDER BY total_revenue DESC
    LIMIT 50
  `);
  return result.rows;
});

fastify.listen({ port: 8080, host: '0.0.0.0' }, (err) => {
  if (err) {
    console.error(err);
    process.exit(1);
  }
});
