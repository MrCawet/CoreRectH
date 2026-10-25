use std::cell::RefCell;
use std::collections::VecDeque;

use moteur::corpus::Corpus;
use moteur::horloge::Horloge;
use moteur::saisie::{Contexte, Historique, Saisie, SaisieTerminée, ÉtatDuTexte};

/// Doublure de l'horloge système : donne, dans l'ordre, les lectures qu'on lui a confiées.
struct HorlogeProgrammée {
    // Une horloge se lit sans être modifiée (`&self`), mais celle-ci doit avancer à chaque lecture.
    remaining_readings: RefCell<VecDeque<u64>>,
}

impl HorlogeProgrammée {
    fn qui_donnera(readings: Vec<u64>) -> Self {
        let remaining_readings = VecDeque::from(readings);
        HorlogeProgrammée {
            remaining_readings: RefCell::new(remaining_readings),
        }
    }
}

impl Horloge for HorlogeProgrammée {
    fn timestamp_ms(&self) -> u64 {
        let mut remaining_readings = self.remaining_readings.borrow_mut();
        remaining_readings
            .pop_front()
            .expect("l'horloge programmée n'a plus de lecture à donner")
    }
}

/// Doublure du stockage du corpus : garde en mémoire les saisies qu'on lui confie.
struct CorpusEnMémoire {
    saisies: Vec<SaisieTerminée>,
}

impl Corpus for CorpusEnMémoire {
    fn ajouter(&mut self, saisie: SaisieTerminée) {
        self.saisies.push(saisie);
    }
}

fn frappe(timestamp_ms: u64, texte: &str) -> ÉtatDuTexte {
    ÉtatDuTexte {
        texte: texte.to_string(),
        timestamp_ms: Some(timestamp_ms),
    }
}

#[test]
fn un_historique_neuf_ne_contient_aucun_état() {
    let horloge = HorlogeProgrammée::qui_donnera(vec![]);
    let historique = Historique::new(Box::new(horloge));

    assert_eq!(historique.contenu(), []);
}

#[test]
fn chaque_état_du_texte_saisi_est_enregistré_dans_l_ordre_avec_l_heure_de_sa_frappe() {
    let horloge =
        HorlogeProgrammée::qui_donnera(vec![1_000, 1_120, 1_250, 1_400, 1_530, 1_610, 1_700]);
    let mut historique = Historique::new(Box::new(horloge));

    historique.enregistrer("B");
    historique.enregistrer("Bo");
    historique.enregistrer("Bn");
    historique.enregistrer("B");
    historique.enregistrer("Bo");
    historique.enregistrer("Bon");
    historique.enregistrer("Bonj");

    assert_eq!(
        historique.contenu(),
        [
            frappe(1_000, "B"),
            frappe(1_120, "Bo"),
            frappe(1_250, "Bn"),
            frappe(1_400, "B"),
            frappe(1_530, "Bo"),
            frappe(1_610, "Bon"),
            frappe(1_700, "Bonj"),
        ]
    );
}

fn contexte_d_une_lettre() -> Contexte {
    Contexte {
        avant: Some("Cher Paul,\n".to_string()),
        après: None,
    }
}

#[test]
fn une_saisie_terminée_est_ajoutée_au_corpus_avec_son_contexte_et_son_historique() {
    let horloge = HorlogeProgrammée::qui_donnera(vec![1_000, 1_120]);
    let champ_vide = "";
    let mut saisie = Saisie::commencer(contexte_d_une_lettre(), champ_vide, Box::new(horloge));
    let mut corpus = CorpusEnMémoire { saisies: vec![] };

    saisie.enregistrer("B");
    saisie.enregistrer("Bo");
    saisie.terminer(&mut corpus);

    assert_eq!(
        corpus.saisies,
        [SaisieTerminée {
            contexte: contexte_d_une_lettre(),
            historique: vec![frappe(1_000, "B"), frappe(1_120, "Bo")],
        }]
    );
}

#[test]
fn la_valeur_déjà_présente_dans_le_champ_ouvre_l_historique_sans_heure() {
    let horloge = HorlogeProgrammée::qui_donnera(vec![1_000]);
    let mut saisie = Saisie::commencer(contexte_d_une_lettre(), "Bonjour", Box::new(horloge));
    let mut corpus = CorpusEnMémoire { saisies: vec![] };

    saisie.enregistrer("Bonjour P");
    saisie.terminer(&mut corpus);

    let valeur_déjà_présente = ÉtatDuTexte {
        texte: "Bonjour".to_string(),
        timestamp_ms: None,
    };
    assert_eq!(
        corpus.saisies,
        [SaisieTerminée {
            contexte: contexte_d_une_lettre(),
            historique: vec![valeur_déjà_présente, frappe(1_000, "Bonjour P")],
        }]
    );
}
