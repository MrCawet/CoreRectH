/*! # Apprendre
   Entraîner un modèle, c'est ajuster ses paramètres pour réduire son erreur. Pour savoir
   dans quel sens pousser chaque paramètre, il faut son gradient : de combien l'erreur
   change quand ce paramètre augmente d'un tout petit peu (c'est sa dérivée).
   Ce module calcule les gradients d'un calcul fait de sommes et de produits. C'est ce que
   fait PyTorch (autograd) dans le livre, et ce que Burn fera pour notre modèle.
   Raschka, annexe A, sections A.3 et A.4 (le graphe de calcul, puis la différentiation automatique) :
   <https://github.com/rasbt/LLMs-from-scratch/blob/main/appendix-A/01_main-chapter-code/code-part1.ipynb>
   Andrej Karpathy construit pas à pas le même petit moteur, micrograd :
   <https://github.com/karpathy/micrograd>
*/

/// L'opération qui a produit un nœud, avec les nœuds dont elle part.
#[derive(Clone, Copy)]
enum Opération {
    Variable,
    Somme(Nœud, Nœud),
    Produit(Nœud, Nœud),
}

/// Les opérations enregistrées une à une, pour pouvoir calculer les gradients.
#[derive(Default)]
pub struct GrapheDeCalcul {
    /// La position de chaque opération est l'index du nœud qu'elle produit.
    opérations: Vec<Opération>,
}

/// Une valeur du graphe de calcul : une variable, ou le résultat d'une opération.
#[derive(Clone, Copy)]
pub struct Nœud {
    index: usize,
    valeur: f64,
}

/// Lire la valeur d'un nœud.
impl Nœud {
    pub fn valeur(&self) -> f64 {
        self.valeur
    }
}

/// Enregistrer l'opération qui produit un nouveau nœud.
impl GrapheDeCalcul {
    fn ajouter_un_nœud(&mut self, opération: Opération, valeur: f64) -> Nœud {
        let index = self.opérations.len();
        self.opérations.push(opération);
        Nœud { index, valeur }
    }
}

/// Construire le graphe, opération par opération, puis en calculer les gradients.
impl GrapheDeCalcul {
    pub fn ajouter_une_variable(&mut self, valeur: f64) -> Nœud {
        self.ajouter_un_nœud(Opération::Variable, valeur)
    }

    pub fn ajouter_la_somme(&mut self, gauche: Nœud, droite: Nœud) -> Nœud {
        let valeur = gauche.valeur + droite.valeur;
        self.ajouter_un_nœud(Opération::Somme(gauche, droite), valeur)
    }

    pub fn ajouter_le_produit(&mut self, gauche: Nœud, droite: Nœud) -> Nœud {
        let valeur = gauche.valeur * droite.valeur;
        self.ajouter_un_nœud(Opération::Produit(gauche, droite), valeur)
    }

    /** # Apprendre
    C'est la rétropropagation. Le résultat change exactement comme lui-même : son
    gradient vaut 1. On remonte ensuite les opérations de la dernière à la première, et
    chacune transmet le gradient de son nœud aux nœuds dont elle part.
    Un nœud est toujours ajouté après ceux dont il part. En remontant dans l'ordre
    inverse, on n'arrive donc à un nœud qu'une fois que tous ceux qui l'utilisent lui
    ont transmis leur part. Micrograd doit trier le graphe pour obtenir cet ordre (tri
    topologique) ; ici, l'ordre d'ajout suffit.
    */
    pub fn gradients_de(&self, résultat: Nœud) -> Gradients {
        let mut gradients = Gradients {
            as_vector: vec![0.0; self.opérations.len()],
        };
        gradients.as_vector[résultat.index] = 1.0;

        for index in (0..self.opérations.len()).rev() {
            let gradient_du_nœud = gradients.as_vector[index];
            match self.opérations[index] {
                Opération::Variable => {}
                Opération::Somme(gauche, droite) => {
                    /* # Apprendre
                       Quand gauche augmente d'un peu, gauche + droite augmente d'autant :
                       chacun reçoit le gradient du nœud tel quel.
                       Un nœud utilisé plusieurs fois reçoit une part à chaque usage : les
                       parts s'additionnent (`+=`).
                    */
                    gradients.as_vector[gauche.index] += gradient_du_nœud;
                    gradients.as_vector[droite.index] += gradient_du_nœud;
                }
                Opération::Produit(gauche, droite) => {
                    /* # Apprendre
                       Quand gauche augmente d'un peu, gauche × droite augmente droite fois
                       plus : la part de gauche est le gradient du nœud multiplié par droite.
                       Multiplier les gradients le long du chemin, c'est la règle de
                       dérivation des fonctions composées (en anglais, « chain rule »).
                    */
                    gradients.as_vector[gauche.index] += gradient_du_nœud * droite.valeur;
                    gradients.as_vector[droite.index] += gradient_du_nœud * gauche.valeur;
                }
            }
        }

        gradients
    }
}

/// De combien le résultat change quand chaque nœud augmente d'un tout petit peu.
pub struct Gradients {
    /// La position de chaque gradient est l'index de son nœud.
    as_vector: Vec<f64>,
}

/// Lire le gradient d'un nœud.
impl Gradients {
    /// `None` si le nœud n'appartient pas au graphe de calcul.
    pub fn par_rapport_à(&self, nœud: Nœud) -> Option<f64> {
        let gradient = self.as_vector.get(nœud.index)?;
        Some(*gradient)
    }
}
