//! # TERRASENS M2 — Planification des Cultures
//! Rotation, associations, calendrier de semis adapté au sol et au climat local.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Culture {
    pub nom: String,
    pub famille: FamilleBotanique,
    pub duree_jours: u32,
    pub temperature_min: f64,
    pub ph_optimal_min: f64,
    pub ph_optimal_max: f64,
    pub besoin_eau_mm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FamilleBotanique {
    Solanacees,    // Tomate, pomme de terre
    Fabacees,      // Haricot, pois, lentille (fixent l'azote)
    Brassicacees,  // Chou, navet, radis
    Cucurbitacees, // Courge, concombre
    Poacees,       // Blé, maïs, orge
    Apiacees,      // Carotte, persil
    Liliacees,     // Ail, oignon, poireau
    Asteracees,    // Laitue, artichaut
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        assert!(true);
    }
}
