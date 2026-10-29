//! Les fonctions que le clavier Kotlin appelle (`org.corerecth.Moteur`).
//!
//! Une saisie en cours vit côté Rust : Kotlin n'en garde que l'adresse, un `Long`,
//! de `commencer` jusqu'à `terminer`, qui la libère.

use std::path::PathBuf;

use jni::objects::{JClass, JString};
use jni::sys::jlong;
use jni::JNIEnv;
use moteur::saisie::Saisie;

use crate::json_lines_corpus::JsonLinesCorpus;
use crate::system_clock::SystemClock;

/// Ce que Kotlin reçoit quand aucune saisie n'a pu commencer.
const NO_SAISIE: jlong = 0;

#[no_mangle]
pub extern "system" fn Java_org_corerecth_Moteur_commencer(
    mut env: JNIEnv,
    _class: JClass,
    valeur_du_champ: JString,
) -> jlong {
    let Ok(valeur_du_champ) = env.get_string(&valeur_du_champ) else {
        return NO_SAISIE;
    };
    let valeur_du_champ: String = valeur_du_champ.into();
    let saisie = Saisie::commencer(&valeur_du_champ, Box::new(SystemClock));
    let boxed_saisie = Box::new(saisie);
    Box::into_raw(boxed_saisie) as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_corerecth_Moteur_enregistrer(
    mut env: JNIEnv,
    _class: JClass,
    saisie: jlong,
    texte: JString,
) {
    if saisie == NO_SAISIE {
        return;
    }
    let Ok(texte) = env.get_string(&texte) else {
        return;
    };
    let texte: String = texte.into();
    // SAFETY: l'adresse vient de `commencer` et n'a pas encore été rendue à `terminer`.
    let saisie = unsafe { &mut *(saisie as *mut Saisie) };
    saisie.enregistrer(&texte);
}

#[no_mangle]
pub extern "system" fn Java_org_corerecth_Moteur_terminer(
    mut env: JNIEnv,
    _class: JClass,
    saisie: jlong,
    chemin_du_corpus: JString,
) {
    if saisie == NO_SAISIE {
        return;
    }
    // SAFETY: l'adresse vient de `commencer`, et Kotlin l'oublie après cet appel.
    let saisie = unsafe { Box::from_raw(saisie as *mut Saisie) };
    let Ok(chemin_du_corpus) = env.get_string(&chemin_du_corpus) else {
        return;
    };
    let chemin_du_corpus: String = chemin_du_corpus.into();
    let mut corpus = JsonLinesCorpus::new(PathBuf::from(chemin_du_corpus));
    saisie.terminer(&mut corpus);
}
