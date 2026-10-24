# Quality gates

Ce qui peut être automatisé.

## CI/CD

- Chaque push lance les tests.
- Une branche dont les tests échouent n'est pas fusionnée dans `main`.
- Pas de déploiement automatique pour l'instant.

## Rust

À chaque push :

- `cargo fmt --check` : le code est formaté.
- `cargo clippy --all-targets -- -D warnings` : aucun avertissement.
- `cargo test` : les tests passent.

Dans le code, vérifié par le compilateur et Clippy :

- `#![forbid(unsafe_code)]` dans le cœur.
- Lints Clippy `unwrap_used` et `expect_used` refusés hors des tests (section `[lints]` de `Cargo.toml`, `allow-unwrap-in-tests` et `allow-expect-in-tests` dans `clippy.toml`).
- Toute modification de ces réglages passe par l'opérateur.

## Commits

Format [Conventional Commits](https://www.conventionalcommits.org/fr/) minimal :

```
type: description
```

- `type` : `feat`, `fix`, `refactor`, `test`, `docs` ou `chore`.
- `description` : en français, à l'impératif ou au présent, sans majuscule ni point final.

Exemple : `feat: enregistre chaque état du texte saisi`
