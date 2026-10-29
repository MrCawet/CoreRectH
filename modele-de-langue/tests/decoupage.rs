use modele_de_langue::decoupage::JeuDeCaractères;

#[test]
fn décoder_un_texte_encodé_redonne_le_texte() {
    let texte = "Petit souci, je ne peux pas choisir le clavier.";
    let jeu_de_caractères = JeuDeCaractères::from_texte(texte);

    let numéros_du_texte = jeu_de_caractères.encoder(texte).unwrap();
    let texte_décodé = jeu_de_caractères.décoder(&numéros_du_texte).unwrap();

    assert_eq!(texte_décodé, texte);
}
