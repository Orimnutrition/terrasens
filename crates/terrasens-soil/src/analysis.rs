//! Moteur d'analyse de sol — évalue les paramètres et génère des recommandations

use serde::{Deserialize, Serialize};
use terrasens_core::{Confiance, Priorite};
use terrasens_rules::CatalogueRegles;

use crate::models::{AnalyseSol, RecommandationSol};

/// Analyseur de sol — coeur du module M1
pub struct AnalyseurSol {
    catalogue: CatalogueRegles,
}

impl AnalyseurSol {
    pub fn new() -> Self {
        Self {
            catalogue: CatalogueRegles::charger_regles_sol(),
        }
    }

    /// Analyse complète d'un échantillon de sol
    pub fn analyser(&self, sol: &AnalyseSol) -> RapportSol {
        let mut recommandations = Vec::new();
        let mut regles_declenchees = Vec::new();

        // Évaluation pH
        if let Some(ph) = &sol.ph {
            if ph.value() < 5.5 {
                recommandations.push(RecommandationSol {
                    parametre: "pH".into(),
                    valeur_actuelle: format!("{}", ph),
                    valeur_optimale: "6.0 — 7.0".into(),
                    action: "Chaulage : 2-4 t/ha de CaCO3".into(),
                    explication: "Votre sol est trop acide. Les plantes peinent à absorber \
                                 les nutriments, surtout le phosphore et le calcium. Un chaulage \
                                 corrigera cela en 6 à 12 mois.".into(),
                    source: "Agroscope — Guide de fertilisation (2017)".into(),
                    priorite: Priorite::Haute,
                    confiance: Confiance::new(0.95),
                });
                regles_declenchees.push("SOL-PH-001".into());
            } else if ph.value() > 8.0 {
                recommandations.push(RecommandationSol {
                    parametre: "pH".into(),
                    valeur_actuelle: format!("{}", ph),
                    valeur_optimale: "6.0 — 7.5".into(),
                    action: "Soufre élémentaire : 50-100 kg/ha".into(),
                    explication: "Votre sol est trop basique. Le fer et le zinc deviennent \
                                 indisponibles. Un apport de soufre ou de compost acide aidera.".into(),
                    source: "INRAE — Référentiel pédologique (2008)".into(),
                    priorite: Priorite::Moyenne,
                    confiance: Confiance::new(0.90),
                });
                regles_declenchees.push("SOL-PH-002".into());
            }
        }

        // Évaluation matière organique
        if let Some(mo) = &sol.matiere_organique {
            if mo.value() < 2.0 {
                recommandations.push(RecommandationSol {
                    parametre: "Matière organique".into(),
                    valeur_actuelle: format!("{}", mo),
                    valeur_optimale: "> 3%".into(),
                    action: "Compost mûr : 10-20 t/ha + engrais vert".into(),
                    explication: "Votre sol manque de vie. La matière organique nourrit les \
                                 organismes du sol qui rendent les nutriments disponibles. C'est \
                                 la base de tout.".into(),
                    source: "Agroscope — Bilan humique (2019)".into(),
                    priorite: Priorite::Haute,
                    confiance: Confiance::new(0.92),
                });
                regles_declenchees.push("SOL-MO-001".into());
            }
        }

        // Évaluation phosphore
        if let Some(p) = &sol.phosphore_olsen {
            if p.value() < 15.0 {
                recommandations.push(RecommandationSol {
                    parametre: "Phosphore (Olsen)".into(),
                    valeur_actuelle: format!("{}", p),
                    valeur_optimale: "20 — 40 mg/kg".into(),
                    action: "Roche phosphatée naturelle ou compost enrichi en P".into(),
                    explication: "Le phosphore est trop bas pour un bon enracinement et une \
                                 bonne floraison. La roche phosphatée se libère sur 3-5 ans.".into(),
                    source: "Arvalis — Guide fertilisation P-K (2021)".into(),
                    priorite: Priorite::Moyenne,
                    confiance: Confiance::new(0.85),
                });
                regles_declenchees.push("SOL-P-001".into());
            }
        }

        // Évaluation potassium
        if let Some(k) = &sol.potassium_echangeable {
            if k.value() < 100.0 {
                recommandations.push(RecommandationSol {
                    parametre: "Potassium".into(),
                    valeur_actuelle: format!("{}", k),
                    valeur_optimale: "150 — 300 mg/kg".into(),
                    action: "Cendres de bois : 2-5 t/ha".into(),
                    explication: "Le potassium régule l'eau dans la plante et renforce sa \
                                 résistance au gel et aux maladies. Les cendres de bois sont \
                                 une excellente source gratuite.".into(),
                    source: "Agroscope — Normes K (2018)".into(),
                    priorite: Priorite::Moyenne,
                    confiance: Confiance::new(0.87),
                });
                regles_declenchees.push("SOL-K-001".into());
            }
        }

        // Évaluation CEC
        if let Some(cec) = sol.cec {
            if cec < 10.0 {
                recommandations.push(RecommandationSol {
                    parametre: "CEC".into(),
                    valeur_actuelle: format!("{:.1} meq/100g", cec),
                    valeur_optimale: "> 15 meq/100g".into(),
                    action: "Matière organique abondante + paillage permanent".into(),
                    explication: "Votre sol est comme un seau percé : les nutriments le \
                                 traversent sans être retenus. En ajoutant de la matière \
                                 organique, vous bouchez les trous.".into(),
                    source: "INRAE — Propriétés physico-chimiques (2015)".into(),
                    priorite: Priorite::Haute,
                    confiance: Confiance::new(0.90),
                });
                regles_declenchees.push("SOL-CEC-001".into());
            }
        }

        // Calcul du score global
        let score = self.calculer_score(sol);

        // Calcul classe texturale
        let texture = sol.classe_texturale();

        RapportSol {
            analyse_id: sol.id.to_string(),
            score_global: score,
            texture,
            recommandations,
            regles_declenchees,
            completude: sol.completude(),
        }
    }

