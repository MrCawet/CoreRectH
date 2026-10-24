# CLAUDE.md

## Repères

- [NOTE-DE-CONCEPTION.md](NOTE-DE-CONCEPTION.md) : les choix techniques, hypothèses de départ à remettre en question.
- [QUALITY-GATES.md](QUALITY-GATES.md) : ce qui peut être automatisé (CI/CD, commits).
- [.claude/rules/rust.md](.claude/rules/rust.md) : règles propres au Rust, à lire avant d'écrire du Rust. Chargées automatiquement dès qu'un fichier `.rs` ou `Cargo.toml` est touché.
- `moteur/` : la crate du cœur de correction.
- `rust-toolchain.toml` : la version de Rust, la même en local et en CI.
- `./ci.sh` : toutes les vérifications (fmt, clippy, tests), à lancer avant de livrer. Un hook Claude Code le lance avant chaque `git push` et bloque le push s'il échoue.

Mettre à jour cette section à chaque changement qu'une nouvelle session perdrait du temps à retrouver : commandes fréquentes, particularités du projet, etc.

## Façon de travailler

L'agent et l'opérateur travaillent de manière itérative :
1. l'opérateur cadre le besoin ;
2. l'agent implémente et livre ;
3. on recommence autant que nécessaire.

L'agent fait donc le plus petit changement possible qui répond à la demande, sans faire de suppositions. Ce changement est complet par rapport à la demande et conforme aux règles de ce fichier.

- **Pas d'anticipation.** Aucun cas limite, test, code ou contrat qui n'a pas été demandé : l'agent le propose, il ne l'écrit pas. De même, il ne pose que les questions nécessaires à la demande en cours.
- **Éprouver un garde-fou.** Un garde-fou (lint, hook, CI) n'est déclaré en place qu'après l'avoir vu bloquer en conditions réelles.
- **Revue indépendante avant une PR** (à l'essai). Un sous-agent, sans le contexte de la session, relit le code au regard de ce fichier. L'agent transmet ses constats à l'opérateur, en les distinguant des siens.

## Réponses à l'opérateur

- **Courtes.** L'essentiel d'abord, le détail seulement sur demande.
- **Les questions à la fin, avec l'outil de questions à choix.** Chaque question rappelle son contexte en une phrase, chaque option dit ce qui change concrètement. Une option « Explique-moi d'abord » est toujours présente.
- **Une recommandation seulement pour un petit choix réversible**, en première option : sans réponse, l'agent l'applique. Une décision importante (difficile à défaire, ou qui touche au métier) n'en a aucune : l'opérateur doit lire pour choisir.
- **Une sous-question à la fois.** Si une réponse en soulève une autre, elle est traitée seule, avant la question suivante.

## Règles de code

1. Écrire du code facile à lire et explicite, plutôt que compact.
2. La `logique` du métier ou d'un comportement s'exprime en français. Le code exprime la même chose, c'est en cela qu'il est explicite. Une traduction dans une autre langue s'éloignerait toujours du sens premier : les identifiants de cette partie du code ne sont donc pas traduits (les accents sont toujours préférables, sauf quand ils ne sont pas possibles).
3. Les `détails d'implémentation` sont toute la partie du code nécessaire au fonctionnement de la `logique`. Cette partie peut varier à l'infini, tant qu'elle n'influe pas sur la `logique`. Les patterns techniques étant toujours en anglais, les identifiants de cette partie du code sont en anglais.
4. N'optimiser que si un problème de performance est signalé explicitement.
5. Rendre le code explicite plutôt que le commenter. Ne commenter que ce que le code ne peut pas dire : pourquoi il existe, ou pourquoi il est écrit d'une manière qui peut sembler étrange.
6. La documentation d'une variable ou d'un attribut n'indique que ce que son nom ne dit pas. Si le nom suffit, ne pas en écrire.
7. Aucune dépendance sans justification : la bibliothèque standard d'abord.

## Lisibilité

L'opérateur est dyslexique. Ces règles valent pour les identifiants, les noms de fichiers et la documentation.

1. Des mots entiers, toujours séparés, sans abréviation : `horodatage_en_millisecondes` plutôt que `ts_ms`, `historique` plutôt que `hist`.
2. Pas de noms qui se ressemblent dans un même voisinage, en particulier ceux qui ne diffèrent que d'une lettre (`état` et `états`) : préférer `état` et `historique`.
3. Une idée par ligne : pas d'enchaînement de méthodes sur une seule ligne, des variables intermédiaires nommées plutôt qu'une expression imbriquée.

## Stratégie de test : TDD, école de Détroit

Approche TDD classique (école de Détroit). Pas de framework de mocking de l'école de Londres : aucune vérification d'appels internes (`verify()`, `toHaveBeenCalledWith`…).

Règles non négociables :

- **Tester les comportements, pas l'implémentation.** Les tests s'écrivent du point de vue des contrats publics et des cas d'usage métier (de l'intérieur vers l'extérieur). Ils n'utilisent que l'API publique.
- **Le test d'abord.** Il échoue pour la bonne raison avant d'écrire le code. Un test existant ne se modifie que si le besoin change, jamais pour faire passer le code.
- **Le cycle se lit dans l'historique.** Un commit par étape :
  1. `test:` le test, qui échoue (rouge) ;
  2. `feat:` ou `fix:` le code qui le fait passer (vert) ;
  3. `refactor:` si besoin, le code amélioré, tests toujours verts.

  Ces commits sont poussés ensemble : le hook et la CI ne vérifient que le dernier.
- **Pas d'isolation au niveau des classes ou des fonctions.** Ne pas mocker ni stubber les classes internes, les fonctions ou les entités du domaine. Quand plusieurs classes collaborent pour réaliser un comportement, elles exécutent leur vrai code en mémoire.
- **Les doublures ne servent qu'aux frontières.** Mocks, stubs ou fakes en mémoire sont réservés aux frontières lentes, non déterministes ou externes au processus : base de données, réseau, API tierces, système de fichiers, horloge système.
- **Immunité au refactoring.** Les tests valident l'état final ou la valeur de retour, jamais la structure interne. Refactoriser un module ne doit jamais casser un test si le comportement métier reste identique.

## Architecture

L'arborescence crie le métier (correction, historique…), pas les outils (screaming architecture). Le métier ne dépend d'aucun détail technique : les frontières sont des adaptateurs branchés autour (clean architecture), et ce sont exactement elles qu'on double dans les tests.
