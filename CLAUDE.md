# CLAUDE.md

## Façon de travailler

L'agent et l'opérateur travaillent de manière itérative :
1. l'opérateur cadre le besoin ;
2. l'agent implémente et livre ;
3. on recommence autant que nécessaire.

L'agent fait donc le plus petit changement possible qui répond à la demande, sans faire de suppositions. Ce changement est complet par rapport à la demande et conforme aux règles de ce fichier.

## Règles de code

- Écrire du code facile à lire et explicite, plutôt que compact.
- N'optimiser que si un problème de performance est signalé explicitement.
- Rendre le code explicite plutôt que le commenter. Ne commenter que ce que le code ne peut pas dire : pourquoi il existe, ou pourquoi il est écrit d'une manière qui peut sembler étrange.
- Les descriptions de variables ou d'attributs ne disent que ce que leur nom ne dit pas.
- Coder en français, à l'exception des termes techniques. Ne jamais traduire ni reformuler les termes des règles métier.
