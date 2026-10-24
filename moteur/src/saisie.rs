use std::time::Duration;

use crate::horloge::Horloge;

#[derive(Debug, PartialEq, Eq)]
pub struct ÉtatDuTexte {
    pub texte: String,
    /// Absent pour la première frappe.
    pub délai_depuis_la_frappe_précédente: Option<Duration>,
}

pub struct Historique {
    horloge: Box<dyn Horloge>,
    contenu: Vec<ÉtatDuTexte>,
    previous_timestamp_milliseconds: Option<u64>,
}

impl Historique {
    pub fn new(horloge: Box<dyn Horloge>) -> Self {
        Historique {
            horloge,
            contenu: Vec::new(),
            previous_timestamp_milliseconds: None,
        }
    }

    pub fn enregistrer(&mut self, texte: &str) {
        let timestamp_milliseconds = self.horloge.timestamp_milliseconds();
        let délai_depuis_la_frappe_précédente =
            self.délai_depuis_la_frappe_précédente(timestamp_milliseconds);
        self.previous_timestamp_milliseconds = Some(timestamp_milliseconds);

        let nouvel_état = ÉtatDuTexte {
            texte: texte.to_string(),
            délai_depuis_la_frappe_précédente,
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
        let previous_timestamp_milliseconds = self.previous_timestamp_milliseconds?;
        // Une horloge qui recule donnerait un débordement : on compte alors un délai nul.
        let elapsed_milliseconds =
            timestamp_milliseconds.saturating_sub(previous_timestamp_milliseconds);
        Some(Duration::from_millis(elapsed_milliseconds))
    }
}
