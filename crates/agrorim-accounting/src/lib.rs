//! # AGRORIM M7 — Comptabilité Agronomique
//!
//! Coûts réels par parcelle, rendement, marge brute, aide à la décision.
//! Intrication ENT-004 : Mémoire ↔ Comptabilité.
//! Adapté au contexte suisse (CHF) et européen (EUR).

pub mod couts;
pub mod service;

pub use couts::*;
pub use service::*;

use serde::{Deserialize, Serialize};

/// Devise
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Devise {
    CHF,
    EUR,
}

impl Devise {
    pub fn symbole(&self) -> &str {
        match self {
            Self::CHF => "CHF",
            Self::EUR => "EUR",
        }
    }
}

/// Catégorie de coût
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CategorieCout {
    Semences,
    Plants,
    Engrais,
    Amendement,
    Phytosanitaire,
    Irrigation,
    MainOeuvre,
    Materiel,
    Energie,
    Certification,
    Autre,
}

impl CategorieCout {
    pub fn nom(&self) -> &str {
        match self {
            Self::Semences => "Semences",
            Self::Plants => "Plants",
            Self::Engrais => "Engrais",
            Self::Amendement => "Amendement",
            Self::Phytosanitaire => "Phytosanitaire",
            Self::Irrigation => "Irrigation",
            Self::MainOeuvre => "Main-d'oeuvre",
            Self::Materiel => "Matériel",
            Self::Energie => "Énergie",
            Self::Certification => "Certification",
            Self::Autre => "Autre",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_devise() {
        assert_eq!(Devise::CHF.symbole(), "CHF");
        assert_eq!(Devise::EUR.symbole(), "EUR");
    }

    #[test]
    fn test_categorie() {
        assert_eq!(CategorieCout::Semences.nom(), "Semences");
    }
}
