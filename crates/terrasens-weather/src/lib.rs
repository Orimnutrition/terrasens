//! # TERRASENS M3 — Météo Hyper-locale
//!
//! Prévisions météo à l'échelle de la parcelle.
//! Sources : capteurs locaux, stations météo, modèles statistiques.
//! Zéro cloud — tout tourne en local.

use serde::{Deserialize, Serialize};
use terrasens_core::{Celsius, GeoPoint, Millimetres, Timestamp};

/// Observation météo ponctuelle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationMeteo {
    pub timestamp: Timestamp,
    pub position: GeoPoint,
    pub temperature: Celsius,
    pub humidite_pct: f64,
    pub precipitation_mm: Millimetres,
    pub vent_kmh: f64,
    pub direction_vent_deg: f64,
    pub pression_hpa: f64,
    pub source: SourceMeteo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceMeteo {
    CapteurLocal(String),
    StationProche(String),
    Estimation,
}

/// Alerte météo pour l'agriculteur
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlerteMeteo {
    pub type_alerte: TypeAlerte,
    pub message: String,
    pub debut: Timestamp,
    pub severite: u8, // 1-5
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeAlerte {
    Gel,
    Canicule,
    Orage,
    Grele,
    Secheresse,
    VentFort,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_observation_creation() {
        let obs = ObservationMeteo {
            timestamp: Timestamp::now(),
            position: GeoPoint::new(46.2, 6.1),
            temperature: Celsius::new(15.0),
            humidite_pct: 65.0,
            precipitation_mm: Millimetres::new(0.0).unwrap(),
            vent_kmh: 12.0,
            direction_vent_deg: 270.0,
            pression_hpa: 1013.25,
            source: SourceMeteo::CapteurLocal("BME280-001".into()),
        };
        assert!((obs.temperature.value() - 15.0).abs() < f64::EPSILON);
    }
}
