/*! # Apprendre
   Le modèle ne lit pas les exemples d'entraînement un par un, mais par lots : la carte
   graphique calcule tous les exemples d'un lot en même temps. Un lot range ses
   exemples dans un tenseur, un tableau de nombres : une ligne par exemple, une colonne
   par caractère. Toutes les lignes d'un tableau ont la même longueur.
   Raschka, chapitre 2, fin de la section 2.6 (`DataLoader`, `batch_size`) :
   <https://github.com/rasbt/LLMs-from-scratch/blob/main/ch02/01_main-chapter-code/ch02.ipynb>
   Burn sépare le travail en trois : un `Dataset` donne les exemples, un `Batcher`
   range une poignée d'exemples en un lot, et le `DataLoader` découpe le `Dataset` en
   poignées de la taille de lot et les passe au `Batcher`.
   <https://burn.dev/books/burn/basic-workflow/data.html>
*/

use std::num::NonZeroUsize;
use std::sync::Arc;

use burn::data::dataloader::batcher::Batcher;
use burn::data::dataloader::{DataLoader, DataLoaderBuilder};
use burn::data::dataset::InMemDataset;
use burn::tensor::{Device, Int, Tensor, TensorData};

use crate::exemples::ExempleDEntraînement;

/// Des exemples d'entraînement rangés en tableau : la ligne n de `cibles` est la cible
/// de la ligne n d'`entrées`.
#[derive(Clone, Debug)]
pub struct Lot {
    pub entrées: Tensor<2, Int>,
    pub cibles: Tensor<2, Int>,
}

/// Range les exemples d'une poignée en un lot.
/** # Apprendre
   Un exemple plus court que les autres laisserait des cases vides dans le tableau :
   le numéro de remplissage les comble. Plus tard, au calcul de l'erreur, ces
   positions seront ignorées : le modèle n'en apprend rien.
   Raschka, chapitre 7, section 7.3 (`custom_collate_fn`, `ignore_index`) :
   <https://github.com/rasbt/LLMs-from-scratch/blob/main/ch07/01_main-chapter-code/ch07.ipynb>
*/
struct ExampleBatcher {
    numéro_de_remplissage: usize,
}

/// Construit le tableau d'un lot.
impl ExampleBatcher {
    fn tableau(&self, lignes: Vec<&Vec<usize>>, device: &Device) -> Tensor<2, Int> {
        let longueurs = lignes.iter().map(|ligne| ligne.len());
        let largeur = longueurs.max().unwrap_or(0);
        let hauteur = lignes.len();

        // Burn ne range pas de usize dans un tenseur. u64 garde tous les numéros :
        // un usize tient sur 64 bits au plus, Android compris.
        let numéro_de_remplissage = self.numéro_de_remplissage as u64;
        let mut numéros = Vec::with_capacity(hauteur * largeur);
        for ligne in lignes {
            for &numéro in ligne {
                numéros.push(numéro as u64);
            }
            for _ in ligne.len()..largeur {
                numéros.push(numéro_de_remplissage);
            }
        }

        let données = TensorData::new(numéros, [hauteur, largeur]);
        Tensor::from_data(données, device)
    }
}

/// Permet au `DataLoader` de Burn de ranger chaque poignée en un lot.
impl Batcher<ExempleDEntraînement, Lot> for ExampleBatcher {
    fn batch(&self, exemples: Vec<ExempleDEntraînement>, device: &Device) -> Lot {
        let entrées = exemples.iter().map(|exemple| &exemple.entrée).collect();
        let cibles = exemples.iter().map(|exemple| &exemple.cible).collect();
        Lot {
            entrées: self.tableau(entrées, device),
            cibles: self.tableau(cibles, device),
        }
    }
}

/// Regroupe les exemples d'entraînement en lots, dans l'ordre.
pub fn lots(
    exemples: Vec<ExempleDEntraînement>,
    taille_de_lot: NonZeroUsize,
    numéro_de_remplissage: usize,
) -> Arc<dyn DataLoader<Lot>> {
    let batcher = ExampleBatcher {
        numéro_de_remplissage,
    };
    DataLoaderBuilder::new(batcher)
        .batch_size(taille_de_lot.get())
        .build(InMemDataset::new(exemples))
}
