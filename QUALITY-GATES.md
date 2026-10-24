# Quality gates

Ce qui peut être automatisé.

## CI/CD

- Chaque push lance les tests.
- Une branche dont les tests échouent n'est pas fusionnée dans `main`.
- Pas de déploiement automatique pour l'instant.

## Rust

`./ci.sh` est la seule source des vérifications, dans cet ordre (du plus rapide au plus lent) :

- `cargo fmt --check` : le code est formaté.
- `cargo clippy --all-targets -- -D warnings` : aucun avertissement.
- `cargo test` : les tests passent.

Il est lancé :

- avant chaque `git push` de Claude Code, sur toutes les branches, par le hook de `.claude/settings.json` : le push est bloqué si une vérification échoue ;
- à chaque push, par GitHub Actions (`.github/workflows/ci.yml`).

La chaîne Rust est fixée par `rust-toolchain.toml` (profil minimal), la même en local et en CI.

Pour une CI rapide : un seul job, annulation des exécutions dépassées, pas d'incrémental. Pas de cache tant qu'il n'y a pas de dépendances à recompiler.

Dans le code, vérifié par le compilateur et Clippy :

- `#![forbid(unsafe_code)]` dans le cœur.
- Lints Clippy `unwrap_used` et `expect_used` refusés hors des fonctions `#[test]`, y compris dans les doublures (section `[lints]` de `Cargo.toml`, `allow-unwrap-in-tests` et `allow-expect-in-tests` dans `clippy.toml`).
- Toute modification de ces réglages passe par l'opérateur.

## Commits

Format [Conventional Commits](https://www.conventionalcommits.org/fr/) minimal :

```
type: description
```

- `type` : `feat`, `fix`, `refactor`, `test`, `docs` ou `chore`.
- `description` : en français, à l'impératif ou au présent, sans majuscule ni point final.

Exemple : `feat: enregistre chaque état du texte saisi`
