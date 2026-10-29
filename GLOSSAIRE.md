# Glossaire

Les termes du métier, tels qu'ils s'écrivent dans le code. Un même mot désigne toujours la même chose. Un nouveau terme s'ajoute ici avant d'apparaître dans le code.

## Saisie

| Terme | Sens | Dans le code |
|---|---|---|
| saisie | Ce que l'utilisateur tape dans un champ, de l'ouverture du champ à sa fermeture. | `Saisie` |
| frappe | Un changement du texte dans le champ. Chaque frappe ajoute un état du texte à l'historique. | `Saisie::enregistrer` |
| état du texte | Le contenu du champ à un instant, avec son heure. | `ÉtatDuTexte` |
| historique | La suite des états du texte d'une saisie. | `Historique` |
| contexte | Le texte qui entoure la saisie, avant et après, quand il existe. Pas encore dans le code. | |
| saisie terminée | Une saisie close, avec son historique. | `SaisieTerminée` |
| corpus | L'ensemble des saisies terminées, réunies pour être analysées. | `Corpus` |
| nettoyage | Retirer du corpus les saisies qui ne servent pas à l'analyse. | `analyse::nettoyage::nettoyer` |
| horloge | Ce qui donne l'heure de chaque état du texte. | `Horloge` |

## Fautes

| Terme | Sens | Dans le code |
|---|---|---|
| faute | Un mot tapé qui diffère du mot voulu. | voir [analyse/fautes.md](analyse/fautes.md) |
| motif | La forme d'une faute : touche voisine, lettre manquante, accent absent… | |

## Modèle de langue

| Terme | Sens | Dans le code |
|---|---|---|
| modèle de langue | Le modèle qui devine la suite d'un texte. | crate `modele-de-langue` |
| découpage | Transformer un texte en suite de numéros, et inversement. | module `decoupage` |
| caractère | Toute unité du texte : lettre, chiffre, espace, ponctuation. On ne dit « lettre » que pour une lettre. | `char` |
| jeu de caractères | Les caractères que le modèle sait lire, chacun associé à un numéro. Le livre de Raschka dit « vocabulaire ». | `JeuDeCaractères` |
| numéro | L'identifiant d'un caractère dans le jeu de caractères. Ce n'est pas une quantité : on ne dit pas « nombre ». | `usize` |
| encoder, décoder | Passer d'un texte à ses numéros, et inversement. | `encoder`, `décoder` |
| exemple d'entraînement | Une entrée et sa cible, tirées d'un texte encodé. | `ExempleDEntraînement`, `exemples_d_entraînement` |
| fenêtre | Une suite de caractères consécutifs du texte, de la taille de fenêtre. Seule la dernière du texte peut être plus courte. Le livre de Raschka dit « contexte » ; ici, « contexte » désigne déjà le texte autour d'une saisie. | |
| entrée | La fenêtre que lit le modèle. | `ExempleDEntraînement::entrée` |
| cible | La même fenêtre décalée d'un caractère, qui donne à chaque position le caractère suivant. Le livre de Raschka dit *target*. | `ExempleDEntraînement::cible` |
| taille de fenêtre | Le nombre maximal de caractères d'une fenêtre. Le livre de Raschka dit `context_size` ou `max_length`. | `taille_de_fenêtre` |
