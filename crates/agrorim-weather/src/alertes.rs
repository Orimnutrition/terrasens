//! Système d'alertes météo pour l'agriculteur
//!
//! Détection automatique : gel, canicule, grêle, sécheresse, vent fort.
//! Sources : Agroscope, MétéoSuisse, seuils INRAE.

use serde::{Deserialize, Serialize};
use agrorim_core::Timestamp;

/// Type d'alerte météo
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeAlerte {
    Gel,
    GelPrintanier,
    Canicule,
    Orage,
    Grele,
    Secheresse,
    VentFort,
    Brouillard,
}

impl TypeAlerte {
    pub fn nom(&self) -> &str {
        match self {
            Self::Gel => "Gel",
            Self::GelPrintanier => "Gel printanier",
            Self::Canicule => "Canicule",
            Self::Orage => "Orage",
            Self::Grele => "Grêle",
            Self::Secheresse => "Sécheresse",
            Self::VentFort => "Vent fort",
            Self::Brouillard => "Brouillard",
        }
    }

    pub fn conseil(&self) -> &str {
        match self {
            Self::Gel | Self::GelPrintanier => "Protéger les jeunes plants avec un voile d'hivernage. Éviter d'arroser le matin.",
            Self::Canicule => "Pailler abondamment, arroser le soir au pied, ombrager les plants fragiles.",
            Self::Orage => "Tuteurer les plants hauts, vérifier le drainage, rentrer le matériel.",
            Self::Grele => "Installer des filets anti-grêle si disponibles. Récolter les fruits mûrs en urgence.",
            Self::Secheresse => "Réduire le travail du sol, pailler, prioriser l'irrigation des cultures sensibles.",
            Self::VentFort => "Tuteurer, protéger les serres, reporter les traitements foliaires.",
            Self::Brouillard => "Surveiller le mildiou et l'oïdium. Aérer les serres dès que possible.",
        }
    }
}

/// Sévérité de 1 (veille) à 5 (critique)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Severite(u8);

impl Severite {
    pub fn new(val: u8) -> Self {
        Self(val.min(5).max(1))
    }

    pub fn value(&self) -> u8 {
        self.0
    }

    pub fn label(&self) -> &str {
        match self.0 {
            1 => "Veille",
            2 => "Attention",
            3 => "Alerte",
            4 => "Danger",
            _ => "Critique",
        }
    }
}

/// Alerte météo pour l'agriculteur
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlerteMeteo {
    pub type_alerte: TypeAlerte,
    pub severite: Severite,
    pub message: String,
    pub debut: Timestamp,
    pub duree_heures: u32,
    pub source: String,
}

/// Détecteur d'alertes basé sur les observations
pub struct DetecteurAlertes;

impl DetecteurAlertes {
    /// Analyser une série de températures et générer des alertes
    pub fn analyser_temperatures(temperatures: &[(Timestamp, f64)], mois: u32) -> Vec<AlerteMeteo> {
        let mut alertes = Vec::new();

        if temperatures.is_empty() {
            return alertes;
        }

        let temp_min = temperatures.iter().map(|(_, t)| *t).fold(f64::INFINITY, f64::min);
        let temp_max = temperatures.iter().map(|(_, t)| *t).fold(f64::NEG_INFINITY, f64::max);

        // Gel
        if temp_min < 0.0 {
            let severite = if temp_min < -5.0 {
                Severite::new(4)
            } else if temp_min < -2.0 {
                Severite::new(3)
            } else {
                Severite::new(2)
            };

            let type_alerte = if (3..=5).contains(&mois) {
                TypeAlerte::GelPrintanier
            } else {
                TypeAlerte::Gel
            };

            alertes.push(AlerteMeteo {
                type_alerte,
                severite,
                message: format!(
                    "Température minimale de {:.1}°C détectée. {}",
                    temp_min,
                    type_alerte.conseil()
                ),
                debut: temperatures[0].0.clone(),
                duree_heures: 12,
                source: "AGRORIM — Analyse locale".into(),
            });
        }

        // Canicule (3 jours > 33°C)
        if temp_max > 33.0 {
            let jours_chauds = temperatures.iter().filter(|(_, t)| *t > 33.0).count();
            if jours_chauds >= 3 {
                alertes.push(AlerteMeteo {
                    type_alerte: TypeAlerte::Canicule,
                    severite: Severite::new(if temp_max > 38.0 { 4 } else { 3 }),
                    message: format!(
                        "Canicule : {} jours au-dessus de 33°C (max {:.1}°C). {}",
                        jours_chauds,
                        temp_max,
                        TypeAlerte::Canicule.conseil()
                    ),
                    debut: temperatures[0].0.clone(),
                    duree_heures: jours_chauds as u32 * 24,
                    source: "AGRORIM — Analyse locale".into(),
                });
            }
        }

        alertes
    }

