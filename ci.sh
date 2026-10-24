#!/usr/bin/env bash
# Seule source des vérifications : lancé en local, par le hook Claude Code avant un push, et par la CI GitHub.
set -euo pipefail

cd "$(dirname "$0")/moteur"

cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
