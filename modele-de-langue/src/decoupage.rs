/*! # Apprendre
   Un modèle ne lit que des nombres. Le découpage transforme le texte en une suite
   de numéros, et inversement. Ici, l'unité est le caractère : chaque caractère
   distinct reçoit un numéro. GPT-2 découpe plutôt en morceaux de mots (BPE).
   L'ensemble des unités numérotées s'appelle le vocabulaire dans le livre ; avec des
   caractères, c'est un jeu de caractères.
   Raschka, chapitre 2, sections 2.2 à 2.4 (découper le texte, puis numéroter les morceaux) :
   <https://github.com/rasbt/LLMs-from-scratch/blob/main/ch02/01_main-chapter-code/ch02.ipynb>
*/

use std::collections::{BTreeSet, HashMap};

/// Les caractères que le modèle sait lire, chacun associé à un numéro.
pub struct JeuDeCaractères {
    /// La position de chaque caractère est son numéro.
    as_vector: Vec<char>,
    as_hash_map: HashMap<char, usize>,
}

/// Retrouver le numéro d'un caractère, ou le caractère d'un numéro.
impl JeuDeCaractères {
    fn numéro_de(&self, caractère: char) -> Option<usize> {
        let numéro = self.as_hash_map.get(&caractère)?;
        Some(*numéro)
    }

    fn caractère_de(&self, numéro: usize) -> Option<char> {
        let caractère = self.as_vector.get(numéro)?;
        Some(*caractère)
    }
}

/// Construire le jeu de caractères, encoder et décoder un texte.
impl JeuDeCaractères {
    pub fn from_texte(texte: &str) -> Self {
        /* # Apprendre
           Les caractères sont triés pour que le même texte donne toujours les mêmes
           numéros : un modèle entraîné n'est valable qu'avec son jeu de caractères.
        */
        let caractères_distincts_triés: BTreeSet<char> = texte.chars().collect();
        let as_vector: Vec<char> = caractères_distincts_triés.into_iter().collect();

        let mut as_hash_map = HashMap::new();
        for (numéro, caractère) in as_vector.iter().enumerate() {
            as_hash_map.insert(*caractère, numéro);
        }

        JeuDeCaractères {
            as_vector,
            as_hash_map,
        }
    }

    /// `None` si un caractère du texte n'est pas dans le jeu de caractères.
    pub fn encoder(&self, texte: &str) -> Option<Vec<usize>> {
        let mut texte_encodé = Vec::new();
        for caractère in texte.chars() {
            let numéro = self.numéro_de(caractère)?;
            texte_encodé.push(numéro);
        }
        Some(texte_encodé)
    }

    /// `None` si un numéro ne correspond à aucun caractère du jeu de caractères.
    pub fn décoder(&self, texte_encodé: &[usize]) -> Option<String> {
        let mut texte = String::new();
        for numéro in texte_encodé {
            let caractère = self.caractère_de(*numéro)?;
            texte.push(caractère);
        }
        Some(texte)
    }
}
