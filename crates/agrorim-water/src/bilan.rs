//! Bilan hydrique et évapotranspiration
//!
//! ETP simplifiée Hargreaves-Samani (pas besoin de données vent/rayonnement).
//! Source : FAO Irrigation and Drainage Paper 56.

use serde::{Deserialize, Serialize};

use crate::{NiveauStress, TypeSolHydrique};

/// Bilan hydrique journalier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BilanHydrique {
    pub jour: String,
    pub reserve_debut_mm: f64,
    pub precipitation_mm: f64,
    pub irrigation_mm: f64,
    pub etp_mm: f64,
    pub etc_mm: f64,  // ETP * Kc
    pub reserve_fin_mm: f64,
    pub stress: NiveauStress,
}

/// Coefficient cultural (Kc) par stade
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CoefficientCultural {
    pub kc_initial: f64,
    pub kc_developpement: f64,
    pub kc_plein: f64,
    pub kc_maturation: f64,
}

impl CoefficientCultural {
    /// Kc standards FAO pour cultures courantes
    pub fn pour_culture(culture: &str) -> Self {
        match culture.to_lowercase().as_str() {
            "tomate" => Self { kc_initial: 0.60, kc_developpement: 0.85, kc_plein: 1.15, kc_maturation: 0.80 },
            "pomme de terre" => Self { kc_initial: 0.50, kc_developpement: 0.80, kc_plein: 1.15, kc_maturation: 0.75 },
            "haricot" => Self { kc_initial: 0.40, kc_developpement: 0.70, kc_plein: 1.15, kc_maturation: 0.35 },
            "carotte" => Self { kc_initial: 0.70, kc_developpement: 0.85, kc_plein: 1.05, kc_maturation: 0.95 },
            "laitue" => Self { kc_initial: 0.70, kc_developpement: 0.85, kc_plein: 1.00, kc_maturation: 0.95 },
            "chou" => Self { kc_initial: 0.70, kc_developpement: 0.85, kc_plein: 1.05, kc_maturation: 0.95 },
            "courgette" => Self { kc_initial: 0.50, kc_developpement: 0.80, kc_plein: 1.00, kc_maturation: 0.75 },
            "blé" => Self { kc_initial: 0.30, kc_developpement: 0.70, kc_plein: 1.15, kc_maturation: 0.25 },
            _ => Self { kc_initial: 0.50, kc_developpement: 0.75, kc_plein: 1.00, kc_maturation: 0.70 },
        }
    }
}

/// Calculateur de bilan hydrique
pub struct CalculateurHydrique {
    pub type_sol: TypeSolHydrique,
    pub profondeur_racinaire_m: f64,
    reserve_actuelle_mm: f64,
    reserve_max_mm: f64,
    historique: Vec<BilanHydrique>,
}

impl CalculateurHydrique {
    pub fn nouveau(type_sol: TypeSolHydrique, profondeur_m: f64) -> Self {
        let reserve_max = type_sol.reserve_utile_mm_m() * profondeur_m;
        Self {
            type_sol,
            profondeur_racinaire_m: profondeur_m,
            reserve_actuelle_mm: reserve_max * 0.8, // début à 80%
            reserve_max_mm: reserve_max,
            historique: Vec::new(),
        }
    }

    /// ETP Hargreaves-Samani (mm/jour)
    /// Nécessite seulement temp min, max et latitude
    pub fn etp_hargreaves(temp_min: f64, temp_max: f64, temp_moy: f64, jour_julien: u32, latitude_deg: f64) -> f64 {
        // Rayonnement extraterrestre simplifié (Ra)
        let lat_rad = latitude_deg.to_radians();
        let declinaison = 0.409 * ((2.0 * std::f64::consts::PI * jour_julien as f64 / 365.0) - 1.39).sin();
        let omega = ((-lat_rad.tan() * declinaison.tan()).max(-1.0).min(1.0)).acos();

        let dr = 1.0 + 0.033 * (2.0 * std::f64::consts::PI * jour_julien as f64 / 365.0).cos();
        let ra = (24.0 * 60.0 / std::f64::consts::PI) * 0.0820 * dr
            * (omega * lat_rad.sin() * declinaison.sin()
               + lat_rad.cos() * declinaison.cos() * omega.sin());

        // Hargreaves-Samani
        let krs = 0.17; // coefficient pour zone continentale
        let amplitude = (temp_max - temp_min).max(0.0);
        0.0023 * (temp_moy + 17.8) * amplitude.sqrt() * ra * krs
    }

