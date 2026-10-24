# Quality gates

Ce qui peut être automatisé.

## CI/CD

- Chaque push lance les tests.
- Une branche dont les tests échouent n'est pas fusionnée dans `main`.
- Pas de déploiement automatique pour l'instant.

## Rust

`./ci.sh` est la seule source des vérifications, dans cet ordre (du plus rapide au plus lent) :

- `cargo fmt --check` : le code est formaté.
- `cargo clippy --locked --all-targets -- -D warnings` : aucun avertissement.
- `cargo test --locked` : les tests passent.

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

## Commits

Format [Conventional Commits](https://www.conventionalcommits.org/fr/) minimal :

```
type: description
```

- `type` : `feat`, `fix`, `refactor`, `test`, `docs` ou `chore`.
- `description` : en français, à l'impératif ou au présent, sans majuscule ni point final.

Exemple : `feat: enregistre chaque état du texte saisi`
