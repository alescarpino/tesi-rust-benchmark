#!/usr/bin/env bash
# Lancia N ripetizioni di test1_crud per ogni linguaggio, allo stesso VU count,
# per ottenere un campione statisticamente valido (stessa metodologia di Test 2).
# Compatibile con bash 3.2 (quella di serie su macOS): niente array associativi.
set -euo pipefail

VUS=20
REPS=10
TEST=test1_crud

LANGS="rust java node"

container_for() {
  case "$1" in
    rust) echo "tesi-benchmark-rust-bench-1" ;;
    java) echo "tesi-benchmark-java-bench-1" ;;
    node) echo "tesi-benchmark-node-bench-1" ;;
  esac
}

port_for() {
  case "$1" in
    rust) echo 3000 ;;
    java) echo 3001 ;;
    node) echo 3002 ;;
  esac
}

for lang in $LANGS; do
  container=$(container_for "$lang")
  port=$(port_for "$lang")
  for i in $(seq 1 "$REPS"); do
    echo "=== $lang run $i/$REPS (vus=$VUS) ==="
    python3 run_benchmark.py \
      --language "$lang" \
      --container "$container" \
      --port "$port" \
      --vus "$VUS" \
      --test "$TEST"
  done
done

echo "Fatto: $REPS run x 3 linguaggi salvati in results/raw/${TEST}.csv"
