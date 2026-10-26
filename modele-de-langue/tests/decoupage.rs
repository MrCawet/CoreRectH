use modele_de_langue::decoupage::Vocabulaire;

#[test]
fn décoder_un_texte_encodé_redonne_le_texte() {
    let texte = "Petit souci, je ne peux pas choisir le clavier.";
    let vocabulaire = Vocabulaire::depuis_le_texte(texte);

    let nombres = vocabulaire.encoder(texte).unwrap();
    let texte_décodé = vocabulaire.décoder(&nombres).unwrap();

    assert_eq!(texte_décodé, texte);
}
