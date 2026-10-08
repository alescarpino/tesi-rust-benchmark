BACKEND_CONTAINER="${1:-tesi-benchmark-rust-bench-1}"
DB_CONTAINER="${2:-tesi-benchmark-db-1}"
OUTPUT="metrics.csv"

echo "timestamp,backend_cpu_pct,backend_mem,db_cpu_pct,db_mem" > "$OUTPUT"

while true; do
  TS=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
  STATS=$(docker stats "$BACKEND_CONTAINER" "$DB_CONTAINER" --no-stream --format "{{.Name}},{{.CPUPerc}},{{.MemUsage}}")

  BACKEND_LINE=$(echo "$STATS" | grep "^${BACKEND_CONTAINER},")
  DB_LINE=$(echo "$STATS" | grep "^${DB_CONTAINER},")

  BACKEND_CPU=$(echo "$BACKEND_LINE" | cut -d',' -f2)
  BACKEND_MEM=$(echo "$BACKEND_LINE" | cut -d',' -f3)
  DB_CPU=$(echo "$DB_LINE" | cut -d',' -f2)
  DB_MEM=$(echo "$DB_LINE" | cut -d',' -f3)

  echo "${TS},${BACKEND_CPU},${BACKEND_MEM},${DB_CPU},${DB_MEM}" >> "$OUTPUT"
done
