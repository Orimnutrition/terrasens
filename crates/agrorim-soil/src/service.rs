//! Service M1 Sol — API haut niveau pour l'interface

use crate::analysis::{AnalyseurSol, RapportSol};
use crate::models::AnalyseSol;
use agrorim_core::{ParcelleId, Ph, Pourcentage, TerraResult};

/// Service principal du module Sol
pub struct SolService {
    analyseur: AnalyseurSol,
    historique: Vec<AnalyseSol>,
}

impl SolService {
    pub fn new() -> Self {
        Self {
            analyseur: AnalyseurSol::new(),
            historique: Vec::new(),
        }
    }

    /// Crée et analyse un échantillon de sol rapidement
    /// Usage : saisie terrain avec le minimum vital
    pub fn analyse_rapide(
        &mut self,
        parcelle_id: ParcelleId,
        ph: f64,
        matiere_organique: f64,
    ) -> TerraResult<RapportSol> {
        let mut sol = AnalyseSol::nouvelle(parcelle_id);
        sol.ph = Ph::new(ph);
        sol.matiere_organique = Pourcentage::new(matiere_organique);

        let rapport = self.analyseur.analyser(&sol);
        self.historique.push(sol);
        Ok(rapport)
    }

    /// Analyse complète avec tous les paramètres
    pub fn analyse_complete(&mut self, sol: AnalyseSol) -> RapportSol {
        let rapport = self.analyseur.analyser(&sol);
        self.historique.push(sol);
        rapport
    }

    /// Nombre d'analyses dans l'historique
    pub fn count_analyses(&self) -> usize {
        self.historique.len()
    }

    /// Tendance pH sur les N dernières analyses d'une parcelle
    pub fn tendance_ph(&self, _parcelle_id: &ParcelleId) -> Vec<f64> {
        self.historique
            .iter()
            .filter_map(|a| a.ph.map(|ph| ph.value()))
            .collect()
    }
}

impl Default for SolService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyse_rapide() {
        let mut service = SolService::new();
        let rapport = service
            .analyse_rapide(ParcelleId::new(), 5.0, 1.5)
            .unwrap();

        assert!(!rapport.recommandations.is_empty());
        assert_eq!(service.count_analyses(), 1);
    }

    #[test]
    fn test_historique() {
        let mut service = SolService::new();
        let parcelle = ParcelleId::new();

        service.analyse_rapide(parcelle.clone(), 5.0, 1.5).unwrap();
        service.analyse_rapide(parcelle.clone(), 5.5, 2.0).unwrap();
        service.analyse_rapide(parcelle.clone(), 6.0, 2.5).unwrap();

        assert_eq!(service.count_analyses(), 3);
        let tendance = service.tendance_ph(&parcelle);
        assert_eq!(tendance.len(), 3);
    }
}
