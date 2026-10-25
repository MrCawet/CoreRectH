use serde_json::Value;

/// Retire du corpus, au format JSON Lines, les saisies qui ne servent pas à l'analyse.
pub fn nettoyer(corpus: &str) -> Result<String, serde_json::Error> {
    let mut corpus_nettoyé = String::new();
    for ligne in corpus.lines() {
        let saisie: Value = serde_json::from_str(ligne)?;
        if a_au_moins_une_frappe(&saisie) {
            corpus_nettoyé.push_str(ligne);
            corpus_nettoyé.push('\n');
        }
    }
    Ok(corpus_nettoyé)
}

fn a_au_moins_une_frappe(saisie: &Value) -> bool {
    let historique = &saisie["historique"];
    match historique.as_array() {
        Some(états) => !états.is_empty(),
        None => false,
    }
}
