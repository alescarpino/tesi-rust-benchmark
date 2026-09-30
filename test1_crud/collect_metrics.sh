CONTAINER="tesi-benchmark-rust-bench-1"
OUTPUT="metrics.csv"

echo "timestamp,cpu_pct,mem_usage" > "$OUTPUT"

while true; do
  TS=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
  STATS=$(docker stats "$CONTAINER" --no-stream --format "{{.CPUPerc}},{{.MemUsage}}")
  echo "${TS},${STATS}" >> "$OUTPUT"
  
done