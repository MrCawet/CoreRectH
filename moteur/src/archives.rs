use crate::saisie::SaisieTerminée;

/// Frontière vers le stockage des saisies terminées, gardées pour être analysées.
pub trait Archives {
    fn archiver(&mut self, saisie: SaisieTerminée);
}