    /// Score global de santé du sol (0-100)
    fn calculer_score(&self, sol: &AnalyseSol) -> f64 {
        let mut score = 100.0;
        let mut facteurs = 0;

        if let Some(ph) = &sol.ph {
            let v = ph.value();
            // Optimum : 6.0-7.0
            let penalite = if v < 6.0 {
                (6.0 - v) * 10.0
            } else if v > 7.0 {
                (v - 7.0) * 8.0
            } else {
                0.0
            };
            score -= penalite;
            facteurs += 1;
        }

        if let Some(mo) = &sol.matiere_organique {
            // Optimum : > 3%
            if mo.value() < 3.0 {
                score -= (3.0 - mo.value()) * 15.0;
            }
            facteurs += 1;
        }

        if let Some(p) = &sol.phosphore_olsen {
            if p.value() < 15.0 {
                score -= (15.0 - p.value()) * 1.0;
            }
            facteurs += 1;
        }

        if let Some(k) = &sol.potassium_echangeable {
            if k.value() < 100.0 {
                score -= (100.0 - k.value()) * 0.2;
            }
            facteurs += 1;
        }

        if facteurs == 0 {
            return 0.0; // Pas assez de données
        }

        score.clamp(0.0, 100.0)
    }
}

impl Default for AnalyseurSol {
    fn default() -> Self {
        Self::new()
    }
}

/// Rapport d'analyse de sol complet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RapportSol {
    pub analyse_id: String,
    pub score_global: f64,
    pub texture: Option<crate::models::ClasseTexturale>,
    pub recommandations: Vec<RecommandationSol>,
    pub regles_declenchees: Vec<String>,
    pub completude: f64,
}

