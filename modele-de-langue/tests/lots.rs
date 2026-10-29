use std::num::NonZeroUsize;

use burn::tensor::{Int, Tensor};
use modele_de_langue::exemples::exemples_d_entraînement;
use modele_de_langue::lots::{Lot, lots};

const NUMÉRO_DE_REMPLISSAGE: usize = 99;

fn lignes(tenseur: Tensor<2, Int>) -> Vec<Vec<i64>> {
    let [_, largeur] = tenseur.dims();
    let données = tenseur.into_data();
    let numéros = données.try_into_vec_as::<i64>().unwrap();
    numéros
        .chunks(largeur)
        .map(|ligne| ligne.to_vec())
        .collect()
}

fn lots_de(texte_encodé: &[usize], taille_de_fenêtre: usize, taille_de_lot: usize) -> Vec<Lot> {
    let taille_de_fenêtre = NonZeroUsize::new(taille_de_fenêtre).unwrap();
    let taille_de_lot = NonZeroUsize::new(taille_de_lot).unwrap();
    let exemples = exemples_d_entraînement(texte_encodé, taille_de_fenêtre);

    let chargeur = lots(exemples, taille_de_lot, NUMÉRO_DE_REMPLISSAGE);

    chargeur.iter().map(|lot| lot.unwrap()).collect()
}

#[test]
fn les_exemples_sont_regroupés_dans_l_ordre_par_taille_de_lot() {
    let texte_encodé = [10, 11, 12, 13, 14, 15, 16, 17, 18, 19];

    let mut lots = lots_de(&texte_encodé, 3, 2).into_iter();

    let premier = lots.next().unwrap();
    assert_eq!(
        lignes(premier.entrées),
        vec![vec![10, 11, 12], vec![13, 14, 15]]
    );
    assert_eq!(
        lignes(premier.cibles),
        vec![vec![11, 12, 13], vec![14, 15, 16]]
    );
    let dernier = lots.next().unwrap();
    assert_eq!(lignes(dernier.entrées), vec![vec![16, 17, 18]]);
    assert_eq!(lignes(dernier.cibles), vec![vec![17, 18, 19]]);
    assert!(lots.next().is_none());
}

#[test]
fn l_exemple_plus_court_est_complété_par_le_numéro_de_remplissage() {
    let texte_encodé = [10, 11, 12, 13, 14, 15];

    let lots = lots_de(&texte_encodé, 3, 2);

    let [lot] = <[Lot; 1]>::try_from(lots).unwrap();
    assert_eq!(
        lignes(lot.entrées),
        vec![vec![10, 11, 12], vec![13, 14, 99]]
    );
    assert_eq!(lignes(lot.cibles), vec![vec![11, 12, 13], vec![14, 15, 99]]);
}
