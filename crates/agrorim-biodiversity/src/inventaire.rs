//! Inventaire de biodiversité et calcul d'indices
//!
//! Indice de Shannon-Wiener : mesure de diversité standard en écologie.
//! H' = -Σ(pi × ln(pi)) où pi = proportion de l'espèce i.
//! Source : Shannon & Weaver (1949), adapté OFEV.

use std::collections::HashMap;

use crate::{CategorieBio, ObservationBio, RoleEcologique};

/// Inventaire de biodiversité d'une parcelle
pub struct InventaireBio {
    observations: Vec<ObservationBio>,
}

impl InventaireBio {
    pub fn nouveau() -> Self {
        Self { observations: Vec::new() }
    }

    /// Ajouter une observation
    pub fn observer(&mut self, espece: &str, categorie: CategorieBio, nombre: u32, localisation: &str, date: &str, role: RoleEcologique) {
        self.observations.push(ObservationBio {
            espece: espece.to_string(),
            categorie,
            nombre,
            localisation: localisation.to_string(),
            date: date.to_string(),
            role,
            notes: String::new(),
        });
    }

    /// Nombre total d'individus observés
    pub fn total_individus(&self) -> u32 {
        self.observations.iter().map(|o| o.nombre).sum()
    }

    /// Nombre d'espèces distinctes (richesse spécifique)
    pub fn richesse_specifique(&self) -> usize {
        let mut especes: Vec<&str> = self.observations.iter().map(|o| o.espece.as_str()).collect();
        especes.sort();
        especes.dedup();
        especes.len()
    }

    /// Indice de Shannon-Wiener (H')
    /// Plus H' est élevé, plus la biodiversité est riche.
    /// H' = 0 : une seule espèce
    /// H' > 3.0 : biodiversité excellente
    pub fn indice_shannon(&self) -> f64 {
        let total = self.total_individus() as f64;
        if total == 0.0 {
            return 0.0;
        }

        // Compter par espèce
        let mut compteur: HashMap<&str, u32> = HashMap::new();
        for obs in &self.observations {
            *compteur.entry(&obs.espece).or_insert(0) += obs.nombre;
        }

        let mut h = 0.0;
        for (_, count) in &compteur {
            let pi = *count as f64 / total;
            if pi > 0.0 {
                h -= pi * pi.ln();
            }
        }

        h
    }

    /// Indice d'équitabilité de Piélou (J)
    /// J = H' / ln(S) où S = richesse spécifique
    /// J = 1.0 : toutes les espèces en proportions égales
    pub fn equitabilite(&self) -> f64 {
        let s = self.richesse_specifique() as f64;
        if s <= 1.0 {
            return 0.0;
        }
        self.indice_shannon() / s.ln()
    }

    /// Évaluation qualitative
    pub fn evaluation(&self) -> &str {
        let h = self.indice_shannon();
        if h > 3.0 {
            "Excellente biodiversité"
        } else if h > 2.0 {
            "Bonne biodiversité"
        } else if h > 1.0 {
            "Biodiversité moyenne"
        } else if h > 0.0 {
            "Biodiversité faible"
        } else {
            "Aucune donnée"
        }
    }

    /// Observations par catégorie
    pub fn par_categorie(&self) -> HashMap<CategorieBio, Vec<&ObservationBio>> {
        let mut map: HashMap<CategorieBio, Vec<&ObservationBio>> = HashMap::new();
        for obs in &self.observations {
            map.entry(obs.categorie).or_default().push(obs);
        }
        map
    }

    /// Nombre d'auxiliaires de culture
    pub fn nb_auxiliaires(&self) -> u32 {
        self.observations.iter()
            .filter(|o| o.categorie.est_auxiliaire())
            .map(|o| o.nombre)
            .sum()
    }

    pub fn nb_observations(&self) -> usize {
        self.observations.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn inventaire_test() -> InventaireBio {
        let mut inv = InventaireBio::nouveau();
        inv.observer("Coccinelle à 7 points", CategorieBio::PredateurNaturel, 15, "Haie nord", "2026-06-15", RoleEcologique::PredationRavageurs);
        inv.observer("Abeille domestique", CategorieBio::Pollinisateur, 30, "Bande fleurie", "2026-06-15", RoleEcologique::Pollinisation);
        inv.observer("Mésange bleue", CategorieBio::Oiseau, 4, "Nichoir P1", "2026-06-15", RoleEcologique::PredationRavageurs);
        inv.observer("Ver de terre", CategorieBio::VerDeTerre, 45, "Sol P1", "2026-06-15", RoleEcologique::AerationSol);
        inv.observer("Pissenlit", CategorieBio::PlanteIndicatrice, 20, "Prairie", "2026-06-15", RoleEcologique::IndicateurSante);
        inv.observer("Syrphe", CategorieBio::Pollinisateur, 12, "Bande fleurie", "2026-06-15", RoleEcologique::Pollinisation);
        inv.observer("Carabe doré", CategorieBio::PredateurNaturel, 8, "Sol P2", "2026-06-15", RoleEcologique::PredationRavageurs);
        inv.observer("Hérisson", CategorieBio::Mammifere, 2, "Haie sud", "2026-06-15", RoleEcologique::PredationRavageurs);
        inv
    }

    #[test]
    fn test_richesse_specifique() {
        let inv = inventaire_test();
        assert_eq!(inv.richesse_specifique(), 8);
    }

    #[test]
    fn test_total_individus() {
        let inv = inventaire_test();
        assert_eq!(inv.total_individus(), 136); // 15+30+4+45+20+12+8+2
    }

    #[test]
    fn test_shannon() {
        let inv = inventaire_test();
        let h = inv.indice_shannon();
        // 8 espèces avec distribution inégale → H' ~ 1.5-2.5
        assert!(h > 1.0);
        assert!(h < 3.0);
    }

    #[test]
    fn test_equitabilite() {
        let inv = inventaire_test();
        let j = inv.equitabilite();
        assert!(j > 0.0);
        assert!(j <= 1.0);
    }

    #[test]
    fn test_evaluation() {
        let inv = inventaire_test();
        let eval = inv.evaluation();
        assert!(!eval.is_empty());
    }

    #[test]
    fn test_auxiliaires() {
        let inv = inventaire_test();
        // Coccinelle(15) + Abeille(30) + Ver(45) + Syrphe(12) + Carabe(8) = 110
        assert_eq!(inv.nb_auxiliaires(), 110);
    }

    #[test]
    fn test_shannon_une_espece() {
        let mut inv = InventaireBio::nouveau();
        inv.observer("Espèce A", CategorieBio::Insecte, 100, "P1", "2026-01-01", RoleEcologique::Neutre);
        assert!((inv.indice_shannon() - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_shannon_deux_especes_egales() {
        let mut inv = InventaireBio::nouveau();
        inv.observer("Espèce A", CategorieBio::Insecte, 50, "P1", "2026-01-01", RoleEcologique::Neutre);
        inv.observer("Espèce B", CategorieBio::Insecte, 50, "P1", "2026-01-01", RoleEcologique::Neutre);
        // H' = ln(2) ≈ 0.693
        assert!((inv.indice_shannon() - 2.0_f64.ln()).abs() < 0.01);
    }
}
