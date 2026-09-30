# Benchmark Rust vs Node.js — Tesi

Confronto delle performance di un endpoint REST identico (crea cliente + leggilo)
implementato in **Rust** (axum + sqlx) e **Node.js** (Express + pg), sullo stesso
database PostgreSQL e con gli stessi limiti di risorse.

## Cosa misura

- **Latenza** (tempo di risposta): media, p50, p95, p99
- **Throughput** (richieste al secondo)
- **Uso di CPU e memoria** (via `docker stats`)

## Struttura

```
tesi-benchmark/
├── db/schema.sql          → la tabella customer (identica per entrambi)
├── rust-backend/          → backend Rust (porta 8081)
├── node-backend/          → backend Node.js (porta 8082)
├── k6/test.js             → lo script di misurazione
└── docker-compose.yml     → avvia tutto con limiti CPU/RAM identici
```

## Come eseguire

### 1. Prerequisiti
- Docker Desktop avviato
- k6 installato: `brew install k6`

### 2. Avvia database + entrambi i backend
```bash
docker compose up --build
```
Aspetta che vedi i due messaggi "in ascolto su...". Lascia questo terminale aperto.

### 3. In un SECONDO terminale, misura le risorse
```bash
docker stats
```
Lascialo aperto durante i test per annotare CPU% e memoria di picco di ogni backend.

### 4. In un TERZO terminale, lancia i test

Prima Rust:
```bash
cd k6
k6 run -e PORT=8081 test.js
```

Poi Node.js:
```bash
k6 run -e PORT=8082 test.js
```

### 5. Leggi i risultati

k6 stampa un report. I valori chiave per la tesi:

- `create_latency` → latenza delle operazioni di creazione
- `read_latency` → latenza delle operazioni di lettura
- `http_req_duration` → latenza complessiva (avg, p95)
- `http_reqs` → throughput totale e req/s
- `http_req_failed` → percentuale di errori

Da `docker stats` annota il picco di CPU% e memoria (MEM USAGE) per
`rust-backend` e `node-backend`.

## Per risultati validi nella tesi

1. **Ripeti ogni test 3-5 volte** e fai la media
2. **Chiudi le altre app** durante i test (browser, ecc.)
3. **Riscalda** il backend con qualche richiesta prima di misurare sul serio
4. **Stessa macchina, stesso momento** per entrambi i linguaggi
5. Docker garantisce **limiti di risorse identici** (1 CPU, 512MB ciascuno)

## Nota metodologica per la tesi

Questo è un confronto "controllato": entrambi i backend fanno esattamente
la stessa cosa, con la stessa query, lo stesso schema e le stesse risorse.
Il backend reale di ARMS fa molto di più (validazioni, autenticazione,
permessi, eventi NATS), quindi questi numeri NON rappresentano le performance
di ARMS in produzione, ma isolano la differenza dovuta al solo linguaggio/stack.
