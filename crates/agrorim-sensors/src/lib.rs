//! # AGRORIM Sensors
//! Couche d'abstraction capteurs IoT — MQTT, LoRa, I2C.
//! Chaque capteur est un flux de données typé.

use serde::{Deserialize, Serialize};
use agrorim_core::Timestamp;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LectureCapteur {
    pub capteur_id: String,
    pub type_capteur: TypeCapteur,
    pub valeur: f64,
    pub unite: String,
    pub timestamp: Timestamp,
    pub batterie_pct: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeCapteur {
    Temperature,
    Humidite,
    HumiditeSol,
    Ph,
    Luminosite,
    Pression,
    Vent,
    Pluie,
    Co2,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        assert!(true);
    }
}
