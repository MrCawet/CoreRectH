use std::time::Duration;

use crate::horloge::Horloge;

#[derive(Debug, PartialEq, Eq)]
pub struct ÉtatDuTexte {
    pub texte: String,
    /// Depuis la frappe précédente. Absent pour la première frappe.
    pub délai: Option<Duration>,
}

pub struct Historique {
    horloge: Box<dyn Horloge>,
    contenu: Vec<ÉtatDuTexte>,
    last_timestamp_ms: Option<u64>,
}

impl Historique {
    pub fn new(horloge: Box<dyn Horloge>) -> Self {
        Historique {
            horloge,
            contenu: Vec::new(),
            last_timestamp_ms: None,
        }
    }

    pub fn enregistrer(&mut self, texte: &str) {
        let délai = self.délai_depuis_la_frappe_précédente();

        let nouvel_état = ÉtatDuTexte {
            texte: texte.to_string(),
            délai,
        };
        self.contenu.push(nouvel_état);
    }

    pub fn contenu(&self) -> &[ÉtatDuTexte] {
        &self.contenu
    }

    fn délai_depuis_la_frappe_précédente(&mut self) -> Option<Duration> {
        let timestamp_ms = self.horloge.timestamp_ms();
        let previous_timestamp_ms = self.last_timestamp_ms;
        self.last_timestamp_ms = Some(timestamp_ms);

        let elapsed_ms = timestamp_ms - previous_timestamp_ms?;
        Some(Duration::from_millis(elapsed_ms))
    }
}
