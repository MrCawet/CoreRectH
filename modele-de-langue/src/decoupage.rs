/// Les lettres que le modèle sait lire, chacune associée à un nombre.
pub struct Vocabulaire {}

impl Vocabulaire {
    pub fn depuis_le_texte(_texte: &str) -> Self {
        Vocabulaire {}
    }

    /// `None` si une lettre du texte n'est pas dans le vocabulaire.
    pub fn encoder(&self, _texte: &str) -> Option<Vec<usize>> {
        Some(Vec::new())
    }

    /// `None` si un nombre ne correspond à aucune lettre du vocabulaire.
    pub fn décoder(&self, _nombres: &[usize]) -> Option<String> {
        Some(String::new())
    }
}
