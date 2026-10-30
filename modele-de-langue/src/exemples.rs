use std::num::NonZeroUsize;

/// Une entrée et sa cible, tirées d'un texte encodé.
/** # Apprendre
   Le modèle ne reçoit pas les exemples d'entraînement un par un : ils sont regroupés
   en lots, quelques exemples traités d'un coup (voir [`crate::lots`]).
   Pourquoi : la carte graphique sait faire le même calcul sur beaucoup de nombres à la
   fois. Traiter 8 exemples ensemble prend à peu près le temps d'en traiter un seul.
   Et le modèle se corrige d'après l'erreur moyenne sur tout le lot, plus stable que
   l'erreur d'un seul exemple : un exemple inhabituel le fait moins dévier.
   Raschka, chapitre 2, fin de la section 2.6 (`batch_size`) :
   <https://github.com/rasbt/LLMs-from-scratch/blob/main/ch02/01_main-chapter-code/ch02.ipynb>
*/
#[derive(Clone, Debug, PartialEq)]
pub struct ExempleDEntraînement {
    pub entrée: Vec<usize>,
    pub cible: Vec<usize>,
}

/** # Apprendre
   Le modèle apprend à deviner le caractère suivant. Un exemple d'entraînement lui
   donne une entrée et sa cible, de même taille : la cible est l'entrée décalée d'un
   caractère. À chaque position de l'entrée, la cible donne le caractère qui vient
   juste après. Une seule fenêtre de n caractères enseigne donc n prédictions :
   deviner le 2e caractère à partir du 1er, le 3e à partir des deux premiers, etc.
   La fenêtre glisse sur le texte d'un pas (le `stride` de Raschka). Ici, le pas vaut
   la taille de fenêtre : les fenêtres se suivent sans se recouvrir. Le modèle ne
   voit donc pas plusieurs fois les mêmes caractères, ce qui le pousserait à
   apprendre le texte par cœur.
   Raschka, chapitre 2, section 2.6 (`GPTDatasetV1`, échantillonner avec une fenêtre
   glissante) :
   <https://github.com/rasbt/LLMs-from-scratch/blob/main/ch02/01_main-chapter-code/ch02.ipynb>
*/
pub fn exemples_d_entraînement(
    texte_encodé: &[usize],
    taille_de_fenêtre: NonZeroUsize,
) -> Vec<ExempleDEntraînement> {
    /* # Apprendre
       `NonZeroUsize` est un `usize` qui ne peut pas valoir 0 : le compilateur
       l'impose à l'appelant, qui doit passer par `NonZeroUsize::new`. Une fenêtre
       vide n'a pas de sens, et la boucle plus bas n'avancerait jamais.
       `get` redonne le `usize`, pour calculer avec.
       <https://doc.rust-lang.org/std/num/type.NonZeroUsize.html>
    */
    let taille_de_fenêtre = taille_de_fenêtre.get();

    /* # Apprendre
       La cible dépasse l'entrée d'un caractère : le dernier caractère du texte ne
       peut être que dans une cible, jamais dans une entrée. Les entrées se partagent
       les autres caractères, une fenêtre chacune. La dernière prend ce qui reste,
       même si c'est moins qu'une fenêtre entière : Raschka laisse ce reste de côté,
       ici aucun caractère n'est perdu.
    */
    // saturating_sub donne 0 plutôt qu'un débordement quand le texte est vide.
    let last_index = texte_encodé.len().saturating_sub(1);

    let mut exemples = Vec::new();
    let mut début = 0;
    while début < last_index {
        let fin_d_une_fenêtre_entière = début + taille_de_fenêtre;
        let fin = fin_d_une_fenêtre_entière.min(last_index);
        let entrée = texte_encodé[début..fin].to_vec();
        let cible = texte_encodé[début + 1..fin + 1].to_vec();
        exemples.push(ExempleDEntraînement { entrée, cible });
        début = fin;
    }
    exemples
}
