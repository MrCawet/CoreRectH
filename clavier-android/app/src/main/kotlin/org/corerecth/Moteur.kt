package org.corerecth

/**
 * Le moteur Rust, appelé par JNI (voir `lib/native/src/main/rust/src/jni_bridge.rs`).
 *
 * Une saisie est l'adresse de son état côté Rust : elle vit de `commencer` jusqu'à `terminer`.
 */
object Moteur {
    @JvmStatic
    external fun commencer(valeurDuChamp: String): Long

    @JvmStatic
    external fun enregistrer(saisie: Long, texte: String)

    @JvmStatic
    external fun terminer(saisie: Long, cheminDuCorpus: String)
}
