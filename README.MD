# WIK-DPS-TP01

## Description

Une API minimale en **Rust** et **TypeScript** qui retourne les headers de la requête HTTP GET `/ping`.

## Prérequis

### Pour la version Rust

- [Rust & Cargo](https://www.rust-lang.org/tools/install)

### Pour la version TypeScript

- [Node.js](https://nodejs.org/)
- [npm](https://www.npmjs.com/)

## Installation

### Version Rust

```bash
git clone https://github.com/uliaxe/wik-dps-tp01
cd wik-dps-tp01
cargo build
```

### Version TypeScript

```bash
git clone <repo>
cd wik-dps-tp01
npm install
```

## Démarrage

### Version Rust

par défaut, le serveur écoute sur le port `8080`

```bash
cargo run
```

ou avec un port personnalisée

```bash
PING_LISTEN_PORT=3000 cargo run
```

### Version TypeScript

par défaut, le serveur écoute sur le port `8080`

```bash
npx ts-node src/server.ts
```

ou avec un port personnalisée

```bash
PING_LISTEN_PORT=3000 npx ts-node src/server.ts
```

## TESTS

Avec un navigateur :

```url
http://localhost:8080/ping
```

avec CURL :

```bash
curl -X GET http://localhost:8080/ping
```

## dépendances

- Rust
  - [actix-web](https://actix.rs/)
  - [dotenv](https://crates.io/crates/dotenv)
  - [serde](https://crates.io/crates/serde)
  - [serde_json](https://crates.io/crates/serde_json)

- TypeScript
  - [dotenv](https://www.npmjs.com/package/dotenv)
  - [express](https://www.npmjs.com/package/express)
  - [ts-node](https://www.npmjs.com/package/ts-node)
  - [typescript](https://www.npmjs.com/package/typescript)
  