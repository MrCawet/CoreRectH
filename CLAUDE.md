# CLAUDE.md

## Façon de travailler

L'agent et l'opérateur travaillent de manière itérative :
1. l'opérateur cadre le besoin ;
2. l'agent implémente et livre ;
3. on recommence autant que nécessaire.

L'agent fait donc le plus petit changement possible qui répond à la demande, sans faire de suppositions. Ce changement est complet par rapport à la demande et conforme aux règles de ce fichier.

## Règles de code

1. La `logique` du métier ou d'un comportement s'exprime en français. Le code exprime la même chose, c'est en cela qu'il est explicite. Une traduction dans une autre langue s'éloignerait toujours du sens premier : les identifiants de cette partie du code ne sont donc pas traduits (les accents sont toujours préférables, sauf quand ils ne sont pas possibles).
2. Les `détails d'implémentation` sont toute la partie du code nécessaire au fonctionnement de la `logique`. Cette partie peut varier à l'infini, tant qu'elle n'influe pas sur la `logique`. Les patterns techniques étant toujours en anglais, les identifiants de cette partie du code sont en anglais.
3. Écrire du code facile à lire et explicite, plutôt que compact.
4. N'optimiser que si un problème de performance est signalé explicitement.
5. Rendre le code explicite plutôt que le commenter. Ne commenter que ce que le code ne peut pas dire : pourquoi il existe, ou pourquoi il est écrit d'une manière qui peut sembler étrange.
6. La documentation d'une variable ou d'un attribut n'indique que ce que son nom ne dit pas. Si le nom suffit, ne pas en écrire.