    /// Analyser les précipitations pour détecter sécheresse
    pub fn analyser_precipitations(precip_mm_semaine: &[f64]) -> Option<AlerteMeteo> {
        if precip_mm_semaine.len() < 3 {
            return None;
        }

        // Sécheresse : moins de 5mm sur 3 semaines consécutives
        let dernieres_3 = &precip_mm_semaine[precip_mm_semaine.len().saturating_sub(3)..];
        let total_3_semaines: f64 = dernieres_3.iter().sum();

        if total_3_semaines < 5.0 {
            let severite = if total_3_semaines < 1.0 {
                Severite::new(4)
            } else {
                Severite::new(3)
            };

            Some(AlerteMeteo {
                type_alerte: TypeAlerte::Secheresse,
                severite,
                message: format!(
                    "Sécheresse : seulement {:.1}mm sur les 3 dernières semaines. {}",
                    total_3_semaines,
                    TypeAlerte::Secheresse.conseil()
                ),
                debut: Timestamp::now(),
                duree_heures: 0,
                source: "AGRORIM — Bilan pluviométrique".into(),
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_severite_clamp() {
        assert_eq!(Severite::new(0).value(), 1);
        assert_eq!(Severite::new(10).value(), 5);
        assert_eq!(Severite::new(3).value(), 3);
    }

    #[test]
    fn test_detection_gel() {
        let temps: Vec<(Timestamp, f64)> = vec![
            (Timestamp::now(), -3.0),
            (Timestamp::now(), -1.5),
            (Timestamp::now(), 2.0),
        ];
        let alertes = DetecteurAlertes::analyser_temperatures(&temps, 1);
        assert!(!alertes.is_empty());
        assert!(alertes.iter().any(|a| a.type_alerte == TypeAlerte::Gel));
    }

    #[test]
    fn test_gel_printanier() {
        let temps = vec![(Timestamp::now(), -1.0)];
        let alertes = DetecteurAlertes::analyser_temperatures(&temps, 4);
        assert!(alertes.iter().any(|a| a.type_alerte == TypeAlerte::GelPrintanier));
    }

    #[test]
    fn test_pas_dalerte_temps_doux() {
        let temps = vec![
            (Timestamp::now(), 15.0),
            (Timestamp::now(), 18.0),
        ];
        let alertes = DetecteurAlertes::analyser_temperatures(&temps, 6);
        assert!(alertes.is_empty());
    }

    #[test]
    fn test_secheresse() {
        let precip = vec![1.0, 0.5, 0.0]; // 1.5mm en 3 semaines
        let alerte = DetecteurAlertes::analyser_precipitations(&precip);
        assert!(alerte.is_some());
        assert_eq!(alerte.unwrap().type_alerte, TypeAlerte::Secheresse);
    }

    #[test]
    fn test_pas_de_secheresse() {
        let precip = vec![10.0, 15.0, 8.0];
        assert!(DetecteurAlertes::analyser_precipitations(&precip).is_none());
    }

    #[test]
    fn test_conseil_existe() {
        for t in [TypeAlerte::Gel, TypeAlerte::Canicule, TypeAlerte::Grele, TypeAlerte::Secheresse] {
            assert!(!t.conseil().is_empty());
            assert!(!t.nom().is_empty());
        }
    }
}