impl RapportSol {
    /// Génère un résumé en langage simple pour l'agriculteur
    pub fn resume_agriculteur(&self) -> String {
        let mut resume = String::new();

        // Score global
        let etat = match self.score_global as u32 {
            80..=100 => "Excellent",
            60..=79 => "Bon",
            40..=59 => "Moyen — des améliorations sont possibles",
            20..=39 => "Préoccupant — actions recommandées",
            _ => "Critique — intervention urgente",
        };

        resume.push_str(&format!(
            "=== RAPPORT DE SOL ===\nScore : {:.0}/100 — {}\n\n",
            self.score_global, etat
        ));

        // Texture
        if let Some(texture) = &self.texture {
            resume.push_str(&format!("Texture : {}\n\n", texture.description()));
        }

        // Recommandations
        if self.recommandations.is_empty() {
            resume.push_str("Aucune action urgente. Votre sol est en bonne santé !\n");
        } else {
            resume.push_str(&format!(
                "{} recommandation(s) :\n\n",
                self.recommandations.len()
            ));
            for (i, reco) in self.recommandations.iter().enumerate() {
                resume.push_str(&format!(
                    "{}. {} — {:?}\n   {} → {}\n   Action : {}\n   Pourquoi : {}\n   Source : {}\n\n",
                    i + 1,
                    reco.parametre,
                    reco.priorite,
                    reco.valeur_actuelle,
                    reco.valeur_optimale,
                    reco.action,
                    reco.explication,
                    reco.source,
                ));
            }
        }

        resume.push_str(&format!(
            "Complétude de l'analyse : {:.0}%\n",
            self.completude
        ));

        resume
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::AnalyseSol;
    use terrasens_core::{MgParKg, ParcelleId, Ph, Pourcentage};

    fn analyse_problematique() -> AnalyseSol {
        let mut sol = AnalyseSol::nouvelle(ParcelleId::new());
        sol.ph = Ph::new(4.8);                              // Trop acide
        sol.matiere_organique = Pourcentage::new(1.5);      // Trop bas
        sol.phosphore_olsen = MgParKg::new(10.0);           // Déficient
        sol.potassium_echangeable = MgParKg::new(60.0);     // Déficient
        sol.cec = Some(8.0);                                // Trop bas
        sol.argile = Pourcentage::new(15.0);
        sol.limon = Pourcentage::new(20.0);
        sol.sable = Pourcentage::new(65.0);
        sol
    }

    fn analyse_bonne() -> AnalyseSol {
        let mut sol = AnalyseSol::nouvelle(ParcelleId::new());
        sol.ph = Ph::new(6.5);
        sol.matiere_organique = Pourcentage::new(4.0);
        sol.phosphore_olsen = MgParKg::new(25.0);
        sol.potassium_echangeable = MgParKg::new(200.0);
        sol.cec = Some(20.0);
        sol
    }

    #[test]
    fn test_analyse_detecte_problemes() {
        let analyseur = AnalyseurSol::new();
        let rapport = analyseur.analyser(&analyse_problematique());

        assert!(!rapport.recommandations.is_empty());
        assert!(rapport.regles_declenchees.contains(&"SOL-PH-001".to_string()));
        assert!(rapport.regles_declenchees.contains(&"SOL-MO-001".to_string()));
        assert!(rapport.score_global < 60.0);
    }

    #[test]
    fn test_analyse_sol_sain() {
        let analyseur = AnalyseurSol::new();
        let rapport = analyseur.analyser(&analyse_bonne());

        assert!(rapport.recommandations.is_empty());
        assert!(rapport.score_global >= 80.0);
    }

    #[test]
    fn test_resume_contient_info() {
        let analyseur = AnalyseurSol::new();
        let rapport = analyseur.analyser(&analyse_problematique());
        let resume = rapport.resume_agriculteur();

        assert!(resume.contains("RAPPORT DE SOL"));
        assert!(resume.contains("Score"));
        assert!(resume.contains("recommandation"));
    }

    #[test]
    fn test_texture_sableuse() {
        let sol = analyse_problematique();
        // 15% argile, 20% limon, 65% sable → devrait être Sableux
        assert!(sol.classe_texturale().is_some());
    }
}
