//! # TERRASENS M6 — Mémoire Ancestrale
//! Savoirs traditionnels, journal de bord, transmission intergénérationnelle.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavoirAncestral {
    pub titre: String,
    pub description: String,
    pub source_orale: String,
    pub region: String,
    pub fiabilite: f64,
    pub tags: Vec<String>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        assert!(true);
    }
}
