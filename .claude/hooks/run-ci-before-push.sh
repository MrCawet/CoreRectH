#!/usr/bin/env bash
# Lancé par Claude Code avant chaque `git push` : un push n'a lieu que si ./ci.sh passe.
project_directory="$(cd "$(dirname "$0")/../.." && pwd)"

if ! ci_output="$("$project_directory/ci.sh" 2>&1)"; then
  echo "Push bloqué : ./ci.sh échoue. Corriger avant de pousser." >&2
  echo "$ci_output" >&2
  exit 2
fi
