/// Une entrée et sa cible, tirées d'un texte encodé.
#[derive(Debug, PartialEq)]
pub struct ExempleDEntraînement {
    pub entrée: Vec<usize>,
    pub cible: Vec<usize>,
}

pub fn exemples_d_entraînement(
    _texte_encodé: &[usize],
    _taille_de_fenêtre: usize,
) -> Vec<ExempleDEntraînement> {
    Vec::new()
}
