# Correctifs pour FlorisBoard

Les bugs de FlorisBoard trouvés et corrigés dans cette copie, à proposer au dépôt officiel ([florisboard/florisboard](https://github.com/florisboard/florisboard)).

Avant de proposer quoi que ce soit, lire leur [AI_POLICY.md](AI_POLICY.md) : le code, l'issue, la PR et les messages de commit doivent être écrits par une personne, pas par une IA. Ce journal sert à comprendre le bug, pas à être copié dans une issue ou une PR.

Leurs règles de contribution : [CONTRIBUTING.md](CONTRIBUTING.md) (en anglais, issue avec leur modèle de rapport de bug, prévenir avant de coder).

## Le curseur ignoré dans Firefox

**État** : corrigé dans cette copie, à vérifier sur le téléphone. Aucune issue ouverte sur le dépôt officiel (vérifié le 2026-10-29).

### Ce qu'on voit

Vu dans Firefox, dans un champ de mot de passe. Probablement le même bug sur la recherche de acode.app (navigateur non confirmé) :

- une lettre tapée s'ajoute toujours à la fin du texte, même avec le curseur au début ou au milieu, et le curseur saute à la fin ;
- la croix (effacer) ne fait rien, ou déplace le curseur sans rien effacer ;
- après « tout sélectionner » et « couper », d'anciennes lettres reviennent à la frappe suivante.

Seul moyen de s'en sortir : « tout sélectionner », puis « couper ».

### La cause

Le nouveau moteur de FlorisBoard (k3lp, `ime/keyboard3/`) garde sa propre copie du texte avant le curseur, et la position du curseur. À chaque touche, il réécrit tout ce bloc dans le champ à partir de sa copie (`ImeEditor.replaceText`).

Il ne met cette copie à jour que dans `FlorisImeService.onUpdateCursorAnchorInfo`. Or une application n'est pas obligée d'envoyer ces informations : Firefox ne les envoie pas. Le journal de débogage le montre : dans le champ de Firefox, aucune ligne `new=…`, alors que le curseur a été déplacé.

Le clavier croit donc que le curseur n'a jamais bougé. Dans un champ de mot de passe, Android ne lui donne pas non plus le texte déjà présent : sa copie est vide, et la croix n'a rien à effacer.

Android appelle pourtant toujours `onUpdateSelection` quand le curseur ou le texte changent, quelle que soit l'application. FlorisBoard ne s'en sert pas.

### Le correctif

`FlorisImeService.onUpdateSelection` transmet aussi la nouvelle sélection au moteur, comme `onUpdateCursorAnchorInfo` (`handleUpdateSelection`), avec la même trace `new=…`.

Quand une application envoie les deux, la deuxième annonce de la même sélection est ignorée par la déduplication existante (`ExpectedContentQueue`) : rien ne change pour elle.

### Reste à vérifier

- Sur le téléphone, dans Firefox : le curseur déplacé est respecté, la croix efface, et le journal de débogage montre des lignes `new=…`.
- Que rien ne change dans une application qui envoie déjà `onUpdateCursorAnchorInfo` (WhatsApp, par exemple).
