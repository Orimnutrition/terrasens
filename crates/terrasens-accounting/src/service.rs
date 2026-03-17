//! Service M7 Comptabilité — API haut niveau

use crate::couts::ComptaParcelle;
use crate::Devise;

#[cfg(test)]
use crate::CategorieCout;

/// Service comptabilité agronomique
pub struct ComptaService {
    parcelles: Vec<ComptaParcelle>,
    devise: Devise,
}

impl ComptaService {
    pub fn new(devise: Devise) -> Self {
        Self {
            parcelles: Vec::new(),
            devise,
        }
    }

    /// Créer une nouvelle parcelle
    pub fn creer_parcelle(&mut self, nom: &str, culture: &str, surface_ha: f64, saison: &str) -> usize {
        let parcelle = ComptaParcelle::nouvelle(nom, culture, surface_ha, saison);
        self.parcelles.push(parcelle);
        self.parcelles.len() - 1
    }

    /// Accéder à une parcelle
    pub fn parcelle(&mut self, index: usize) -> Option<&mut ComptaParcelle> {
        self.parcelles.get_mut(index)
    }

    /// Marge brute totale exploitation
    pub fn marge_brute_totale(&self) -> f64 {
        self.parcelles.iter().map(|p| p.marge_brute()).sum()
    }

    /// Résumé exploitation
    pub fn resume(&self) -> String {
        let mut s = String::new();
        s.push_str("=== COMPTABILITÉ EXPLOITATION ===\n\n");
        s.push_str(&format!("Parcelles : {}\n\n", self.parcelles.len()));

        let mut total_couts = 0.0;
        let mut total_revenus = 0.0;

        for p in &self.parcelles {
            let marge = p.marge_brute();
            let indicateur = if marge >= 0.0 { "+" } else { "" };
            s.push_str(&format!(
                "  {} ({}) : {}{:.2} {} | {:.0} kg/ha\n",
                p.parcelle, p.culture, indicateur, marge, self.devise.symbole(), p.rendement_kg_ha()
            ));
            total_couts += p.total_couts();
            total_revenus += p.total_revenus();
        }

        let total_marge = total_revenus - total_couts;
        s.push_str(&format!("\nTOTAL COÛTS : {:.2} {}\n", total_couts, self.devise.symbole()));
        s.push_str(&format!("TOTAL REVENUS : {:.2} {}\n", total_revenus, self.devise.symbole()));

        let ind = if total_marge >= 0.0 { "+" } else { "" };
        s.push_str(&format!("MARGE EXPLOITATION : {}{:.2} {}\n", ind, total_marge, self.devise.symbole()));

        s
    }

    pub fn nb_parcelles(&self) -> usize {
        self.parcelles.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::couts::CanalVente;

    #[test]
    fn test_service_creation() {
        let service = ComptaService::new(Devise::CHF);
        assert_eq!(service.nb_parcelles(), 0);
    }

    #[test]
    fn test_ajout_parcelle() {
        let mut service = ComptaService::new(Devise::CHF);
        let idx = service.creer_parcelle("P1", "Tomate", 0.1, "2026");
        assert_eq!(idx, 0);
        assert_eq!(service.nb_parcelles(), 1);
    }

    #[test]
    fn test_resume_exploitation() {
        let mut service = ComptaService::new(Devise::CHF);
        let idx = service.creer_parcelle("P1", "Tomate", 0.1, "2026");
        if let Some(p) = service.parcelle(idx) {
            p.ajouter_cout(CategorieCout::Semences, "Graines", 50.0, Devise::CHF);
            p.ajouter_revenu(100.0, 5.0, Devise::CHF, CanalVente::VenteDirecte);
        }
        let resume = service.resume();
        assert!(resume.contains("COMPTABILITÉ EXPLOITATION"));
        assert!(resume.contains("MARGE"));
    }
}
