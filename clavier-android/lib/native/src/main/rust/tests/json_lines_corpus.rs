use std::fs;
use std::path::PathBuf;

use fl_native_rust::json_lines_corpus::JsonLinesCorpus;
use moteur::corpus::Corpus;
use moteur::saisie::{Contexte, SaisieTerminée, ÉtatDuTexte};

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
        contexte: Contexte {
            avant: Some("Il a dit \"oui\",\n\tpuis \\ rien\u{1}".to_string()),
            après: None,
        },
        historique: vec![frappe(1_000, "B"), frappe(1_120, "Bo")],
    });
    corpus.ajouter(SaisieTerminée {
        contexte: Contexte {
            avant: None,
            après: Some("fin".to_string()),
        },
        historique: vec![],
    });

    let contenu = fs::read_to_string(&chemin).expect("le corpus doit avoir été écrit");
    assert_eq!(
        contenu,
        concat!(
            r#"{"contexte":{"avant":"Il a dit \"oui\",\n\tpuis \\ rien\u0001","après":null},"historique":[{"texte":"B","timestamp_ms":1000},{"texte":"Bo","timestamp_ms":1120}]}"#,
            "\n",
            r#"{"contexte":{"avant":null,"après":"fin"},"historique":[]}"#,
            "\n",
        )
    );
}
