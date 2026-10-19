import http from 'k6/http';
import { check, sleep } from 'k6';

// Test 3 (deserializzazione): POST con un array di 1000 righe (come createFromPlanner in
// ARMS). Il server le deserializza e valida, senza scrivere su DB (vedi
// validate_contract_rows in main.rs) — isola il costo di parsing JSON dal costo di scrittura.
export const options = {
  vus: 999, // sovrascritto da --vus a riga di comando
  duration: '30s',
  summaryTrendStats: ['avg', 'min', 'med', 'max', 'p(90)', 'p(95)', 'p(99)'],
};

const BASE_URL = __ENV.BASE_URL || 'http://localhost:3000';

// Costruito una volta per VU (codice a livello di modulo = "init context" in k6), non ad
// ogni iterazione: vogliamo misurare il costo di parsing lato server, non la generazione
// del payload lato client.
function buildPayload(n) {
  const rows = [];
  for (let i = 0; i < n; i++) {
    rows.push({
      item_id: Math.floor(Math.random() * 10000) + 1,
      quantity: Math.floor(Math.random() * 10) + 1,
      price: (Math.random() * 1000).toFixed(2),
    });
  }
  return JSON.stringify({ rows });
}

const payload = buildPayload(1000);
const params = { headers: { 'Content-Type': 'application/json' } };

export default function () {
  const res = http.post(`${BASE_URL}/contract-rows/validate`, payload, params);
  check(res, { 'status is 200': (r) => r.status === 200 });
  sleep(1);
}
