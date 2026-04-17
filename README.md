# Rust Snake Webassembly

A test app generated with GeminiCLI to test Webassembly apps in Rust

## Commandes utiles

Ce projet utilise [Trunk](https://trunkrs.dev/) comme outil de build et serveur de développement.

### Installation des prérequis

Si vous ne l'avez pas déjà fait, installez la cible WebAssembly et Trunk :

```bash
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
```

### Lancer le projet en développement

Pour compiler et lancer le serveur avec rechargement automatique (hot-reload) :

```bash
trunk serve
```

L'application sera disponible sur `http://localhost:8080`.

### Builder pour la production

Pour générer les fichiers optimisés dans le dossier `dist/` :

```bash
trunk build --release
```

### Tests

Pour lancer les tests (si présents) :

```bash
cargo test
```