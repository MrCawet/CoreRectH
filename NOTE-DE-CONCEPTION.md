# Note de conception

Résume les choix techniques faits pour atteindre le [cap](CAP.md).

## Problème de départ

CoreRectH doit corriger le texte en cours de saisie dans les autres applications. Aucun système ne donne un accès universel aux champs de texte : il faut s'insérer là où le texte passe.

## Architecture

Un **cœur de correction** commun, appelé par des **points d'entrée** propres à chaque environnement.

```
                     Cœur de correction
        ▲              ▲              ▲              ▲
   moteur IBus    plugin PhpStorm   mod Claude Code   plugin OnlyOffice / Euro-Office
```

La forme du cœur (service local, bibliothèque) reste à choisir.

## Linux desktop (GNOME)

### Base : un moteur de saisie IBus

IBus est la méthode de saisie intégrée à GNOME. Un moteur IBus voit la frappe dans toutes les applications, sous X11 comme sous Wayland.

Il corrige de deux façons :

1. **Lire et modifier le texte autour du curseur**, quand l'application le permet. Donne le contexte du document.
2. **Effacer et retaper** : le moteur se souvient de ce qu'il a écrit, envoie des retours arrière puis le texte corrigé. Fonctionne partout, mais le contexte se limite à ce qui a été tapé.

| Logiciel | Lire et modifier autour du curseur | Effacer et retaper |
|---|---|---|
| Éditeur de texte GNOME | oui | oui |
| Firefox | oui, sauf éditeurs web dessinés eux-mêmes | oui |
| Thunderbird | oui | oui |
| LibreOffice | oui | oui |
| PhpStorm | non | probablement (voie de saisie Java, à tester) |
| Terminaux | non | oui |
| OnlyOffice / Euro-Office | non (document dessiné dans un canevas) | à tester |

### PhpStorm : un plugin JetBrains

- Accès complet au document.
- Sait distinguer le code du langage naturel (commentaires, PHPDoc, chaînes, Markdown, messages de commit) : seul le langage naturel est corrigé.
- Les corrections passent par l'éditeur, donc s'annulent avec Ctrl+Z.

### Claude Code : un mod Claude Code

Valable dans n'importe quel terminal, dont celui de PhpStorm.

- L'événement `prompt.edit` se déclenche à chaque frappe dans la zone de saisie. Le mod peut renvoyer le texte corrigé et la position du curseur.
- `$.session.messages()` donne la conversation comme contexte.
- `$.store` conserve l'historique des corrections. Une commande `/corrections` permet de le consulter.

Le traitement d'une frappe dispose de 50 ms. D'où deux niveaux :

1. **Corrections immédiates**, locales et rapides, appliquées à la fin du mot.
2. **Corrections par le contexte**, calculées en arrière-plan et appliquées à la frappe suivante, sans déplacer le curseur.

### OnlyOffice / Euro-Office : un plugin, plus tard

Euro-Office est un fork d'OnlyOffice : même moteur, même API de plugins à ce jour. Un plugin JavaScript lit le document et remplace le texte par l'éditeur. En attendant, le moteur IBus corrige en effaçant et retapant.

### Pistes écartées

- **Plugin PhpStorm interceptant le terminal** : le terminal ne sait pas où est la zone de saisie de Claude Code. N'apporterait rien de plus que le moteur IBus.
- **Hook `UserPromptSubmit` de Claude Code** : ne peut pas réécrire le prompt.
- **Correction de Claude Code à l'envoi seulement** : remplacée par `prompt.edit`, qui corrige pendant la frappe.

## Android

À discuter. Piste envisagée : un clavier (InputMethodService), qui a accès au champ de saisie actif dans toutes les applications. Alternative : un service d'accessibilité, plus limité et restreint par Google Play.

## À vérifier en pratique

- Le moteur IBus dans chaque logiciel de la liste, en particulier PhpStorm et OnlyOffice.
- `prompt.edit` dans Claude Code, vérifié seulement dans la documentation et les types.
- La compatibilité de l'API de plugins entre OnlyOffice et Euro-Office.
