use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::PathBuf;

use moteur::corpus::Corpus;
use moteur::saisie::{SaisieTerminée, ÉtatDuTexte};

/// Écrit le corpus dans un fichier JSON Lines : une saisie terminée par ligne, ajoutée à la fin.
pub struct JsonLinesCorpus {
    path: PathBuf,
}

impl JsonLinesCorpus {
    pub fn new(path: PathBuf) -> Self {
        JsonLinesCorpus { path }
    }

    fn append_line(&self, line: &str) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        writeln!(file, "{line}")
    }
}

impl Corpus for JsonLinesCorpus {
    fn ajouter(&mut self, saisie: SaisieTerminée) {
        let line = saisie_to_json(&saisie);
        // Le trait ne prévoit pas d'échec : une saisie perdue ne doit pas bloquer la frappe.
        let _ = self.append_line(&line);
    }
}

fn saisie_to_json(saisie: &SaisieTerminée) -> String {
    let états: Vec<String> = saisie.historique.iter().map(état_to_json).collect();
    let historique = états.join(",");
    format!(r#"{{"historique":[{historique}]}}"#)
}

fn état_to_json(état: &ÉtatDuTexte) -> String {
    let texte = string_to_json(&état.texte);
    let timestamp_ms = état.timestamp_ms;
    format!(r#"{{"texte":{texte},"timestamp_ms":{timestamp_ms}}}"#)
}

fn string_to_json(text: &str) -> String {
    let mut json = String::from("\"");
    for character in text.chars() {
        match character {
            '"' => json.push_str("\\\""),
            '\\' => json.push_str("\\\\"),
            '\n' => json.push_str("\\n"),
            '\r' => json.push_str("\\r"),
            '\t' => json.push_str("\\t"),
            control if control < ' ' => {
                let code = control as u32;
                json.push_str(&format!("\\u{code:04x}"));
            }
            other => json.push(other),
        }
    }
    json.push('"');
    json
}
