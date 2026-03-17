//! # TERRASENS M5 — Gestion Hydrique
//!
//! Bilan hydrique parcellaire, irrigation optimisée, alertes sécheresse.
//! Intrication ENT-002 : Météo ↔ Eau.
//! Sources : FAO (Penman-Monteith simplifié), Agroscope.

pub mod bilan;
pub mod service;

pub use bilan::*;
pub use service::*;

use serde::{Deserialize, Serialize};

/// Type de sol pour le calcul de réserve utile
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeSolHydrique {
    Sableux,       // RU faible
    Limoneux,      // RU élevée
    Argileux,      // RU moyenne, drainage lent
    LimonoArgileux,
    SabloLimoneux,
}

impl TypeSolHydrique {
    /// Réserve utile en mm par mètre de sol (source FAO)
    pub fn reserve_utile_mm_m(&self) -> f64 {
        match self {
            Self::Sableux => 80.0,
            Self::SabloLimoneux => 120.0,
            Self::Limoneux => 180.0,
            Self::LimonoArgileux => 160.0,
            Self::Argileux => 140.0,
        }
    }

    pub fn nom(&self) -> &str {
        match self {
            Self::Sableux => "Sableux",
            Self::SabloLimoneux => "Sablo-limoneux",
            Self::Limoneux => "Limoneux",
            Self::LimonoArgileux => "Limono-argileux",
            Self::Argileux => "Argileux",
        }
    }
}

/// Stade de stress hydrique
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NiveauStress {
    Aucun,
    Leger,
    Modere,
    Severe,
    Critique,
}

impl NiveauStress {
    pub fn nom(&self) -> &str {
        match self {
            Self::Aucun => "Aucun stress",
            Self::Leger => "Stress léger",
            Self::Modere => "Stress modéré",
            Self::Severe => "Stress sévère",
            Self::Critique => "Stress critique",
        }
    }

    pub fn conseil(&self) -> &str {
        match self {
            Self::Aucun => "Réserve hydrique suffisante. Pas d'irrigation nécessaire.",
            Self::Leger => "Surveiller l'évolution. Pailler pour conserver l'humidité.",
            Self::Modere => "Irrigation recommandée dans les 48h. Priorité aux cultures sensibles.",
            Self::Severe => "Irrigation urgente. Réduire le travail du sol.",
            Self::Critique => "Irrigation immédiate obligatoire. Risque de perte de récolte.",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reserve_utile() {
        assert!(TypeSolHydrique::Limoneux.reserve_utile_mm_m() > TypeSolHydrique::Sableux.reserve_utile_mm_m());
    }

    #[test]
    fn test_stress_conseil() {
        assert!(!NiveauStress::Critique.conseil().is_empty());
        assert!(!NiveauStress::Aucun.nom().is_empty());
    }
}
