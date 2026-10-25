use std::time::{SystemTime, UNIX_EPOCH};

use moteur::horloge::Horloge;

pub struct SystemClock;

impl Horloge for SystemClock {
    fn timestamp_ms(&self) -> u64 {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(since_unix_epoch) => since_unix_epoch.as_millis() as u64,
            // Une horloge réglée avant 1970 : cas absurde, qui ne doit pas bloquer la frappe.
            Err(_) => 0,
        }
    }
}
