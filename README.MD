# WIK-DPS-TP01

## Description

Une API minimale en Rust qui retourne les headers de la requête HTTP GET `/ping`.

## Prérequis

- [Rust & Cargo](https://www.rust-lang.org/tools/install)

## Installation

```bash
git clone <repo>
cd wik-dps-tp01
cargo build
```

## Lancement

Par défaut le serveur écoute sur le port `8080`.

```bash
cargo run
```

ou avec un port personnalisé

```bash
PING_LISTEN_PORT=3000 cargo run
```

# Tests

Avec un navigateur :

```bash	
http://localhost:8080/ping
```

Avec curl :

```bash
curl -X GET http://localhost:8080/ping
```

## dépendances

- [actix-web](https://actix.rs/)
- [dotenv](https://crates.io/crates/dotenv)
- [serde](https://crates.io/crates/serde)
- [serde_json](https://crates.io/crates/serde_json)

