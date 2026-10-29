use modele_de_langue::exemples::{ExempleDEntraînement, exemples_d_entraînement};

#[test]
fn la_cible_est_l_entrée_décalée_d_un_caractère_et_les_fenêtres_ne_se_recouvrent_pas() {
    let texte_encodé = [10, 11, 12, 13, 14, 15, 16];

    let exemples = exemples_d_entraînement(&texte_encodé, 3).unwrap();

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
fn une_taille_de_fenêtre_nulle_ne_donne_aucun_exemple() {
    let texte_encodé = [10, 11, 12];

    let exemples = exemples_d_entraînement(&texte_encodé, 0);

    assert_eq!(exemples, None);
}

#[test]
fn un_texte_plus_court_que_la_fenêtre_ne_donne_aucun_exemple() {
    let texte_encodé = [10, 11];

    let exemples = exemples_d_entraînement(&texte_encodé, 3);

    assert_eq!(exemples, Some(vec![]));
}

#[test]
fn un_texte_vide_ne_donne_aucun_exemple() {
    let texte_encodé = [];

    let exemples = exemples_d_entraînement(&texte_encodé, 3);

    assert_eq!(exemples, Some(vec![]));
}
