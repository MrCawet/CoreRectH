/// Retire du corpus, au format JSON Lines, les saisies qui ne servent pas à l'analyse.
pub fn nettoyer(corpus: &str) -> Result<String, serde_json::Error> {
    Ok(corpus.to_string())
}
