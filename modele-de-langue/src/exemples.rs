/// Une entrée et sa cible, tirées d'un texte encodé.
#[derive(Debug, PartialEq)]
pub struct ExempleDEntraînement {
    pub entrée: Vec<usize>,
    pub cible: Vec<usize>,
}

/// `None` si la taille de fenêtre est nulle.
///
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
    taille_de_fenêtre: usize,
) -> Option<Vec<ExempleDEntraînement>> {
    if taille_de_fenêtre == 0 {
        return None;
    }

    /* # Apprendre
       La cible dépasse l'entrée d'un caractère : une fenêtre ne commence que s'il
       reste au moins un caractère après elle. Les caractères de la fin du texte qui
       ne remplissent pas une fenêtre entière sont laissés de côté.
    */
    // saturating_sub donne 0 plutôt qu'un débordement quand le texte est plus court
    // que la fenêtre : aucune fenêtre ne commence alors.
    let limite_des_débuts = texte_encodé.len().saturating_sub(taille_de_fenêtre);

    let mut exemples = Vec::new();
    for début in (0..limite_des_débuts).step_by(taille_de_fenêtre) {
        let fin = début + taille_de_fenêtre;
        let entrée = texte_encodé[début..fin].to_vec();
        let cible = texte_encodé[début + 1..fin + 1].to_vec();
        exemples.push(ExempleDEntraînement { entrée, cible });
    }
    Some(exemples)
}
