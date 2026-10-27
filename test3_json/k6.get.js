import http from 'k6/http';
import { check, sleep } from 'k6';

// Test 3 (serializzazione): GET che torna fino a 1000 contract_rows con branch e item annidati

export const options = {
  vus: 999, 
  duration: '30s',
  summaryTrendStats: ['avg', 'min', 'med', 'max', 'p(90)', 'p(95)', 'p(99)'],
};

const BASE_URL = __ENV.BASE_URL || 'http://localhost:3000';

export default function () {
  const res = http.get(`${BASE_URL}/reports/contract-rows-detailed`);
  check(res, { 'status is 200': (r) => r.status === 200 });
  sleep(1);
}
