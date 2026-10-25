use crate::corpus::Corpus;
use crate::horloge::Horloge;

#[derive(Debug, PartialEq, Eq)]
pub struct ÉtatDuTexte {
    pub texte: String,
    /// Absent pour la valeur déjà présente dans le champ à l'ouverture : elle n'a pas été frappée.
    pub timestamp_ms: Option<u64>,
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
            timestamp_ms: Some(self.horloge.timestamp_ms()),
        };
        self.contenu.push(nouvel_état);
    }

    pub fn enregistrer_valeur_déjà_présente(&mut self, valeur: &str) {
        let état_sans_heure = ÉtatDuTexte {
            texte: valeur.to_string(),
            timestamp_ms: None,
        };
        self.contenu.push(état_sans_heure);
    }

    pub fn contenu(&self) -> &[ÉtatDuTexte] {
        &self.contenu
    }

    pub fn into_contenu(self) -> Vec<ÉtatDuTexte> {
        self.contenu
    }
}

/// Le texte qui entoure la saisie, quand il existe.
#[derive(Debug, PartialEq, Eq)]
pub struct Contexte {
    pub avant: Option<String>,
    pub après: Option<String>,
}

pub struct Saisie {
    contexte: Contexte,
    historique: Historique,
}

impl Saisie {
    pub fn commencer(contexte: Contexte, valeur_du_champ: &str, horloge: Box<dyn Horloge>) -> Self {
        let mut historique = Historique::new(horloge);
        if !valeur_du_champ.is_empty() {
            historique.enregistrer_valeur_déjà_présente(valeur_du_champ);
        }
        Saisie {
            contexte,
            historique,
        }
    }

    pub fn enregistrer(&mut self, texte: &str) {
        self.historique.enregistrer(texte);
    }

    pub fn terminer(self, corpus: &mut dyn Corpus) {
        let saisie_terminée = SaisieTerminée {
            contexte: self.contexte,
            historique: self.historique.into_contenu(),
        };
        corpus.ajouter(saisie_terminée);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SaisieTerminée {
    pub contexte: Contexte,
    pub historique: Vec<ÉtatDuTexte>,
}
