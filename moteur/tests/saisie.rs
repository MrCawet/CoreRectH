use std::cell::RefCell;
use std::collections::VecDeque;
use std::time::Duration;

use moteur::horloge::Horloge;
use moteur::saisie::{Historique, ÉtatDuTexte};

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
    fn timestamp_milliseconds(&self) -> u64 {
        let mut remaining_readings = self.remaining_readings.borrow_mut();
        remaining_readings
            .pop_front()
            .expect("l'horloge programmée n'a plus de lecture à donner")
    }
}

fn première_frappe(texte: &str) -> ÉtatDuTexte {
    ÉtatDuTexte {
        texte: texte.to_string(),
        délai_depuis_la_frappe_précédente: None,
    }
}

fn frappe_après(délai_en_millisecondes: u64, texte: &str) -> ÉtatDuTexte {
    ÉtatDuTexte {
        texte: texte.to_string(),
        délai_depuis_la_frappe_précédente: Some(Duration::from_millis(délai_en_millisecondes)),
    }
}

#[test]
fn un_historique_neuf_ne_contient_aucun_état() {
    let horloge = HorlogeProgrammée::qui_donnera(vec![]);
    let historique = Historique::new(Box::new(horloge));

    assert_eq!(historique.contenu(), []);
}

#[test]
fn chaque_état_du_texte_saisi_est_enregistré_dans_l_ordre_avec_le_délai_depuis_la_frappe_précédente()
 {
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
            première_frappe("B"),
            frappe_après(120, "Bo"),
            frappe_après(130, "Bn"),
            frappe_après(150, "B"),
            frappe_après(130, "Bo"),
            frappe_après(80, "Bon"),
            frappe_après(90, "Bonj"),
        ]
    );
}
