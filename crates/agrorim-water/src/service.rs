//! Service M5 Gestion Hydrique — API haut niveau
//! Intrication ENT-002 : Météo ↔ Eau

use crate::bilan::{CalculateurHydrique, CoefficientCultural};
use crate::{NiveauStress, TypeSolHydrique};

/// Service hydrique principal
pub struct HydriqueService {
    calculateur: CalculateurHydrique,
    culture: String,
    kc: CoefficientCultural,
}

impl HydriqueService {
    pub fn new(type_sol: TypeSolHydrique, profondeur_m: f64, culture: &str) -> Self {
        Self {
            calculateur: CalculateurHydrique::nouveau(type_sol, profondeur_m),
            culture: culture.to_string(),
            kc: CoefficientCultural::pour_culture(culture),
        }
    }

    /// Enregistrer un jour (INTRICATION ENT-002 : données météo → bilan hydrique)
    pub fn enregistrer_jour(&mut self, date: &str, precipitation_mm: f64, irrigation_mm: f64, temp_moy: f64) {
        // ETP simplifiée basée sur température
        let etp = (0.2 * temp_moy).max(0.5); // simplification
        self.calculateur.jour(date, precipitation_mm, irrigation_mm, etp, self.kc.kc_plein);
    }

    /// Résumé pour l'agriculteur
    pub fn resume(&self) -> String {
        let mut s = String::new();
        s.push_str("=== BILAN HYDRIQUE AGRORIM ===\n\n");
        s.push_str(&format!("Culture : {}\n", self.culture));
        s.push_str(&format!("Type de sol : {}\n", self.calculateur.type_sol.nom()));
        s.push_str(&format!(
            "Réserve utile : {:.0} / {:.0} mm ({:.0}%)\n",
            self.calculateur.reserve_actuelle(),
            self.calculateur.reserve_max(),
            self.calculateur.taux_remplissage() * 100.0
        ));

        let taux = self.calculateur.taux_remplissage();
        let stress = if taux > 0.6 {
            NiveauStress::Aucun
        } else if taux > 0.4 {
            NiveauStress::Leger
        } else if taux > 0.25 {
            NiveauStress::Modere
        } else if taux > 0.10 {
            NiveauStress::Severe
        } else {
            NiveauStress::Critique
        };

        s.push_str(&format!("\nÉtat : {}\n", stress.nom()));
        s.push_str(&format!("Conseil : {}\n", stress.conseil()));

        let irrig = self.calculateur.irrigation_recommandee();
        if irrig > 0.0 {
            s.push_str(&format!("\nIrrigation recommandée : {:.0} mm\n", irrig));
        }

        s.push_str(&format!("\nJours enregistrés : {}\n", self.calculateur.nb_jours()));
        s
    }

    pub fn irrigation_recommandee(&self) -> f64 {
        self.calculateur.irrigation_recommandee()
    }

    pub fn taux_remplissage(&self) -> f64 {
        self.calculateur.taux_remplissage()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        let service = HydriqueService::new(TypeSolHydrique::Limoneux, 0.6, "Tomate");
        assert!(service.taux_remplissage() > 0.7); // commence à 80%
    }

    #[test]
    fn test_secheresse_progressive() {
        let mut service = HydriqueService::new(TypeSolHydrique::Sableux, 0.3, "Laitue");
        for i in 0..7 {
            service.enregistrer_jour(&format!("2026-07-{:02}", 10 + i), 0.0, 0.0, 28.0);
        }
        assert!(service.taux_remplissage() < 0.8);
        assert!(service.irrigation_recommandee() > 0.0);
    }

    #[test]
    fn test_resume() {
        let mut service = HydriqueService::new(TypeSolHydrique::Limoneux, 0.5, "Carotte");
        service.enregistrer_jour("2026-07-10", 5.0, 0.0, 22.0);
        let resume = service.resume();
        assert!(resume.contains("BILAN HYDRIQUE"));
        assert!(resume.contains("Carotte"));
    }
}
