package org.corerecth

import android.os.Environment
import android.text.InputType
import android.view.inputmethod.EditorInfo
import android.view.inputmethod.ExtractedTextRequest
import android.view.inputmethod.InputConnection
import java.io.File

/**
 * Enregistre chaque saisie dans un champ, de l'ouverture du clavier à sa fermeture,
 * puis l'ajoute au corpus, dans `Documents/CoreRectH/corpus.jsonl`.
 * Les champs de mot de passe ne sont jamais enregistrés.
 */
class EnregistreurDeSaisie {
    private var saisieEnCours: Long? = null
    private var dernierTexte: String = ""

    fun commencer(champ: EditorInfo, connexion: InputConnection?) {
        // Android peut rouvrir le clavier sans l'avoir fermé : la saisie précédente ne doit pas être perdue.
        terminer()
        if (estUnMotDePasse(champ)) {
            return
        }
        val valeurDuChamp = lireLeChamp(connexion)
        saisieEnCours = Moteur.commencer(valeurDuChamp)
        dernierTexte = valeurDuChamp
    }

    fun surChangementDuChamp(connexion: InputConnection?) {
        val saisie = saisieEnCours ?: return
        val texte = lireLeChamp(connexion)
        if (texte == dernierTexte) {
            return
        }
        Moteur.enregistrer(saisie, texte)
        dernierTexte = texte
    }

    fun terminer() {
        val saisie = saisieEnCours ?: return
        Moteur.terminer(saisie, cheminDuCorpus())
        saisieEnCours = null
    }

    private fun estUnMotDePasse(champ: EditorInfo): Boolean {
        val classe = champ.inputType and InputType.TYPE_MASK_CLASS
        val variation = champ.inputType and InputType.TYPE_MASK_VARIATION
        val variationsDeTexteMotDePasse = listOf(
            InputType.TYPE_TEXT_VARIATION_PASSWORD,
            InputType.TYPE_TEXT_VARIATION_VISIBLE_PASSWORD,
            InputType.TYPE_TEXT_VARIATION_WEB_PASSWORD,
        )
        val texteMotDePasse = classe == InputType.TYPE_CLASS_TEXT && variation in variationsDeTexteMotDePasse
        val nombreMotDePasse = classe == InputType.TYPE_CLASS_NUMBER &&
            variation == InputType.TYPE_NUMBER_VARIATION_PASSWORD
        return texteMotDePasse || nombreMotDePasse
    }

    private fun lireLeChamp(connexion: InputConnection?): String {
        val texteExtrait = connexion?.getExtractedText(ExtractedTextRequest(), 0)
        val texte = texteExtrait?.text
        return texte?.toString() ?: ""
    }

    private fun cheminDuCorpus(): String {
        val documents = Environment.getExternalStoragePublicDirectory(Environment.DIRECTORY_DOCUMENTS)
        val dossier = File(documents, "CoreRectH")
        dossier.mkdirs()
        val fichier = File(dossier, "corpus.jsonl")
        return fichier.path
    }
}
