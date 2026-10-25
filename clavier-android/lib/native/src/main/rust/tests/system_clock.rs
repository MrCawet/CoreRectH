use std::time::{SystemTime, UNIX_EPOCH};

use fl_native_rust::system_clock::SystemClock;
use moteur::horloge::Horloge;

fn maintenant_ms() -> u64 {
    let depuis_l_époque_unix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("l'horloge du système est avant 1970");
    depuis_l_époque_unix.as_millis() as u64
}

#[test]
fn l_horloge_système_donne_l_heure_actuelle_en_ms_depuis_l_époque_unix() {
    let horloge = SystemClock;

    let avant_la_lecture = maintenant_ms();
    let lecture = horloge.timestamp_ms();
    let après_la_lecture = maintenant_ms();

    assert!(avant_la_lecture <= lecture);
    assert!(lecture <= après_la_lecture);
}
