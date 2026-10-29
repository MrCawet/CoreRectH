/// Les opérations enregistrées une à une, pour pouvoir calculer les gradients.
#[derive(Default)]
pub struct GrapheDeCalcul {}

/// Une valeur du graphe de calcul : une variable, ou le résultat d'une opération.
#[derive(Clone, Copy)]
pub struct Nœud {}

/// Lire la valeur d'un nœud.
impl Nœud {
    pub fn valeur(&self) -> f64 {
        0.0
    }
}

/// Construire le graphe, opération par opération, puis en calculer les gradients.
impl GrapheDeCalcul {
    pub fn ajouter_une_variable(&mut self, _valeur: f64) -> Nœud {
        Nœud {}
    }

    pub fn ajouter_la_somme(&mut self, _gauche: Nœud, _droite: Nœud) -> Nœud {
        Nœud {}
    }

    pub fn ajouter_le_produit(&mut self, _gauche: Nœud, _droite: Nœud) -> Nœud {
        Nœud {}
    }

    pub fn gradients_de(&self, _résultat: Nœud) -> Gradients {
        Gradients {}
    }
}

/// De combien le résultat change quand chaque nœud augmente d'un tout petit peu.
pub struct Gradients {}

/// Lire le gradient d'un nœud.
impl Gradients {
    /// `None` si le nœud n'appartient pas au graphe de calcul.
    pub fn par_rapport_à(&self, _nœud: Nœud) -> Option<f64> {
        Some(0.0)
    }
}
