//! Modèles de données sol

use serde::{Deserialize, Serialize};
use terrasens_core::{Confiance, MgParKg, ParcelleId, Ph, Pourcentage, Priorite, TerraId, Timestamp};

/// Analyse de sol complète
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyseSol {
    pub id: TerraId,
    pub parcelle_id: ParcelleId,
    pub date_prelevement: Timestamp,
    pub date_analyse: Timestamp,
    pub source: SourceAnalyse,
    pub profondeur_cm: u32,

    // Paramètres physico-chimiques
    pub ph: Option<Ph>,
    pub matiere_organique: Option<Pourcentage>,
    pub azote_total: Option<MgParKg>,
    pub phosphore_olsen: Option<MgParKg>,
    pub potassium_echangeable: Option<MgParKg>,
    pub calcium: Option<MgParKg>,
    pub magnesium: Option<MgParKg>,
    pub cec: Option<f64>, // meq/100g
    pub rapport_cn: Option<f64>,

    // Texture
    pub argile: Option<Pourcentage>,
    pub limon: Option<Pourcentage>,
    pub sable: Option<Pourcentage>,

    // Métadonnées
    pub laboratoire: Option<String>,
    pub notes: Option<String>,
}

impl AnalyseSol {
    /// Crée une nouvelle analyse vide pour une parcelle
    pub fn nouvelle(parcelle_id: ParcelleId) -> Self {
        Self {
            id: TerraId::new(),
            parcelle_id,
            date_prelevement: Timestamp::now(),
            date_analyse: Timestamp::now(),
            source: SourceAnalyse::ManuelleAgriculteur,
            profondeur_cm: 30,
            ph: None,
            matiere_organique: None,
            azote_total: None,
            phosphore_olsen: None,
            potassium_echangeable: None,
            calcium: None,
            magnesium: None,
            cec: None,
            rapport_cn: None,
            argile: None,
            limon: None,
            sable: None,
            laboratoire: None,
            notes: None,
        }
    }

    /// Classe texturale du sol (triangle des textures simplifié)
    pub fn classe_texturale(&self) -> Option<ClasseTexturale> {
        match (self.argile, self.limon, self.sable) {
            (Some(argile), Some(limon), Some(sable)) => {
                let a = argile.value();
                let l = limon.value();
                let s = sable.value();

                // Vérification cohérence (somme ≈ 100%)
                if (a + l + s - 100.0).abs() > 5.0 {
                    return None;
                }

                Some(if a > 40.0 {
                    ClasseTexturale::Argileux
                } else if s > 65.0 {
                    ClasseTexturale::Sableux
                } else if l > 50.0 {
                    ClasseTexturale::Limoneux
                } else if a > 25.0 && s > 20.0 {
                    ClasseTexturale::ArgileuxSableux
                } else if a > 25.0 && l > 25.0 {
                    ClasseTexturale::ArgileuxLimoneux
                } else {
                    ClasseTexturale::Equilibre
                })
            }
            _ => None,
        }
    }

    /// Complétude de l'analyse (% de champs remplis)
    pub fn completude(&self) -> f64 {
        let total = 12;
        let mut remplis = 0;
        if self.ph.is_some() { remplis += 1; }
        if self.matiere_organique.is_some() { remplis += 1; }
        if self.azote_total.is_some() { remplis += 1; }
        if self.phosphore_olsen.is_some() { remplis += 1; }
        if self.potassium_echangeable.is_some() { remplis += 1; }
        if self.calcium.is_some() { remplis += 1; }
        if self.magnesium.is_some() { remplis += 1; }
        if self.cec.is_some() { remplis += 1; }
        if self.rapport_cn.is_some() { remplis += 1; }
        if self.argile.is_some() { remplis += 1; }
        if self.limon.is_some() { remplis += 1; }
        if self.sable.is_some() { remplis += 1; }
        (remplis as f64 / total as f64) * 100.0
    }
}

/// Source de l'analyse de sol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceAnalyse {
    /// Saisie manuelle par l'agriculteur
    ManuelleAgriculteur,
    /// Import depuis un laboratoire (PDF/CSV)
    Laboratoire(String),
    /// Capteur terrain en temps réel
    Capteur(String),
    /// Estimation par modèle ML
    ModeleML { confiance: Confiance },
}

/// Classification texturale simplifiée
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClasseTexturale {
    Argileux,
    ArgileuxLimoneux,
    ArgileuxSableux,
    Limoneux,
    Sableux,
    Equilibre,
}

impl ClasseTexturale {
    pub fn description(&self) -> &'static str {
        match self {
            Self::Argileux => "Sol argileux — retient bien l'eau et les nutriments, mais peut être lourd à travailler",
            Self::ArgileuxLimoneux => "Sol argilo-limoneux — bon équilibre, fertile, facile à travailler",
            Self::ArgileuxSableux => "Sol argilo-sableux — drainage correct, bonne structure",
            Self::Limoneux => "Sol limoneux — très fertile, attention à la battance (croûte en surface)",
            Self::Sableux => "Sol sableux — se réchauffe vite, mais retient peu l'eau et les nutriments",
            Self::Equilibre => "Sol équilibré — texture idéale pour la plupart des cultures",
        }
    }
}

/// Recommandation issue de l'analyse de sol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommandationSol {
    pub parametre: String,
    pub valeur_actuelle: String,
    pub valeur_optimale: String,
    pub action: String,
    pub explication: String,
    pub source: String,
    pub priorite: Priorite,
    pub confiance: Confiance,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nouvelle_analyse() {
        let parcelle = ParcelleId::new();
        let analyse = AnalyseSol::nouvelle(parcelle);
        assert_eq!(analyse.profondeur_cm, 30);
        assert!(analyse.ph.is_none());
    }

    #[test]
    fn test_classe_texturale() {
        let parcelle = ParcelleId::new();
        let mut analyse = AnalyseSol::nouvelle(parcelle);
        analyse.argile = Pourcentage::new(50.0);
        analyse.limon = Pourcentage::new(30.0);
        analyse.sable = Pourcentage::new(20.0);
        assert_eq!(analyse.classe_texturale(), Some(ClasseTexturale::Argileux));
    }

    #[test]
    fn test_classe_texturale_sableux() {
        let parcelle = ParcelleId::new();
        let mut analyse = AnalyseSol::nouvelle(parcelle);
        analyse.argile = Pourcentage::new(5.0);
        analyse.limon = Pourcentage::new(10.0);
        analyse.sable = Pourcentage::new(85.0);
        assert_eq!(analyse.classe_texturale(), Some(ClasseTexturale::Sableux));
    }

    #[test]
    fn test_completude() {
        let parcelle = ParcelleId::new();
        let mut analyse = AnalyseSol::nouvelle(parcelle);
        assert!((analyse.completude()).abs() < f64::EPSILON);

        analyse.ph = Ph::new(6.5);
        analyse.matiere_organique = Pourcentage::new(3.5);
        let completude = analyse.completude();
        assert!((completude - (2.0 / 12.0 * 100.0)).abs() < 0.1);
    }
}