    /// Calculer le bilan d'un jour
    pub fn jour(&mut self, date: &str, precipitation_mm: f64, irrigation_mm: f64, etp_mm: f64, kc: f64) -> BilanHydrique {
        let reserve_debut = self.reserve_actuelle_mm;
        let etc = etp_mm * kc;

        let apports = precipitation_mm + irrigation_mm;
        let mut reserve_fin = reserve_debut + apports - etc;

        // Limiter entre 0 et max
        if reserve_fin > self.reserve_max_mm {
            reserve_fin = self.reserve_max_mm;
        }
        if reserve_fin < 0.0 {
            reserve_fin = 0.0;
        }

        let taux_remplissage = reserve_fin / self.reserve_max_mm;
        let stress = if taux_remplissage > 0.6 {
            NiveauStress::Aucun
        } else if taux_remplissage > 0.4 {
            NiveauStress::Leger
        } else if taux_remplissage > 0.25 {
            NiveauStress::Modere
        } else if taux_remplissage > 0.10 {
            NiveauStress::Severe
        } else {
            NiveauStress::Critique
        };

        self.reserve_actuelle_mm = reserve_fin;

        let bilan = BilanHydrique {
            jour: date.to_string(),
            reserve_debut_mm: reserve_debut,
            precipitation_mm,
            irrigation_mm,
            etp_mm,
            etc_mm: etc,
            reserve_fin_mm: reserve_fin,
            stress,
        };

        self.historique.push(bilan.clone());
        bilan
    }

    /// Irrigation recommandée pour ramener à 80% de la RU
    pub fn irrigation_recommandee(&self) -> f64 {
        let cible = self.reserve_max_mm * 0.8;
        (cible - self.reserve_actuelle_mm).max(0.0)
    }

    pub fn reserve_actuelle(&self) -> f64 {
        self.reserve_actuelle_mm
    }

    pub fn reserve_max(&self) -> f64 {
        self.reserve_max_mm
    }

    pub fn taux_remplissage(&self) -> f64 {
        self.reserve_actuelle_mm / self.reserve_max_mm
    }

    pub fn nb_jours(&self) -> usize {
        self.historique.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kc_tomate() {
        let kc = CoefficientCultural::pour_culture("Tomate");
        assert!(kc.kc_plein > kc.kc_initial);
        assert!((kc.kc_plein - 1.15).abs() < 0.01);
    }

    #[test]
    fn test_etp_hargreaves() {
        // Genève, mi-juillet, jour 196
        let etp = CalculateurHydrique::etp_hargreaves(15.0, 30.0, 22.5, 196, 46.2);
        // ETP été Genève ~ 4-6 mm/jour
        assert!(etp > 1.0);
        assert!(etp < 10.0);
    }

    #[test]
    fn test_bilan_journalier() {
        let mut calc = CalculateurHydrique::nouveau(TypeSolHydrique::Limoneux, 0.6);
        let reserve_init = calc.reserve_actuelle();

        let bilan = calc.jour("2026-07-15", 0.0, 0.0, 5.0, 1.15);
        assert!(bilan.reserve_fin_mm < reserve_init);
        assert_eq!(bilan.stress, NiveauStress::Aucun);
    }

    #[test]
    fn test_stress_hydrique() {
        let mut calc = CalculateurHydrique::nouveau(TypeSolHydrique::Sableux, 0.3);
        // Sol sableux peu profond = faible réserve
        // Simuler 10 jours sans pluie avec forte ETP
        for i in 0..10 {
            calc.jour(&format!("jour-{}", i), 0.0, 0.0, 5.0, 1.0);
        }
        // Devrait être en stress
        assert!(calc.taux_remplissage() < 0.5);
    }

    #[test]
    fn test_irrigation_recommandee() {
        let mut calc = CalculateurHydrique::nouveau(TypeSolHydrique::Limoneux, 0.6);
        // Vider un peu la réserve
        calc.jour("j1", 0.0, 0.0, 30.0, 1.0);
        let irrig = calc.irrigation_recommandee();
        assert!(irrig > 0.0);
    }

    #[test]
    fn test_pluie_remplit() {
        let mut calc = CalculateurHydrique::nouveau(TypeSolHydrique::Limoneux, 0.6);
        calc.jour("j1", 0.0, 0.0, 50.0, 1.0); // vider
        let avant = calc.reserve_actuelle();
        calc.jour("j2", 30.0, 0.0, 2.0, 1.0); // pluie
        assert!(calc.reserve_actuelle() > avant);
    }
}
