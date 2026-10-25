use std::error::Error;
use std::io::{self, Read, Write};

use analyse::nettoyage::nettoyer;

/// Usage : nettoyer le corpus reçu sur l'entrée standard, écrire le résultat sur la sortie standard.
fn main() -> Result<(), Box<dyn Error>> {
    let mut corpus = String::new();
    io::stdin().read_to_string(&mut corpus)?;

    let corpus_nettoyé = nettoyer(&corpus)?;

    io::stdout().write_all(corpus_nettoyé.as_bytes())?;
    Ok(())
}
