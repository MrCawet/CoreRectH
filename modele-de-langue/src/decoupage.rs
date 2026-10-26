use std::collections::{BTreeSet, HashMap};

// [apprendre] Un modèle ne lit que des nombres. Le découpage transforme le texte en une suite
// [apprendre] de nombres, et inversement. Ici, l'unité est la lettre : chaque caractère
// [apprendre] distinct reçoit un numéro. GPT-2 découpe plutôt en morceaux de mots (BPE).
// [apprendre] Raschka, chapitre 2, sections 2.2 à 2.4 (découper le texte, puis numéroter les morceaux).

/// Les lettres que le modèle sait lire, chacune associée à un nombre.
pub struct Vocabulaire {
    letter_by_number: Vec<char>,
    number_by_letter: HashMap<char, usize>,
}

impl Vocabulaire {
    pub fn depuis_le_texte(texte: &str) -> Self {
        // [apprendre] Les lettres sont triées pour que le même texte donne toujours les mêmes
        // [apprendre] numéros : un modèle entraîné n'est valable qu'avec son vocabulaire.
        let distinct_letters: BTreeSet<char> = texte.chars().collect();
        let letter_by_number: Vec<char> = distinct_letters.into_iter().collect();

        let mut number_by_letter = HashMap::new();
        for (number, letter) in letter_by_number.iter().enumerate() {
            number_by_letter.insert(*letter, number);
        }

        Vocabulaire {
            letter_by_number,
            number_by_letter,
        }
    }

    /// `None` si une lettre du texte n'est pas dans le vocabulaire.
    pub fn encoder(&self, texte: &str) -> Option<Vec<usize>> {
        let mut nombres = Vec::new();
        for letter in texte.chars() {
            let number = self.number_by_letter.get(&letter)?;
            nombres.push(*number);
        }
        Some(nombres)
    }

    /// `None` si un nombre ne correspond à aucune lettre du vocabulaire.
    pub fn décoder(&self, nombres: &[usize]) -> Option<String> {
        let mut texte = String::new();
        for number in nombres {
            let letter = self.letter_by_number.get(*number)?;
            texte.push(*letter);
        }
        Some(texte)
    }
}
