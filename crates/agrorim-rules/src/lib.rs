//! # AGRORIM Rules Engine
//!
//! Moteur de règles agronomiques explicables.
//! Chaque règle a une source scientifique et une explication en langage simple.
//!
//! ## Principes
//! - Toute recommandation est traçable (source)
//! - Toute recommandation est compréhensible (explication)
//! - Aucune boîte noire

use serde::{Deserialize, Serialize};
use agrorim_core::{Confiance, Priorite};

/// Règle agronomique explicable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegleAgronomique {
    /// Identifiant unique de la règle
    pub id: String,
    /// Nom lisible
    pub nom: String,
    /// Module concerné (M1-M8)
    pub module: String,
    /// Condition déclenchante (texte)
    pub condition: String,
    /// Recommandation
    pub recommandation: String,
    /// Explication en langage simple
    pub explication: String,
    /// Source scientifique
    pub source: String,
    /// Priorité
    pub priorite: Priorite,
    /// Confiance
    pub confiance: Confiance,
}

/// Résultat d'évaluation d'une règle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultatRegle {
    pub regle: RegleAgronomique,
    pub declenchee: bool,
    pub details: String,
}

/// Catalogue de règles agronomiques
#[derive(Debug, Default)]
pub struct CatalogueRegles {
    regles: Vec<RegleAgronomique>,
}

impl CatalogueRegles {
    pub fn new() -> Self {
        Self { regles: Vec::new() }
    }

    pub fn ajouter(&mut self, regle: RegleAgronomique) {
        self.regles.push(regle);
    }

    pub fn par_module(&self, module: &str) -> Vec<&RegleAgronomique> {
        self.regles.iter().filter(|r| r.module == module).collect()
    }

    pub fn count(&self) -> usize {
        self.regles.len()
    }

    /// Charge les règles sol par défaut (basées sur la littérature agronomique)
    pub fn charger_regles_sol() -> Self {
        let mut catalogue = Self::new();

        catalogue.ajouter(RegleAgronomique {
            id: "SOL-PH-001".into(),
            nom: "pH trop acide".into(),
            module: "M1".into(),
            condition: "pH < 5.5".into(),
            recommandation: "Apporter de la chaux (CaCO3) à raison de 2-4 t/ha".into(),
            explication: "Votre sol est trop acide. Les plantes ont du mal à absorber les nutriments. \
                         Un chaulage permettra de remonter le pH et d'améliorer la disponibilité des éléments.".into(),
            source: "Agroscope — Guide de fertilisation des grandes cultures (2017)".into(),
            priorite: Priorite::Haute,
            confiance: Confiance::new(0.95),
        });

        catalogue.ajouter(RegleAgronomique {
            id: "SOL-PH-002".into(),
            nom: "pH trop basique".into(),
            module: "M1".into(),
            condition: "pH > 8.0".into(),
            recommandation: "Apporter du soufre élémentaire (50-100 kg/ha) ou du compost acide".into(),
            explication: "Votre sol est trop basique. Le fer, le manganèse et le zinc deviennent \
                         indisponibles pour les plantes. Un apport de matière organique acide aidera.".into(),
            source: "INRAE — Référentiel pédologique (2008)".into(),
            priorite: Priorite::Moyenne,
            confiance: Confiance::new(0.90),
        });

        catalogue.ajouter(RegleAgronomique {
            id: "SOL-MO-001".into(),
            nom: "Matière organique insuffisante".into(),
            module: "M1".into(),
            condition: "Matière organique < 2%".into(),
            recommandation: "Apporter 10-20 t/ha de compost mûr ou envisager un engrais vert".into(),
            explication: "Votre sol manque de matière organique. C'est le moteur de la vie du sol : \
                         elle nourrit les vers, les champignons et les bactéries qui rendent les \
                         nutriments disponibles pour vos cultures.".into(),
            source: "Agroscope — Bilan humique des sols cultivés (2019)".into(),
            priorite: Priorite::Haute,
            confiance: Confiance::new(0.92),
        });

        catalogue.ajouter(RegleAgronomique {
            id: "SOL-N-001".into(),
            nom: "Azote déficient".into(),
            module: "M1".into(),
            condition: "N total < 1.5 g/kg".into(),
            recommandation: "Implanter une légumineuse (trèfle, luzerne) comme engrais vert".into(),
            explication: "L'azote est insuffisant dans votre sol. Plutôt qu'un engrais chimique, \
                         une légumineuse fixe l'azote de l'air gratuitement grâce à ses bactéries \
                         symbiotiques (rhizobium).".into(),
            source: "Fiches techniques GRAB — Engrais verts (2020)".into(),
            priorite: Priorite::Haute,
            confiance: Confiance::new(0.88),
        });

        catalogue.ajouter(RegleAgronomique {
            id: "SOL-P-001".into(),
            nom: "Phosphore déficient".into(),
            module: "M1".into(),
            condition: "P Olsen < 15 mg/kg".into(),
            recommandation: "Apporter de la poudre de roche phosphatée ou du compost riche en P".into(),
            explication: "Le phosphore est bas. Il est essentiel pour l'enracinement et la floraison. \
                         La poudre de roche se libère lentement et nourrit le sol sur plusieurs années.".into(),
            source: "Arvalis — Guide fertilisation phospho-potassique (2021)".into(),
            priorite: Priorite::Moyenne,
            confiance: Confiance::new(0.85),
        });

        catalogue.ajouter(RegleAgronomique {
            id: "SOL-K-001".into(),
            nom: "Potassium déficient".into(),
            module: "M1".into(),
            condition: "K échangeable < 100 mg/kg".into(),
            recommandation: "Apporter des cendres de bois (2-5 t/ha) ou du patentkali".into(),
            explication: "Le potassium est trop bas. Il régule l'eau dans la plante et renforce \
                         sa résistance aux maladies et au gel. Les cendres de bois sont une source \
                         naturelle excellente.".into(),
            source: "Agroscope — Normes de fertilisation K (2018)".into(),
            priorite: Priorite::Moyenne,
            confiance: Confiance::new(0.87),
        });

        catalogue.ajouter(RegleAgronomique {
            id: "SOL-CEC-001".into(),
            nom: "CEC très basse".into(),
            module: "M1".into(),
            condition: "CEC < 10 meq/100g".into(),
            recommandation: "Sol sableux pauvre : augmenter la matière organique, pailler abondamment".into(),
            explication: "La capacité d'échange de votre sol est faible — il ne retient pas bien \
                         les nutriments. En ajoutant de la matière organique et un paillage permanent, \
                         vous créez une 'éponge à nutriments' naturelle.".into(),
            source: "INRAE — Propriétés physico-chimiques des sols (2015)".into(),
            priorite: Priorite::Haute,
            confiance: Confiance::new(0.90),
        });

        catalogue
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalogue_chargement() {
        let catalogue = CatalogueRegles::charger_regles_sol();
        assert!(catalogue.count() >= 7);
    }

    #[test]
    fn test_filtrage_par_module() {
        let catalogue = CatalogueRegles::charger_regles_sol();
        let regles_m1 = catalogue.par_module("M1");
        assert_eq!(regles_m1.len(), catalogue.count());
    }

    #[test]
    fn test_regle_a_source() {
        let catalogue = CatalogueRegles::charger_regles_sol();
        for regle in catalogue.par_module("M1") {
            assert!(!regle.source.is_empty(), "Règle {} sans source !", regle.id);
            assert!(
                !regle.explication.is_empty(),
                "Règle {} sans explication !",
                regle.id
            );
        }
    }
}
