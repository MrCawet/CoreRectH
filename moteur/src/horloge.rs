/// Frontière vers l'horloge système.
pub trait Horloge {
    /// L'origine est quelconque, seuls les écarts comptent.
    fn timestamp_ms(&self) -> u64;
}
