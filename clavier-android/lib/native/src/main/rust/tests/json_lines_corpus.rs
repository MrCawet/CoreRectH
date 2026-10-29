use std::fs;
use std::path::PathBuf;

use fl_native_rust::json_lines_corpus::JsonLinesCorpus;
use moteur::corpus::Corpus;
use moteur::saisie::{SaisieTerminée, ÉtatDuTexte};

fn chemin_temporaire(nom: &str) -> PathBuf {
    let chemin = std::env::temp_dir().join(format!("{}-{}", std::process::id(), nom));
    let _ = fs::remove_file(&chemin);
    chemin
}

fn frappe(timestamp_ms: u64, texte: &str) -> ÉtatDuTexte {
    ÉtatDuTexte {
        texte: texte.to_string(),
        timestamp_ms,
    }
}

#[test]
fn chaque_saisie_ajoutée_est_écrite_sur_sa_propre_ligne_json() {
    let chemin = chemin_temporaire("corpus.jsonl");
    let mut corpus = JsonLinesCorpus::new(chemin.clone());

    corpus.ajouter(SaisieTerminée {
        historique: vec![
            frappe(1_000, "B"),
            frappe(1_120, "Il a dit \"oui\",\n\tpuis \\ rien\u{1}"),
        ],
    });
    corpus.ajouter(SaisieTerminée { historique: vec![] });

    let contenu = fs::read_to_string(&chemin).expect("le corpus doit avoir été écrit");
    assert_eq!(
        contenu,
        concat!(
            r#"{"historique":[{"texte":"B","timestamp_ms":1000},{"texte":"Il a dit \"oui\",\n\tpuis \\ rien\u0001","timestamp_ms":1120}]}"#,
            "\n",
            r#"{"historique":[]}"#,
            "\n",
        )
    );
}
