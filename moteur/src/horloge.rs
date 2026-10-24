/// Frontière vers l'horloge système, branchée par chaque point d'entrée.
pub trait Horloge {
    fn maintenant_en_millisecondes(&self) -> u64;
}
