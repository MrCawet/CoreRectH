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
        let timestamp_milliseconds = self.horloge.timestamp_milliseconds();
        let délai = self.délai_depuis_la_frappe_précédente(timestamp_milliseconds);
        self.last_timestamp_ms = Some(timestamp_milliseconds);

        let nouvel_état = ÉtatDuTexte {
            texte: texte.to_string(),
            délai,
        };
        self.contenu.push(nouvel_état);
    }

    pub fn contenu(&self) -> &[ÉtatDuTexte] {
        &self.contenu
    }

    fn délai_depuis_la_frappe_précédente(
        &self,
        timestamp_milliseconds: u64,
    ) -> Option<Duration> {
        let last_timestamp_ms = self.last_timestamp_ms?;
        let elapsed_milliseconds = timestamp_milliseconds - last_timestamp_ms;
        Some(Duration::from_millis(elapsed_milliseconds))
    }
}
