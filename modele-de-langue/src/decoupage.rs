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
    caractères_numérotés: Vec<char>,
    numéro_de_chaque_caractère: HashMap<char, usize>,
}

impl JeuDeCaractères {
    pub fn depuis_le_texte(texte: &str) -> Self {
        /* # Apprendre
           Les caractères sont triés pour que le même texte donne toujours les mêmes
           numéros : un modèle entraîné n'est valable qu'avec son jeu de caractères.
        */
        let caractères_distincts_triés: BTreeSet<char> = texte.chars().collect();
        let caractères_numérotés: Vec<char> = caractères_distincts_triés.into_iter().collect();

        let mut numéro_de_chaque_caractère = HashMap::new();
        for (numéro, caractère) in caractères_numérotés.iter().enumerate() {
            numéro_de_chaque_caractère.insert(*caractère, numéro);
        }

        JeuDeCaractères {
            caractères_numérotés,
            numéro_de_chaque_caractère,
        }
    }

    /// `None` si un caractère du texte n'est pas dans le jeu de caractères.
    pub fn encoder(&self, texte: &str) -> Option<Vec<usize>> {
        let mut numéros_du_texte = Vec::new();
        for caractère in texte.chars() {
            let numéro = self.numéro_de_chaque_caractère.get(&caractère)?;
            numéros_du_texte.push(*numéro);
        }
        Some(numéros_du_texte)
    }

    /// `None` si un numéro ne correspond à aucun caractère du jeu de caractères.
    pub fn décoder(&self, numéros_du_texte: &[usize]) -> Option<String> {
        let mut texte = String::new();
        for numéro in numéros_du_texte {
            let caractère = self.caractères_numérotés.get(*numéro)?;
            texte.push(*caractère);
        }
        Some(texte)
    }
}
