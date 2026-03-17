//! Service M8 Biodiversité — API haut niveau

use crate::inventaire::InventaireBio;
use crate::{CategorieBio, RoleEcologique};

/// Service biodiversité
pub struct BiodiversiteService {
    inventaire: InventaireBio,
    parcelle: String,
}

impl BiodiversiteService {
    pub fn new(parcelle: &str) -> Self {
        Self {
            inventaire: InventaireBio::nouveau(),
            parcelle: parcelle.to_string(),
        }
    }

    /// Enregistrer une observation
    pub fn observer(&mut self, espece: &str, categorie: CategorieBio, nombre: u32, role: RoleEcologique) {
        self.inventaire.observer(espece, categorie, nombre, &self.parcelle, "", role);
    }

    /// Résumé complet pour l'agriculteur
    pub fn resume(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("=== BIODIVERSITÉ — {} ===\n\n", self.parcelle));

        s.push_str(&format!("Espèces observées : {}\n", self.inventaire.richesse_specifique()));
        s.push_str(&format!("Individus totaux : {}\n", self.inventaire.total_individus()));
        s.push_str(&format!("Auxiliaires de culture : {}\n\n", self.inventaire.nb_auxiliaires()));

        let h = self.inventaire.indice_shannon();
        let j = self.inventaire.equitabilite();
        s.push_str(&format!("Indice de Shannon (H') : {:.2}\n", h));
        s.push_str(&format!("Équitabilité (J) : {:.2}\n", j));
        s.push_str(&format!("Évaluation : {}\n\n", self.inventaire.evaluation()));

        // Répartition par catégorie
        let par_cat = self.inventaire.par_categorie();
        if !par_cat.is_empty() {
            s.push_str("Par catégorie :\n");
            let mut cats: Vec<_> = par_cat.iter().collect();
            cats.sort_by(|a, b| {
                let sum_a: u32 = a.1.iter().map(|o| o.nombre).sum();
                let sum_b: u32 = b.1.iter().map(|o| o.nombre).sum();
                sum_b.cmp(&sum_a)
            });
            for (cat, obs) in &cats {
                let total: u32 = obs.iter().map(|o| o.nombre).sum();
                let especes: Vec<_> = obs.iter().map(|o| o.espece.as_str()).collect();
                s.push_str(&format!("  {} : {} individus ({})\n", cat.nom(), total, especes.join(", ")));
            }
        }

        // Conseils
        s.push_str("\nConseils :\n");
        if h < 1.5 {
            s.push_str("  - Installer des bandes fleuries pour attirer les pollinisateurs\n");
            s.push_str("  - Planter des haies diversifiées pour les oiseaux et prédateurs\n");
            s.push_str("  - Réduire le travail du sol pour préserver les vers de terre\n");
        } else if h < 2.5 {
            s.push_str("  - Maintenir les habitats existants (haies, bandes fleuries)\n");
            s.push_str("  - Installer des nichoirs pour les mésanges et rapaces\n");
        } else {
            s.push_str("  - Excellente biodiversité ! Continuer les pratiques actuelles.\n");
            s.push_str("  - Documenter vos pratiques pour les partager avec d'autres.\n");
        }

        s
    }

    pub fn indice_shannon(&self) -> f64 {
        self.inventaire.indice_shannon()
    }

    pub fn nb_especes(&self) -> usize {
        self.inventaire.richesse_specifique()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        let service = BiodiversiteService::new("P1");
        assert_eq!(service.nb_especes(), 0);
    }

    #[test]
    fn test_observation() {
        let mut service = BiodiversiteService::new("P1");
        service.observer("Coccinelle", CategorieBio::PredateurNaturel, 10, RoleEcologique::PredationRavageurs);
        service.observer("Abeille", CategorieBio::Pollinisateur, 20, RoleEcologique::Pollinisation);
        assert_eq!(service.nb_especes(), 2);
    }

    #[test]
    fn test_resume() {
        let mut service = BiodiversiteService::new("Parcelle Nord");
        service.observer("Coccinelle", CategorieBio::PredateurNaturel, 10, RoleEcologique::PredationRavageurs);
        service.observer("Abeille", CategorieBio::Pollinisateur, 20, RoleEcologique::Pollinisation);
        service.observer("Mésange", CategorieBio::Oiseau, 5, RoleEcologique::PredationRavageurs);

        let resume = service.resume();
        assert!(resume.contains("BIODIVERSITÉ"));
        assert!(resume.contains("Shannon"));
        assert!(resume.contains("Parcelle Nord"));
    }
}
