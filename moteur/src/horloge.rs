/// Frontière vers l'horloge système.
pub trait Horloge {
    /// Monotone : ne recule jamais. L'origine est quelconque, seuls les écarts comptent.
    fn timestamp_milliseconds(&self) -> u64;
}
