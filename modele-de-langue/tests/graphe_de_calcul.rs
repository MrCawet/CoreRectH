use modele_de_langue::graphe_de_calcul::GrapheDeCalcul;

#[test]
fn les_gradients_d_un_calcul_disent_de_combien_le_résultat_change_avec_chaque_variable() {
    let mut graphe = GrapheDeCalcul::default();
    let a = graphe.ajouter_une_variable(2.0);
    let b = graphe.ajouter_une_variable(3.0);

    let produit = graphe.ajouter_le_produit(a, b);
    let résultat = graphe.ajouter_la_somme(produit, a);
    let gradients = graphe.gradients_de(résultat);

    // résultat = a × b + a, donc il change de b + 1 avec a, et de a avec b.
    assert_eq!(résultat.valeur(), 8.0);
    assert_eq!(gradients.par_rapport_à(a).unwrap(), 4.0);
    assert_eq!(gradients.par_rapport_à(b).unwrap(), 2.0);
}
