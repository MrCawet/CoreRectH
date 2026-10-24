use std::cell::RefCell;
use std::collections::VecDeque;

use moteur::historique::{Historique, ÉtatDuTexte};
use moteur::horloge::Horloge;

/// Doublure de l'horloge système : donne, dans l'ordre, les instants qu'on lui a confiés.
struct HorlogeProgrammée {
    instants_à_venir: RefCell<VecDeque<u64>>,
}

impl HorlogeProgrammée {
    fn qui_donnera(instants_en_millisecondes: &[u64]) -> Self {
        let instants_à_venir = instants_en_millisecondes.iter().copied().collect();
        HorlogeProgrammée {
            instants_à_venir: RefCell::new(instants_à_venir),
        }
    }
}

impl Horloge for HorlogeProgrammée {
    fn maintenant_en_millisecondes(&self) -> u64 {
        let mut instants_à_venir = self.instants_à_venir.borrow_mut();
        instants_à_venir
            .pop_front()
            .expect("l'horloge programmée n'a plus d'instant à donner")
    }
}

fn état(texte: &str, horodatage_en_millisecondes: u64) -> ÉtatDuTexte {
    ÉtatDuTexte {
        texte: texte.to_string(),
        horodatage_en_millisecondes,
    }
}

#[test]
fn un_historique_neuf_ne_contient_aucun_état() {
    let horloge = HorlogeProgrammée::qui_donnera(&[]);
    let historique = Historique::new(Box::new(horloge));

    assert_eq!(historique.états_enregistrés(), []);
}

#[test]
fn chaque_état_du_texte_saisi_est_enregistré_dans_l_ordre_avec_son_horodatage() {
    let horloge =
        HorlogeProgrammée::qui_donnera(&[1_000, 1_120, 1_250, 1_400, 1_530, 1_610, 1_700]);
    let mut historique = Historique::new(Box::new(horloge));

    historique.enregistrer("B");
    historique.enregistrer("Bo");
    historique.enregistrer("Bn");
    historique.enregistrer("B");
    historique.enregistrer("Bo");
    historique.enregistrer("Bon");
    historique.enregistrer("Bonj");

    assert_eq!(
        historique.états_enregistrés(),
        [
            état("B", 1_000),
            état("Bo", 1_120),
            état("Bn", 1_250),
            état("B", 1_400),
            état("Bo", 1_530),
            état("Bon", 1_610),
            état("Bonj", 1_700),
        ]
    );
}
