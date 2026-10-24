# Note de conception

Résume les choix techniques faits pour atteindre le [cap](CAP.md).

Ce sont des hypothèses de départ, à remettre en question à tout moment.

## Problème de départ

CoreRectH doit corriger le texte en cours de saisie dans les autres applications. Aucun système ne donne un accès universel aux champs de texte : il faut s'insérer là où le texte passe.

## Architecture

Un **cœur de correction** commun, appelé par des **points d'entrée** propres à chaque environnement.

```
                            Cœur de correction
             ▲                 ▲                 ▲                 ▲
   méthode de saisie IBus   plugin PhpStorm   mod Claude Code   plugin OnlyOffice / Euro-Office
```

Le cœur est une bibliothèque Rust : la crate `moteur`. Les points d'entrée l'appellent directement (en natif, par UniFFI depuis la JVM, en WebAssembly depuis JavaScript).

## Linux desktop (GNOME)

### Base : une méthode de saisie IBus

IBus est le système de méthodes de saisie intégré à GNOME. Une méthode de saisie IBus voit la frappe dans toutes les applications, sous X11 comme sous Wayland.

Elle corrige de deux façons :

1. **Lire et modifier le texte autour du curseur**, quand l'application le permet. Donne le contexte du document.
2. **Effacer et retaper** : la méthode de saisie se souvient de ce qu'elle a écrit, envoie des retours arrière puis le texte corrigé. Fonctionne partout, mais le contexte se limite à ce qui a été tapé.

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

Euro-Office est un fork d'OnlyOffice : même base de code, même API de plugins à ce jour. Un plugin JavaScript lit le document et remplace le texte par l'éditeur. En attendant, la méthode de saisie IBus corrige en effaçant et retapant.

### Pistes écartées

- **Plugin PhpStorm interceptant le terminal** : le terminal ne sait pas où est la zone de saisie de Claude Code. N'apporterait rien de plus que la méthode de saisie IBus.
- **Hook `UserPromptSubmit` de Claude Code** : ne peut pas réécrire le prompt.
- **Correction de Claude Code à l'envoi seulement** : remplacée par `prompt.edit`, qui corrige pendant la frappe.

## Android

### Un clavier, basé sur FlorisBoard

Un clavier (InputMethodService) a accès au champ de saisie actif dans toutes les applications. CoreRectH est donc un clavier.

Il part de FlorisBoard (Apache-2.0, Kotlin) :

- FlorisBoard n'a pas de correction intégrée : le cœur sera le seul correcteur, comme partout ailleurs, avec un seul historique.
- Son nouveau système de dispositions (k3lp, fondé sur le futur standard Unicode des claviers) décrit les dispositions dans des fichiers : changer de disposition ne demande pas de code.
- L'accès au champ est prêt : lire le texte autour du curseur, remplacer une plage de texte.

### Le cœur ne reçoit que le texte

Le cœur corrige le mot une fois terminé, à partir du texte et du contexte, sans savoir où le doigt a touché l'écran. Comme sous Linux, où un clavier physique ne donne pas cette information : le cœur fonctionne de la même façon partout.

### Pistes écartées

- **HeliBoard** : sa correction intégrée ferait double emploi avec le cœur.
- **FUTO Keyboard** : sa licence interdit de modifier le code pour un usage commercial.
- **Service d'accessibilité** : ne peut que remplacer tout le contenu d'un champ, et Google Play restreint cet usage.
- **Dictée** : on n'écrit pas comme on parle.
- **Dispositions alternatives** (T9, MessagEase, nid d'abeille, BÉPO) : la vitesse vient de la confiance dans la correction, pas de la disposition.

## À vérifier en pratique

- La méthode de saisie IBus dans chaque logiciel de la liste, en particulier PhpStorm et OnlyOffice.
- `prompt.edit` dans Claude Code, vérifié seulement dans la documentation et les types.
- La compatibilité de l'API de plugins entre OnlyOffice et Euro-Office.
- L'horloge du moteur : fournie par le moteur, ou par chaque point d'entrée (instant de la frappe donné par IBus ou Android, `performance.now()` en JavaScript, où l'horloge de Rust n'existe pas en WebAssembly). Le trait `Horloge` laisse le choix ouvert jusqu'au premier point d'entrée.
