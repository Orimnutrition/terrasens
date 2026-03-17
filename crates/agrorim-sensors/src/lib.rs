//! # AGRORIM Sensors
//!
//! Couche d'abstraction capteurs IoT — MQTT, LoRa, I2C.
//! Chaque capteur est un flux de données typé, géré localement.
//!
//! ## Architecture
//!
//! - [`mqtt`] — Abstraction MQTT simulée (100 % locale, zéro réseau)
//! - [`registre`] — Inventaire et surveillance de l'état des capteurs
//! - [`service`] — API de haut niveau combinant MQTT + registre
//!
//! ## Exemple rapide
//!
//! ```rust
//! use agrorim_sensors::TypeCapteur;
//! use agrorim_sensors::service::CapteurService;
//!
//! let mut service = CapteurService::new();
//! service.ajouter_capteur("T-001", "Temp Serre A", TypeCapteur::Temperature, "Serre A");
//! service.simuler_lecture("T-001", 22.0, "°C");
//!
//! assert_eq!(service.nb_capteurs(), 1);
//! assert_eq!(service.nb_alertes(), 0);
//! ```
//!
//! ## Piliers inviolables respectés
//!
//! 1. Local-first absolu — zéro cloud, zéro TCP
//! 2. Zéro extraction de données
//! 4. Explicabilité totale — alertes en français clair
//! 5. Déconnectabilité garantie — fonctionne sans broker externe

pub mod mqtt;
pub mod registre;
pub mod service;

use serde::{Deserialize, Serialize};
use agrorim_core::Timestamp;

/// Lecture brute émise par un capteur terrain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LectureCapteur {
    /// Identifiant du capteur source
    pub capteur_id: String,
    /// Type de mesure effectuée
    pub type_capteur: TypeCapteur,
    /// Valeur numérique mesurée
    pub valeur: f64,
    /// Unité de la mesure (ex : "°C", "%", "hPa")
    pub unite: String,
    /// Horodatage UTC de la mesure
    pub timestamp: Timestamp,
    /// Niveau de batterie au moment de la mesure (None si alimentation secteur)
    pub batterie_pct: Option<f64>,
}

/// Type de capteur terrain AGRORIM
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TypeCapteur {
    /// Température de l'air ou du sol (°C)
    Temperature,
    /// Humidité relative de l'air (%)
    Humidite,
    /// Humidité volumique du sol (%)
    HumiditeSol,
    /// Potentiel hydrogène du sol
    Ph,
    /// Intensité lumineuse (lux)
    Luminosite,
    /// Pression atmosphérique (hPa)
    Pression,
    /// Vitesse du vent (km/h)
    Vent,
    /// Pluviométrie (mm)
    Pluie,
    /// Concentration en CO2 (ppm)
    Co2,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lecture_capteur_creation() {
        let lecture = LectureCapteur {
            capteur_id: "T-001".to_string(),
            type_capteur: TypeCapteur::Temperature,
            valeur: 22.5,
            unite: "°C".to_string(),
            timestamp: Timestamp::now(),
            batterie_pct: Some(87.0),
        };
        assert_eq!(lecture.capteur_id, "T-001");
        assert!((lecture.valeur - 22.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_type_capteur_serialisation() {
        let type_capteur = TypeCapteur::Ph;
        let json = serde_json::to_string(&type_capteur).unwrap();
        assert_eq!(json, r#""Ph""#);
        let deserialisé: TypeCapteur = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialisé, TypeCapteur::Ph);
    }
}
