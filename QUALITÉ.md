# Qualité

## Stratégie de test : TDD, école de Détroit

Approche TDD classique (école de Détroit). Pas de framework de mocking de l'école de Londres : aucune vérification d'appels internes (`verify()`, `toHaveBeenCalledWith`…).

Règles non négociables :

- **Tester les comportements, pas l'implémentation.** Les tests s'écrivent du point de vue des contrats publics et des cas d'usage métier (de l'intérieur vers l'extérieur).
- **Pas d'isolation au niveau des classes ou des fonctions.** Ne pas mocker ni stubber les classes internes, les fonctions ou les entités du domaine. Quand plusieurs classes collaborent pour réaliser un comportement, elles exécutent leur vrai code en mémoire.
- **Les doublures ne servent qu'aux frontières.** Mocks, stubs ou fakes en mémoire sont réservés aux frontières lentes, non déterministes ou externes au processus : base de données, réseau, API tierces, système de fichiers, horloge système.
- **Immunité au refactoring.** Les tests valident l'état final ou la valeur de retour, jamais la structure interne. Refactoriser un module ne doit jamais casser un test si le comportement métier reste identique.

## Architecture

L'arborescence crie le métier (correction, historique…), pas les outils (screaming architecture). Le métier ne dépend d'aucun détail technique : les frontières sont des adaptateurs branchés autour (clean architecture), et ce sont exactement elles qu'on double dans les tests.

## CI/CD

- Chaque push lance les tests.
- Une branche dont les tests échouent n'est pas fusionnée dans `main`.
- Pas de déploiement automatique pour l'instant.

## Commits

Format [Conventional Commits](https://www.conventionalcommits.org/fr/) minimal :

```
type: description
```

- `type` : `feat`, `fix`, `refactor`, `test`, `docs` ou `chore`.
- `description` : en français, à l'impératif ou au présent, sans majuscule ni point final.

Exemple : `feat: enregistre chaque état du texte saisi`
