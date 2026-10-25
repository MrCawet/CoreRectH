use analyse::nettoyage::nettoyer;

#[test]
fn une_saisie_sans_frappe_est_retirée_du_corpus() {
    let saisie_avec_frappe =
        r#"{"contexte":{"avant":null,"après":null},"historique":[{"texte":"P","timestamp_ms":1}]}"#;
    let saisie_sans_frappe = r#"{"contexte":{"avant":null,"après":null},"historique":[]}"#;
    let corpus = format!("{saisie_sans_frappe}\n{saisie_avec_frappe}\n{saisie_sans_frappe}\n");

    let corpus_nettoyé = nettoyer(&corpus).unwrap();

    assert_eq!(corpus_nettoyé, format!("{saisie_avec_frappe}\n"));
}
