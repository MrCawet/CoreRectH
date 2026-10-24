/// Frontière vers l'horloge système.
pub trait Horloge {
    fn maintenant_en_millisecondes(&self) -> u64;
}
