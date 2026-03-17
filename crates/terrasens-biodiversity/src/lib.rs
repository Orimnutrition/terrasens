//! # TERRASENS M8 — Biodiversité et Écosystème
//!
//! Inventaire espèces, corridors écologiques, indice de biodiversité.
//! Intrication ENT-003 : Santé ↔ Biodiversité.
//! Source : OFEV (Office fédéral de l'environnement), INRAE.

pub mod inventaire;
pub mod service;

pub use inventaire::*;
pub use service::*;

use serde::{Deserialize, Serialize};

/// Catégorie biologique
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CategorieBio {
    Insecte,
    Oiseau,
    Mammifere,
    Reptile,
    Amphibien,
    PlanteIndicatrice,
    Champignon,
    VerDeTerre,
    Pollinisateur,
    PredateurNaturel,
}

impl CategorieBio {
    pub fn nom(&self) -> &str {
        match self {
            Self::Insecte => "Insecte",
            Self::Oiseau => "Oiseau",
            Self::Mammifere => "Mammifère",
            Self::Reptile => "Reptile",
            Self::Amphibien => "Amphibien",
            Self::PlanteIndicatrice => "Plante indicatrice",
            Self::Champignon => "Champignon",
            Self::VerDeTerre => "Ver de terre",
            Self::Pollinisateur => "Pollinisateur",
            Self::PredateurNaturel => "Prédateur naturel",
        }
    }

    pub fn est_auxiliaire(&self) -> bool {
        matches!(self, Self::Pollinisateur | Self::PredateurNaturel | Self::VerDeTerre)
    }
}

/// Rôle écologique
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoleEcologique {
    Pollinisation,
    PredationRavageurs,
    DecompositionMO,
    AerationSol,
    IndicateurSante,
    HabitatRefuge,
    FixationAzote,
    Neutre,
}

impl RoleEcologique {
    pub fn nom(&self) -> &str {
        match self {
            Self::Pollinisation => "Pollinisation",
            Self::PredationRavageurs => "Prédation des ravageurs",
            Self::DecompositionMO => "Décomposition MO",
            Self::AerationSol => "Aération du sol",
            Self::IndicateurSante => "Indicateur de santé",
            Self::HabitatRefuge => "Habitat refuge",
            Self::FixationAzote => "Fixation d'azote",
            Self::Neutre => "Neutre",
        }
    }
}

/// Observation de biodiversité
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationBio {
    pub espece: String,
    pub categorie: CategorieBio,
    pub nombre: u32,
    pub localisation: String,
    pub date: String,
    pub role: RoleEcologique,
    pub notes: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_categorie_nom() {
        assert_eq!(CategorieBio::Insecte.nom(), "Insecte");
        assert_eq!(CategorieBio::VerDeTerre.nom(), "Ver de terre");
    }

    #[test]
    fn test_auxiliaires() {
        assert!(CategorieBio::Pollinisateur.est_auxiliaire());
        assert!(CategorieBio::PredateurNaturel.est_auxiliaire());
        assert!(!CategorieBio::Reptile.est_auxiliaire());
    }
}
