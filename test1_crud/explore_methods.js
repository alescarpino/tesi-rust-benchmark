// Script esplorativo, non ufficiale: confronta il costo di GET/POST/DELETE.
// Non fa parte della pipeline del Test 1 (quella resta solo GET).
import http from 'k6/http';
import { check, sleep } from 'k6';

export const options = {
  vus: 10,
  duration: '15s',
  summaryTrendStats: ['avg', 'min', 'med', 'max', 'p(90)', 'p(95)', 'p(99)'],
};

const BASE_URL = __ENV.BASE_URL || 'http://localhost:3000';
const METHOD = __ENV.METHOD || 'GET';

export default function () {
  const id = Math.floor(Math.random() * 10000) + 1;
  let res;

  if (METHOD === 'GET') {
    res = http.get(`${BASE_URL}/items/${id}`);
  } else if (METHOD === 'POST') {
    const payload = JSON.stringify({ name: `probe-${Date.now()}`, price: 9.99 });
    res = http.post(`${BASE_URL}/items`, payload, { headers: { 'Content-Type': 'application/json' } });
  } else if (METHOD === 'DELETE') {
    res = http.del(`${BASE_URL}/items/${id}`);
  }

  check(res, { 'nessun errore server': (r) => r.status < 500 });
  sleep(1);
}
