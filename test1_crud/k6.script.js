import http from 'k6/http';
import { check, sleep } from 'k6';

export const options = {
  vus: 999,
  duration: '30s',
  summaryTrendStats: ['avg', 'min', 'med', 'max', 'p(90)', 'p(95)', 'p(99)'],
};

export default function () {
  const id = Math.floor(Math.random() * 10000) + 1;
  const res = http.get(`http://localhost:3000/items/${id}`);

  check(res, {
    'status is 200': (r) => r.status === 200,
  });

  sleep(1);
}