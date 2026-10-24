#!/usr/bin/env bash
# Lancé par Claude Code avant chaque commande Bash : un `git push` n'a lieu que si ./ci.sh passe.
# Le script filtre lui-même : le champ `if` de .claude/settings.json s'est déclenché sur d'autres commandes que `git push`.
project_directory="$(cd "$(dirname "$0")/../.." && pwd)"

hook_input="$(cat)"
command="$(jq -r '.tool_input.command // empty' <<<"$hook_input")"

git_push_pattern='git[[:space:]]+(-C[[:space:]]+[^[:space:]]+[[:space:]]+)?push'
if ! [[ "$command" =~ $git_push_pattern ]]; then
  exit 0
fi

if ! ci_output="$("$project_directory/ci.sh" 2>&1)"; then
  echo "Push bloqué : ./ci.sh échoue. Corriger avant de pousser." >&2
  echo "$ci_output" >&2
  exit 2
fi
