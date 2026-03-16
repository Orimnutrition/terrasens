//! # TERRASENS M8 — Biodiversité et Écosystème
//! Inventaire espèces, corridors écologiques, indice de biodiversité.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObservationBiodiversite {
    pub espece: String,
    pub categorie: CategorieBio,
    pub nombre: u32,
    pub localisation: String,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CategorieBio {
    Insecte,
    Oiseau,
    Mammifere,
    Reptile,
    Amphibien,
    PlanteIndicatrice,
    Champignon,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        assert!(true);
    }
}
