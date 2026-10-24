use crate::saisie::SaisieTerminée;

/// Frontière vers le stockage des saisies terminées, réunies pour être analysées.
pub trait Corpus {
    fn ajouter(&mut self, saisie: SaisieTerminée);
}
