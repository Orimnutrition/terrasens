//! # TERRASENS M4 — Santé Végétale
//!
//! Diagnostic maladies et ravageurs.
//! Base de connaissances : 15+ pathologies courantes Suisse/Europe.
//! Sources : Agroscope, INRAE, GRAB.

pub mod diagnostic;
pub mod service;

pub use diagnostic::*;
pub use service::*;

use serde::{Deserialize, Serialize};

/// Catégorie de problème phytosanitaire
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CategorieProbleme {
    Fongique,
    Bacterien,
    Viral,
    Insecte,
    Acarien,
    Nematode,
    Carence,
    Physiologique,
}

impl CategorieProbleme {
    pub fn nom(&self) -> &str {
        match self {
            Self::Fongique => "Fongique",
            Self::Bacterien => "Bactérien",
            Self::Viral => "Viral",
            Self::Insecte => "Insecte",
            Self::Acarien => "Acarien",
            Self::Nematode => "Nématode",
            Self::Carence => "Carence",
            Self::Physiologique => "Physiologique",
        }
    }
}

/// Gravité du problème
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Gravite {
    Faible,
    Moderee,
    Forte,
    Critique,
}

impl Gravite {
    pub fn nom(&self) -> &str {
        match self {
            Self::Faible => "Faible",
            Self::Moderee => "Modérée",
            Self::Forte => "Forte",
            Self::Critique => "Critique",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_categorie_nom() {
        assert_eq!(CategorieProbleme::Fongique.nom(), "Fongique");
        assert_eq!(CategorieProbleme::Insecte.nom(), "Insecte");
    }

    #[test]
    fn test_gravite_ordre() {
        assert!(Gravite::Faible < Gravite::Critique);
        assert!(Gravite::Moderee < Gravite::Forte);
    }
}
