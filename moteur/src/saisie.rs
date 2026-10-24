use crate::horloge::Horloge;

#[derive(Debug, PartialEq, Eq)]
pub struct ÉtatDuTexte {
    pub texte: String,
    pub horodatage_en_millisecondes: u64,
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
        let horodatage_en_millisecondes = self.horloge.maintenant_en_millisecondes();
        let nouvel_état = ÉtatDuTexte {
            texte: texte.to_string(),
            horodatage_en_millisecondes,
        };
        self.contenu.push(nouvel_état);
    }

    pub fn contenu(&self) -> &[ÉtatDuTexte] {
        &self.contenu
    }
}
