//! Types fondamentaux TERRASENS

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Identifiant unique TERRASENS — jamais exposé à l'extérieur
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TerraId(Uuid);

impl TerraId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Default for TerraId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for TerraId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Horodatage TERRASENS — toujours UTC, converti localement à l'affichage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timestamp(DateTime<Utc>);

impl Timestamp {
    pub fn now() -> Self {
        Self(Utc::now())
    }

    pub fn as_datetime(&self) -> &DateTime<Utc> {
        &self.0
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
}

/// Identifiant de parcelle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParcelleId(TerraId);

impl ParcelleId {
    pub fn new() -> Self {
        Self(TerraId::new())
    }
}

impl Default for ParcelleId {
    fn default() -> Self {
        Self::new()
    }
}

/// Identifiant d'agriculteur
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgriculteurId(TerraId);

impl AgriculteurId {
    pub fn new() -> Self {
        Self(TerraId::new())
    }
}

impl Default for AgriculteurId {
    fn default() -> Self {
        Self::new()
    }
}

/// Parcelle agricole — unité fondamentale TERRASENS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parcelle {
    pub id: ParcelleId,
    pub nom: String,
    pub superficie_hectares: f64,
    pub localisation: GeoPoint,
    pub altitude_metres: f64,
    pub proprietaire: AgriculteurId,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

/// Point géographique WGS84
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GeoPoint {
    pub latitude: f64,
    pub longitude: f64,
}

impl GeoPoint {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self { latitude, longitude }
    }

    /// Distance en mètres (formule de Haversine)
    pub fn distance_to(&self, other: &GeoPoint) -> f64 {
        let r = 6_371_000.0; // rayon Terre en mètres
        let d_lat = (other.latitude - self.latitude).to_radians();
        let d_lon = (other.longitude - self.longitude).to_radians();
        let a = (d_lat / 2.0).sin().powi(2)
            + self.latitude.to_radians().cos()
                * other.latitude.to_radians().cos()
                * (d_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().asin();
        r * c
    }
}

/// Saison agricole
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Saison {
    Printemps,
    Ete,
    Automne,
    Hiver,
}

/// Niveau de priorité
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Priorite {
    Basse,
    Moyenne,
    Haute,
    Critique,
}

/// Niveau de confiance d'une recommandation
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Confiance(f64);

impl Confiance {
    /// Crée un niveau de confiance entre 0.0 et 1.0
    pub fn new(value: f64) -> Self {
        Self(value.clamp(0.0, 1.0))
    }

    pub fn value(&self) -> f64 {
        self.0
    }

    pub fn is_high(&self) -> bool {
        self.0 >= 0.8
    }

    pub fn is_low(&self) -> bool {
        self.0 < 0.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_terra_id_unique() {
        let id1 = TerraId::new();
        let id2 = TerraId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_geopoint_distance() {
        // Genève → Zurich ≈ 225 km
        let geneve = GeoPoint::new(46.2044, 6.1432);
        let zurich = GeoPoint::new(47.3769, 8.5417);
        let distance = geneve.distance_to(&zurich);
        assert!((distance - 225_000.0).abs() < 10_000.0); // ±10km de tolérance
    }

    #[test]
    fn test_confiance_clamp() {
        let c = Confiance::new(1.5);
        assert!((c.value() - 1.0).abs() < f64::EPSILON);
        let c2 = Confiance::new(-0.5);
        assert!((c2.value()).abs() < f64::EPSILON);
    }

    #[test]
    fn test_confiance_levels() {
        assert!(Confiance::new(0.9).is_high());
        assert!(!Confiance::new(0.5).is_high());
        assert!(Confiance::new(0.3).is_low());
    }
}
