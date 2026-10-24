use crate::horloge::Horloge;

#[derive(Debug, PartialEq, Eq)]
pub struct ÉtatDuTexte {
    pub texte: String,
    pub timestamp_ms: u64,
}

pub struct Historique {
    horloge: Box<dyn Horloge>,
    contenu: Vec<ÉtatDuTexte>,
}

impl Historique {
    pub fn new(horloge: Box<dyn Horloge>) -> Self {
        Historique {
            horloge,
            contenu: Vec::new(),
        }
    }

    pub fn enregistrer(&mut self, texte: &str) {
        let nouvel_état = ÉtatDuTexte {
            texte: texte.to_string(),
            timestamp_ms: self.horloge.timestamp_ms(),
        };
        self.contenu.push(nouvel_état);
    }

    pub fn contenu(&self) -> &[ÉtatDuTexte] {
        &self.contenu
    }
}
