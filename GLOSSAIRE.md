# Glossaire

Les termes du métier, tels qu'ils s'écrivent dans le code. Un même mot désigne toujours la même chose. Un nouveau terme s'ajoute ici avant d'apparaître dans le code.

## Saisie

| Terme | Sens | Dans le code |
|---|---|---|
| saisie | Ce que l'utilisateur tape dans un champ, de l'ouverture du champ à sa fermeture. | `Saisie` |
| frappe | Un changement du texte dans le champ. Chaque frappe ajoute un état du texte à l'historique. | `Saisie::enregistrer` |
| état du texte | Le contenu du champ à un instant, avec son heure. | `ÉtatDuTexte` |
| historique | La suite des états du texte d'une saisie. | `Historique` |
| contexte | Le texte qui entoure la saisie, avant et après, quand il existe. | `Contexte` |
| saisie terminée | Une saisie close, avec son contexte et son historique. | `SaisieTerminée` |
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

## Calcul des gradients

Un petit moteur écrit pour apprendre comment un modèle s'entraîne. Burn fera ce travail pour le vrai modèle.

| Terme | Sens | Dans le code |
|---|---|---|
| graphe de calcul | Les opérations enregistrées une à une, de la variable jusqu'au résultat, pour pouvoir calculer les gradients. | `GrapheDeCalcul` |
| nœud | Une valeur du graphe de calcul : une variable, ou le résultat d'une opération. | `Nœud` |
| variable | Une valeur donnée au départ. Pour le modèle, ce seront ses paramètres. | `GrapheDeCalcul::ajouter_une_variable` |
| gradient | De combien le résultat change quand un nœud augmente d'un tout petit peu. | `Gradients` |
| rétropropagation | Calculer les gradients en remontant le graphe, du résultat vers les variables. | `GrapheDeCalcul::gradients_de` |
