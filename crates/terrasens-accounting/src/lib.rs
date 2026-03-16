//! # TERRASENS M7 — Comptabilité Agronomique
//! Coûts réels par parcelle, rendement, marge brute, aide à la décision.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoutParcelle {
    pub intrants_chf: f64,
    pub main_oeuvre_heures: f64,
    pub rendement_kg_ha: f64,
    pub prix_vente_chf_kg: f64,
    pub marge_brute_chf: f64,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        assert!(true);
    }
}
