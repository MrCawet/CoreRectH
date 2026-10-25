#!/usr/bin/env bash
# Seule source des vérifications : lancé en local, par le hook Claude Code avant un push, et par la CI GitHub.
set -euo pipefail

racine="$(dirname "$0")"

# Le moteur, puis la crate Rust du clavier Android qui l'appelle par JNI.
for crate in moteur clavier-android/lib/native/src/main/rust; do
    (
        cd "$racine/$crate"
        cargo fmt --check
        cargo clippy --locked --all-targets -- -D warnings
        cargo test --locked
    )
done
