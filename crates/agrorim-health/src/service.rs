//! Service M4 Santé Végétale — API haut niveau

use crate::diagnostic::{BasePathologies, DiagnosticResultat};
use crate::{Gravite};

/// Service santé végétale
pub struct SanteService {
    base: BasePathologies,
    historique: Vec<DiagnosticResultat>,
}

impl SanteService {
    pub fn new() -> Self {
        Self {
            base: BasePathologies::nouvelle(),
            historique: Vec::new(),
        }
    }

    /// Diagnostiquer une plante
    pub fn diagnostiquer(&mut self, culture: &str, symptomes: &[&str]) -> RapportSante {
        let resultats = self.base.diagnostiquer(culture, symptomes);
        self.historique.extend(resultats.clone());

        RapportSante {
            culture: culture.to_string(),
            symptomes_observes: symptomes.iter().map(|s| s.to_string()).collect(),
            diagnostics: resultats,
        }
    }

    /// Nombre de pathologies dans la base
    pub fn nb_pathologies(&self) -> usize {
        self.base.count()
    }

    pub fn nb_diagnostics(&self) -> usize {
        self.historique.len()
    }
}

/// Rapport de diagnostic complet
#[derive(Debug)]
pub struct RapportSante {
    pub culture: String,
    pub symptomes_observes: Vec<String>,
    pub diagnostics: Vec<DiagnosticResultat>,
}

impl RapportSante {
    pub fn resume(&self) -> String {
        let mut s = String::new();
        s.push_str("=== DIAGNOSTIC SANTÉ VÉGÉTALE ===\n\n");
        s.push_str(&format!("Culture : {}\n", self.culture));
        s.push_str(&format!("Symptômes observés : {}\n\n", self.symptomes_observes.join(", ")));

        if self.diagnostics.is_empty() {
            s.push_str("Aucune pathologie identifiée pour ces symptômes.\n");
            s.push_str("Vérifiez les conditions de culture ou consultez un conseiller.\n");
        } else {
            s.push_str(&format!("{} diagnostic(s) possible(s) :\n\n", self.diagnostics.len()));

            for (i, diag) in self.diagnostics.iter().take(3).enumerate() {
                let urgence = match diag.gravite {
                    Gravite::Critique => "URGENT",
                    Gravite::Forte => "Prioritaire",
                    Gravite::Moderee => "À surveiller",
                    Gravite::Faible => "Préventif",
                };

                s.push_str(&format!(
                    "{}. {} ({}) — Confiance : {:.0}% [{}]\n",
                    i + 1,
                    diag.pathologie.nom,
                    diag.pathologie.categorie.nom(),
                    diag.confiance * 100.0,
                    urgence,
                ));
                s.push_str(&format!("   Traitement bio : {}\n", diag.pathologie.traitement_bio));
                s.push_str(&format!("   Prévention : {}\n", diag.pathologie.prevention));
                s.push_str(&format!("   Source : {}\n\n", diag.pathologie.source));
            }
        }

        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        let service = SanteService::new();
        assert!(service.nb_pathologies() >= 15);
    }

    #[test]
    fn test_diagnostic_complet() {
        let mut service = SanteService::new();
        let rapport = service.diagnostiquer("Tomate", &["taches brunes", "feuilles"]);
        assert!(!rapport.diagnostics.is_empty());
        let resume = rapport.resume();
        assert!(resume.contains("DIAGNOSTIC"));
    }

    #[test]
    fn test_historique() {
        let mut service = SanteService::new();
        service.diagnostiquer("Tomate", &["taches brunes"]);
        service.diagnostiquer("Chou", &["chenilles"]);
        assert!(service.nb_diagnostics() > 0);
    }
}
