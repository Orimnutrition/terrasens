//! # AGRORIM M6 — Mémoire Ancestrale
//!
//! Savoirs traditionnels, journal de bord, transmission intergénérationnelle.
//! Chaque savoir est documenté avec sa source orale et sa fiabilité.

pub mod journal;
pub mod savoirs;
pub mod service;

pub use journal::*;
pub use savoirs::*;
pub use service::*;

use serde::{Deserialize, Serialize};

/// Catégorie de savoir
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CategorieSavoir {
    Semis,
    Recolte,
    Meteo,
    Sol,
    Protection,
    Conservation,
    Transformation,
    Elevage,
    Lune,
    General,
}

impl CategorieSavoir {
    pub fn nom(&self) -> &str {
        match self {
            Self::Semis => "Semis",
            Self::Recolte => "Récolte",
            Self::Meteo => "Météo",
            Self::Sol => "Sol",
            Self::Protection => "Protection des cultures",
            Self::Conservation => "Conservation",
            Self::Transformation => "Transformation",
            Self::Elevage => "Élevage",
            Self::Lune => "Calendrier lunaire",
            Self::General => "Général",
        }
    }
}

/// Savoir ancestral documenté
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavoirAncestral {
    pub titre: String,
    pub description: String,
    pub categorie: CategorieSavoir,
    pub source_orale: String,
    pub region: String,
    pub fiabilite: f64,
    pub validation_scientifique: Option<String>,
    pub tags: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_categorie_nom() {
        assert_eq!(CategorieSavoir::Semis.nom(), "Semis");
        assert_eq!(CategorieSavoir::Lune.nom(), "Calendrier lunaire");
    }
}
