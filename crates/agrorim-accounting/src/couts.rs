//! Gestion des coûts et revenus par parcelle
//!
//! Marge brute = Revenus - Coûts directs
//! Source : Agroscope — Données de base pour le calcul de la marge brute (2024)

use serde::{Deserialize, Serialize};
use crate::{CategorieCout, Devise};

/// Ligne de coût
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LigneCout {
    pub categorie: CategorieCout,
    pub description: String,
    pub montant: f64,
    pub devise: Devise,
    pub quantite: Option<f64>,
    pub unite: Option<String>,
}

/// Ligne de revenu (récolte vendue)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LigneRevenu {
    pub culture: String,
    pub quantite_kg: f64,
    pub prix_kg: f64,
    pub devise: Devise,
    pub canal_vente: CanalVente,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CanalVente {
    VenteDirecte,
    Marche,
    Cooperative,
    GrossisteGMS,
    Panier,
    Autoconsommation,
}

impl CanalVente {
    pub fn nom(&self) -> &str {
        match self {
            Self::VenteDirecte => "Vente directe",
            Self::Marche => "Marché",
            Self::Cooperative => "Coopérative",
            Self::GrossisteGMS => "Grossiste/GMS",
            Self::Panier => "Panier/AMAP",
            Self::Autoconsommation => "Autoconsommation",
        }
    }
}

/// Comptabilité d'une parcelle
pub struct ComptaParcelle {
    pub parcelle: String,
    pub culture: String,
    pub surface_ha: f64,
    pub saison: String,
    couts: Vec<LigneCout>,
    revenus: Vec<LigneRevenu>,
}

impl ComptaParcelle {
    pub fn nouvelle(parcelle: &str, culture: &str, surface_ha: f64, saison: &str) -> Self {
        Self {
            parcelle: parcelle.to_string(),
            culture: culture.to_string(),
            surface_ha,
            saison: saison.to_string(),
            couts: Vec::new(),
            revenus: Vec::new(),
        }
    }

    /// Ajouter un coût
    pub fn ajouter_cout(&mut self, categorie: CategorieCout, description: &str, montant: f64, devise: Devise) {
        self.couts.push(LigneCout {
            categorie,
            description: description.to_string(),
            montant,
            devise,
            quantite: None,
            unite: None,
        });
    }

    /// Ajouter un revenu
    pub fn ajouter_revenu(&mut self, quantite_kg: f64, prix_kg: f64, devise: Devise, canal: CanalVente) {
        self.revenus.push(LigneRevenu {
            culture: self.culture.clone(),
            quantite_kg,
            prix_kg,
            devise,
            canal_vente: canal,
        });
    }

    /// Total des coûts
    pub fn total_couts(&self) -> f64 {
        self.couts.iter().map(|c| c.montant).sum()
    }

    /// Total des revenus
    pub fn total_revenus(&self) -> f64 {
        self.revenus.iter().map(|r| r.quantite_kg * r.prix_kg).sum()
    }

    /// Marge brute
    pub fn marge_brute(&self) -> f64 {
        self.total_revenus() - self.total_couts()
    }

    /// Marge brute par hectare
    pub fn marge_brute_ha(&self) -> f64 {
        if self.surface_ha > 0.0 {
            self.marge_brute() / self.surface_ha
        } else {
            0.0
        }
    }

    /// Rendement total en kg
    pub fn rendement_kg(&self) -> f64 {
        self.revenus.iter().map(|r| r.quantite_kg).sum()
    }

    /// Rendement par hectare
    pub fn rendement_kg_ha(&self) -> f64 {
        if self.surface_ha > 0.0 {
            self.rendement_kg() / self.surface_ha
        } else {
            0.0
        }
    }

    /// Coûts par catégorie
    pub fn couts_par_categorie(&self) -> Vec<(CategorieCout, f64)> {
        let mut cats: std::collections::HashMap<String, (CategorieCout, f64)> = std::collections::HashMap::new();
        for c in &self.couts {
            let entry = cats.entry(c.categorie.nom().to_string()).or_insert((c.categorie, 0.0));
            entry.1 += c.montant;
        }
        let mut result: Vec<(CategorieCout, f64)> = cats.into_values().collect();
        result.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        result
    }

