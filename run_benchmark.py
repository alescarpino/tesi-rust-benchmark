#!/usr/bin/env python3
"""Orchestra un run completo: metriche + k6, e accoda una riga aggregata al CSV dei risultati."""

import subprocess
import time
import json
import csv
import re
import os
from datetime import datetime, timezone

LANGUAGE = "rust"
TEST_NAME = "test1_crud"
K6_SCRIPT = "test1_crud/k6.script.js"
METRICS_SCRIPT = "test1_crud/collect_metrics.sh"
RAW_METRICS_CSV = "metrics.csv"
K6_SUMMARY_JSON = "summary.json"
RESULTS_CSV = "results/raw/test1_crud.csv"


def now_iso():
    return datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def parse_mem_mib(mem_usage: str) -> float:
    # "42.1MiB / 512MiB" -> 42.1
    used = mem_usage.split("/")[0].strip()
    match = re.match(r"([\d.]+)\s*(Mi?B|Gi?B)", used)
    if not match:
        return 0.0
    value, unit = float(match.group(1)), match.group(2)
    return value * 1024 if unit.startswith("G") else value


def next_run_id(language: str, test: str) -> int:
    if not os.path.exists(RESULTS_CSV):
        return 1
    with open(RESULTS_CSV, newline="") as f:
        rows = [r for r in csv.DictReader(f) if r["language"] == language and r["test"] == test]
    return len(rows) + 1


def main():
    if os.path.exists(RAW_METRICS_CSV):
        os.remove(RAW_METRICS_CSV)

    # 1. avvia il campionamento CPU/memoria in background
    metrics_proc = subprocess.Popen(["bash", METRICS_SCRIPT])
    time.sleep(1)  # margine perché il loop scriva almeno l'header prima che parta k6

    try:
        # 2. lancia k6 in primo piano, registrando i bordi esatti della finestra di test
        start_ts = now_iso()
        subprocess.run(
            ["k6", "run", K6_SCRIPT, f"--summary-export={K6_SUMMARY_JSON}"],
            check=True,
        )
        end_ts = now_iso()
    finally:
        # 3. ferma il campionamento SEMPRE, anche se k6 fallisce o lancia un'eccezione,
        #    altrimenti resta un processo orfano che continua a girare in background
        metrics_proc.terminate()
        metrics_proc.wait()

    # 4. legge i risultati di k6
    with open(K6_SUMMARY_JSON) as f:
        summary = json.load(f)
    m = summary["metrics"]
    avg_ms = m["http_req_duration"]["avg"]
    p90_ms = m["http_req_duration"]["p(90)"]
    p95_ms = m["http_req_duration"]["p(95)"]
    p99_ms = m["http_req_duration"].get("p(99)", "")
    req_per_s = m["http_reqs"]["rate"]
    passes = m["checks"]["passes"]
    fails = m["checks"]["fails"]
    checks_pct = 100 * passes / (passes + fails) if (passes + fails) else 0.0

    # 5. legge le metriche di sistema, filtrando SOLO la finestra temporale del test vero
    cpu_values, mem_values = [], []
    with open(RAW_METRICS_CSV, newline="") as f:
        reader = csv.reader(f)
        next(reader)  # salta l'header
        for ts, cpu_pct, mem_usage in reader:
            if start_ts <= ts <= end_ts:
                cpu_values.append(float(cpu_pct.strip("%")))
                mem_values.append(parse_mem_mib(mem_usage))

    cpu_avg = sum(cpu_values) / len(cpu_values) if cpu_values else 0.0
    mem_avg = sum(mem_values) / len(mem_values) if mem_values else 0.0

    # 6. accoda la riga al CSV dei risultati (crea file + header se non esiste ancora)
    os.makedirs(os.path.dirname(RESULTS_CSV), exist_ok=True)
    is_new_file = not os.path.exists(RESULTS_CSV)
    run_id = next_run_id(LANGUAGE, TEST_NAME)

    with open(RESULTS_CSV, "a", newline="") as f:
        writer = csv.writer(f)
        if is_new_file:
            writer.writerow([
                "language", "test", "run_id", "timestamp",
                "avg_ms", "p90_ms", "p95_ms", "p99_ms",
                "req_per_s", "checks_pct", "cpu_avg_pct", "mem_avg_mib",
                "samples_in_window",
            ])
        writer.writerow([
            LANGUAGE, TEST_NAME, run_id, start_ts,
            round(avg_ms, 3), round(p90_ms, 3), round(p95_ms, 3),
            round(p99_ms, 3) if p99_ms != "" else "",
            round(req_per_s, 3), round(checks_pct, 2),
            round(cpu_avg, 3), round(mem_avg, 3),
            len(cpu_values),
        ])

    print(f"Run #{run_id} salvato in {RESULTS_CSV}")
    print(f"  latenza: avg={avg_ms:.2f}ms p90={p90_ms:.2f}ms p95={p95_ms:.2f}ms")
    print(f"  throughput={req_per_s:.2f} req/s   checks={checks_pct:.1f}%")
    print(f"  cpu_avg={cpu_avg:.2f}%   mem_avg={mem_avg:.2f}MiB   ({len(cpu_values)} campioni nella finestra)")


if __name__ == "__main__":
    main()
