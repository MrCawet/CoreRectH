# Catalogue des fautes

Les fautes relevées dans le corpus, classées par motif. Ce catalogue se complète à chaque nouveau corpus.

## Méthode

1. Pour chaque faute, noter ce qui a été tapé, le mot voulu, le motif de la faute, et l'information minimale qui permet de deviner la correction.
2. Traiter les motifs un par un dans le moteur, en commençant par ceux qui demandent le moins d'information. Les fautes du catalogue servent d'exemples pour les tests.
3. Mesurer sur le corpus avant de passer au motif suivant.

Pour une faute que l'opérateur n'a pas corrigée lui-même, seul lui connaît le mot voulu : l'agent le propose, l'opérateur le valide.

## Motifs

### Touche voisine (AZERTY)

Information nécessaire : le mot, la disposition du clavier, un dictionnaire.

| Tapé | Voulu |
|---|---|
| `commzncer` | `commencer` |
| `rzflechir` | `réfléchir` |
| `lz` | `le` |
| `crrexrion` | `correction` |
| `min` | `mon` |
| `co texte` | `contexte` |
| `u  petit` | `un petit` |

### Lettre manquante, en trop ou doublée

Information nécessaire : le mot, un dictionnaire.

| Tapé | Voulu |
|---|---|
| `choso` | `chois` (corrigée à la main) |
| `pa` | `pas` |
| `netoyage` | `nettoyage` |
| `refflzchir` | `réfléchir` |
| `incrementalle` | `incrémentale` |
| `avi` | `avis` |

### Apostrophe absente

Information nécessaire : le mot, un dictionnaire. Le mot sans apostrophe n'existe pas.

| Tapé | Voulu |
|---|---|
| `jai` | `j'ai` |
| `cest` | `c'est` |
| `davoir` | `d'avoir` |

### Accent absent

Information nécessaire : le mot, un dictionnaire.

| Tapé | Voulu |
|---|---|
| `methode` | `méthode` |
| `default` | `défaut` |

### Accent qui change le mot

Information nécessaire : les mots d'avant dans la phrase. Les deux formes existent.

| Tapé | Voulu |
|---|---|
| `a y` | `à y` |
| `a min avi` | `à mon avis` |
| `du qwerty` | `dû qwerty` |

### Accord

Information nécessaire : les mots d'avant dans la phrase.

| Tapé | Voulu |
|---|---|
| `je ne peut` | `je ne peux` |
| `on vas` | `on va` |
| `toute les fautes` | `toutes les fautes` |
| `quel information` | `quelle information` |
| `jai trop enticiper` | `j'ai trop anticipé` |

## Constats

- La plupart des fautes se devinent avec le mot seul, un dictionnaire et la disposition du clavier.
- Les autres demandent la phrase en cours, déjà présente dans l'historique de la saisie.
- Aucune ne demande le contexte autour du champ.
- Une faute peut relever de plusieurs motifs : `refflzchir` a une lettre doublée, une touche voisine et deux accents absents.

## Sources

- Le premier corpus du clavier Android, pris en partie en QWERTY : les fautes dues à la disposition QWERTY sont écartées.
- Les messages de l'opérateur à l'agent, tapés sur le même clavier.
