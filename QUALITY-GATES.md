# Quality gates

Ce qui peut être automatisé.

## CI/CD

- Chaque push lance les tests.
- Une branche dont les tests échouent n'est pas fusionnée dans `main`.
- Pas de déploiement automatique pour l'instant.
- L'action **Publier** (`.github/workflows/publier.yml`), lancée à la main, publie le clavier Android : `./ci.sh`, compilation de l'APK, signature, puis tag de version à la date du jour (`v2026.10.25`, puis `v2026.10.25.2`…) et release GitHub. Elle demande deux secrets : `SIGNING_KEYSTORE_BASE64` et `SIGNING_KEYSTORE_PASSWORD`.

## Rust

`./ci.sh` est la seule source des vérifications, dans cet ordre (du plus rapide au plus lent) :

- `cargo fmt --check` : le code est formaté.
- `cargo clippy --locked --all-targets -- -D warnings` : aucun avertissement.
- `cargo test --locked` : les tests passent.
- `cargo doc --locked --no-deps --document-private-items`, avec `-D warnings` : la documentation se génère sans avertissement (liens et sections `# Apprendre` compris).

`--locked` fait échouer la vérification si `Cargo.lock` ne correspond plus à `Cargo.toml`, au lieu de le mettre à jour en silence.

Il est lancé :

- avant chaque `git push` de Claude Code, sur toutes les branches, par le hook de `.claude/settings.json` : le push est bloqué si une vérification échoue ;
- à chaque push, par GitHub Actions (`.github/workflows/ci.yml`).

La chaîne Rust est fixée par `rust-toolchain.toml` (profil minimal), la même en local et en CI.

Pour une CI rapide : un seul job, annulation des exécutions dépassées, pas d'incrémental. Pas de cache tant qu'il n'y a pas de dépendances à recompiler.

Les actions GitHub sont désignées par leur version (`@v5`). Les épingler par SHA dès qu'arrive une action tierce ou un secret.

Dans le code, vérifié par le compilateur et Clippy :

- `#![forbid(unsafe_code)]` dans le cœur.
- Lints Clippy `unwrap_used` et `expect_used` refusés dans le code de la crate, permis dans `tests/` (`#![forbid(...)]` en tête de `src/lib.rs` : aucun `#[allow]` ne peut les réactiver).
- Toute modification de ces réglages passe par l'opérateur.

## Mesurer les tests, à la main

Hors de `./ci.sh` et de la CI, pour ne pas les alourdir : l'opérateur les lance de temps en temps, dans le dossier d'une crate (`moteur`, `modele-de-langue`…).

- **Couverture** (`cargo llvm-cov --text`) : les lignes que les tests exécutent. Trouve le code jamais exécuté.
- **Mutation** (`cargo mutants`) : casse le code une modification à la fois et vérifie qu'un test échoue.
  - Un mutant `missed` est du code que rien ne vérifie.
  - Pour juger un test : le retirer et relancer. Si aucun mutant ne devient `missed`, il ne protège rien de plus que les autres.
  - `--file <chemin>` limite à un fichier.
  - Documentation : <https://mutants.rs/>

Installation, depuis le dépôt (pour la version de Rust de `rust-toolchain.toml`), une fois par machine et à chaque changement de version :

```sh
cargo install --locked cargo-llvm-cov cargo-mutants
rustup component add llvm-tools
```

## Commits

Format [Conventional Commits](https://www.conventionalcommits.org/fr/) minimal :

```
type: description
```

- `type` : `feat`, `fix`, `refactor`, `test`, `docs` ou `chore`.
- `description` : en français, à l'impératif ou au présent, sans majuscule ni point final.

Exemple : `feat: enregistre chaque état du texte saisi`
