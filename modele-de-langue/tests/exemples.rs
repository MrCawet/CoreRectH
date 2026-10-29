use std::num::NonZeroUsize;

use modele_de_langue::exemples::{ExempleDEntraînement, exemples_d_entraînement};

#[test]
fn la_cible_est_l_entrée_décalée_d_un_caractère_et_les_fenêtres_ne_se_recouvrent_pas() {
    let texte_encodé = [10, 11, 12, 13, 14, 15, 16];
    let taille_de_fenêtre = NonZeroUsize::new(3).unwrap();

    let exemples = exemples_d_entraînement(&texte_encodé, taille_de_fenêtre);

    assert_eq!(
        exemples,
        vec![
            ExempleDEntraînement {
                entrée: vec![10, 11, 12],
                cible: vec![11, 12, 13],
            },
            ExempleDEntraînement {
                entrée: vec![13, 14, 15],
                cible: vec![14, 15, 16],
            },
        ]
    );
}

#[test]
fn la_dernière_fenêtre_peut_être_plus_courte() {
    let texte_encodé = [10, 11, 12, 13, 14, 15];
    let taille_de_fenêtre = NonZeroUsize::new(3).unwrap();

    let exemples = exemples_d_entraînement(&texte_encodé, taille_de_fenêtre);

    assert_eq!(
        exemples,
        vec![
            ExempleDEntraînement {
                entrée: vec![10, 11, 12],
                cible: vec![11, 12, 13],
            },
            ExempleDEntraînement {
                entrée: vec![13, 14],
                cible: vec![14, 15],
            },
        ]
    );
}

#[test]
fn un_texte_plus_court_que_la_fenêtre_donne_un_exemple_plus_court() {
    let texte_encodé = [10, 11];
    let taille_de_fenêtre = NonZeroUsize::new(3).unwrap();

    let exemples = exemples_d_entraînement(&texte_encodé, taille_de_fenêtre);

    assert_eq!(
        exemples,
        vec![ExempleDEntraînement {
            entrée: vec![10],
            cible: vec![11],
        }]
    );
}

#[test]
fn un_texte_vide_ne_donne_aucun_exemple() {
    let texte_encodé = [];
    let taille_de_fenêtre = NonZeroUsize::new(3).unwrap();

    let exemples = exemples_d_entraînement(&texte_encodé, taille_de_fenêtre);

    assert_eq!(exemples, vec![]);
}