    /// Rapport financier
    pub fn rapport(&self) -> String {
        let devise = if self.couts.is_empty() && self.revenus.is_empty() {
            "CHF"
        } else if let Some(c) = self.couts.first() {
            c.devise.symbole()
        } else if let Some(r) = self.revenus.first() {
            r.devise.symbole()
        } else {
            "CHF"
        };

        let mut s = String::new();
        s.push_str(&format!("=== COMPTABILITÉ PARCELLE {} ===\n\n", self.parcelle));
        s.push_str(&format!("Culture : {} | Surface : {:.2} ha | Saison : {}\n\n", self.culture, self.surface_ha, self.saison));

        // Coûts par catégorie
        s.push_str("COÛTS :\n");
        for (cat, montant) in self.couts_par_categorie() {
            s.push_str(&format!("  {:<20} {:>8.2} {}\n", cat.nom(), montant, devise));
        }
        s.push_str(&format!("  {:<20} {:>8.2} {}\n\n", "TOTAL COÛTS", self.total_couts(), devise));

        // Revenus
        s.push_str("REVENUS :\n");
        for r in &self.revenus {
            s.push_str(&format!(
                "  {:.0} kg × {:.2} {}/{} ({}) = {:.2} {}\n",
                r.quantite_kg, r.prix_kg, devise, "kg", r.canal_vente.nom(),
                r.quantite_kg * r.prix_kg, devise
            ));
        }
        s.push_str(&format!("  {:<20} {:>8.2} {}\n\n", "TOTAL REVENUS", self.total_revenus(), devise));

        // Marge
        let marge = self.marge_brute();
        let indicateur = if marge > 0.0 { "+" } else { "" };
        s.push_str(&format!("MARGE BRUTE : {}{:.2} {}\n", indicateur, marge, devise));
        s.push_str(&format!("MARGE/HA : {}{:.2} {}/ha\n", indicateur, self.marge_brute_ha(), devise));
        s.push_str(&format!("RENDEMENT : {:.0} kg/ha\n", self.rendement_kg_ha()));

        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parcelle_test() -> ComptaParcelle {
        let mut p = ComptaParcelle::nouvelle("P1", "Tomate", 0.1, "2026");
        p.ajouter_cout(CategorieCout::Semences, "Graines tomate coeur de boeuf", 45.0, Devise::CHF);
        p.ajouter_cout(CategorieCout::Engrais, "Compost 2 tonnes", 120.0, Devise::CHF);
        p.ajouter_cout(CategorieCout::Irrigation, "Goutte-à-goutte saison", 80.0, Devise::CHF);
        p.ajouter_cout(CategorieCout::MainOeuvre, "40h × 25 CHF", 1000.0, Devise::CHF);
        p.ajouter_revenu(500.0, 4.50, Devise::CHF, CanalVente::VenteDirecte);
        p.ajouter_revenu(200.0, 3.00, Devise::CHF, CanalVente::Marche);
        p
    }

    #[test]
    fn test_total_couts() {
        let p = parcelle_test();
        assert!((p.total_couts() - 1245.0).abs() < 0.01);
    }

    #[test]
    fn test_total_revenus() {
        let p = parcelle_test();
        // 500*4.50 + 200*3.00 = 2250 + 600 = 2850
        assert!((p.total_revenus() - 2850.0).abs() < 0.01);
    }

    #[test]
    fn test_marge_brute() {
        let p = parcelle_test();
        assert!(p.marge_brute() > 0.0);
        assert!((p.marge_brute() - 1605.0).abs() < 0.01);
    }

    #[test]
    fn test_rendement_ha() {
        let p = parcelle_test();
        // 700 kg / 0.1 ha = 7000 kg/ha
        assert!((p.rendement_kg_ha() - 7000.0).abs() < 0.01);
    }

    #[test]
    fn test_rapport() {
        let p = parcelle_test();
        let rapport = p.rapport();
        assert!(rapport.contains("COMPTABILITÉ"));
        assert!(rapport.contains("MARGE BRUTE"));
        assert!(rapport.contains("CHF"));
    }

    #[test]
    fn test_couts_par_categorie() {
        let p = parcelle_test();
        let cats = p.couts_par_categorie();
        assert!(!cats.is_empty());
        // Main d'oeuvre devrait être le plus gros poste
        assert_eq!(cats[0].0.nom(), "Main-d'oeuvre");
    }
}
