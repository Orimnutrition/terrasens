//! Rotation des cultures
//!
//! Règle fondamentale : ne JAMAIS planter la même famille botanique
//! deux années consécutives sur la même parcelle.

use serde::{Deserialize, Serialize};


use crate::catalogue::FamilleBotanique;

/// Plan de rotation sur N années
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanRotation {
    pub parcelle_id: String,
    pub annees: Vec<AnneeRotation>,
}

/// Ce qui est planté une année donnée
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnneeRotation {
    pub annee: u32,
    pub culture_principale: String,
    pub famille: FamilleBotanique,
    pub engrais_vert: Option<String>,
    pub notes: String,
}

/// Validateur de rotation
pub struct ValidateurRotation;

impl ValidateurRotation {
    /// Vérifie qu'un plan de rotation respecte les règles
    pub fn valider(plan: &PlanRotation) -> Vec<ViolationRotation> {
        let mut violations = Vec::new();

        for i in 1..plan.annees.len() {
            let precedent = &plan.annees[i - 1];
            let courant = &plan.annees[i];

            // Règle 1 : pas la même famille deux ans de suite
            if precedent.famille == courant.famille {
                violations.push(ViolationRotation {
                    annee: courant.annee,
                    regle: "Même famille botanique deux ans consécutifs".into(),
                    detail: format!(
                        "{} ({}) après {} ({}) — délai recommandé : {} ans",
                        courant.culture_principale,
                        courant.famille.nom(),
                        precedent.culture_principale,
                        precedent.famille.nom(),
                        courant.famille.delai_rotation()
                    ),
                    source: "Agroscope — Règles de rotation (2017)".into(),
                });
            }

            // Règle 2 : vérifier le délai spécifique de la famille
            let delai = courant.famille.delai_rotation();
            if delai > 1 {
                for j in (0..i).rev().take(delai as usize - 1) {
                    if plan.annees[j].famille == courant.famille {
                        violations.push(ViolationRotation {
                            annee: courant.annee,
                            regle: format!(
                                "Délai de rotation insuffisant pour {}",
                                courant.famille.nom()
                            ),
                            detail: format!(
                                "{} en {} et {} en {} — délai minimum {} ans requis",
                                plan.annees[j].culture_principale,
                                plan.annees[j].annee,
                                courant.culture_principale,
                                courant.annee,
                                delai
                            ),
                            source: "Agroscope — Rotations culturales (2017)".into(),
                        });
                    }
                }
            }
        }

        // Règle 3 : recommander une légumineuse tous les 4 ans
        let nb_annees = plan.annees.len();
        if nb_annees >= 4 {
            let has_legumineuse = plan.annees.iter().any(|a| a.famille == FamilleBotanique::Fabacees);
            if !has_legumineuse {
                violations.push(ViolationRotation {
                    annee: 0,
                    regle: "Aucune légumineuse dans la rotation".into(),
                    detail: "Intégrer un haricot, pois, trèfle ou luzerne pour fixer l'azote \
                             naturellement et réduire les besoins en engrais.".into(),
                    source: "INRAE — Rôle des légumineuses dans la rotation (2019)".into(),
                });
            }
        }

        violations
    }

    /// Suggère la prochaine culture optimale
    pub fn suggerer_suivant(historique: &[AnneeRotation]) -> Vec<SuggestionRotation> {
        let mut suggestions = Vec::new();

        let familles_recentes: Vec<_> = historique.iter().rev().take(4).map(|a| a.famille).collect();

        // Privilégier les familles absentes depuis longtemps
        let toutes_familles = [
            FamilleBotanique::Solanacees,
            FamilleBotanique::Fabacees,
            FamilleBotanique::Brassicacees,
            FamilleBotanique::Cucurbitacees,
            FamilleBotanique::Poacees,
            FamilleBotanique::Apiacees,
            FamilleBotanique::Liliacees,
            FamilleBotanique::Asteracees,
            FamilleBotanique::Chenopodiacees,
        ];

        for famille in &toutes_familles {
            let delai = famille.delai_rotation() as usize;
            let trop_recent = familles_recentes.iter().take(delai).any(|f| f == famille);

            if !trop_recent {
                let priorite = if famille.fixe_azote() && !familles_recentes.contains(famille) {
                    0.95 // Priorité haute pour les légumineuses si absentes
                } else {
                    0.7
                };

                suggestions.push(SuggestionRotation {
                    famille: *famille,
                    score: priorite,
                    raison: if famille.fixe_azote() {
                        "Légumineuse : fixe l'azote gratuitement, excellent précédent".into()
                    } else {
                        format!("Famille {} absente depuis assez longtemps", famille.nom())
                    },
                });
            }
        }

        suggestions.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        suggestions
    }
}

/// Violation détectée dans un plan de rotation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViolationRotation {
    pub annee: u32,
    pub regle: String,
    pub detail: String,
    pub source: String,
}

/// Suggestion de famille pour la prochaine culture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestionRotation {
    pub famille: FamilleBotanique,
    pub score: f64,
    pub raison: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plan_mauvais() -> PlanRotation {
        PlanRotation {
            parcelle_id: "test".into(),
            annees: vec![
                AnneeRotation {
                    annee: 2024,
                    culture_principale: "Tomate".into(),
                    famille: FamilleBotanique::Solanacees,
                    engrais_vert: None,
                    notes: String::new(),
                },
                AnneeRotation {
                    annee: 2025,
                    culture_principale: "Pomme de terre".into(),
                    famille: FamilleBotanique::Solanacees, // VIOLATION !
                    engrais_vert: None,
                    notes: String::new(),
                },
            ],
        }
    }

    fn plan_bon() -> PlanRotation {
        PlanRotation {
            parcelle_id: "test".into(),
            annees: vec![
                AnneeRotation {
                    annee: 2024,
                    culture_principale: "Tomate".into(),
                    famille: FamilleBotanique::Solanacees,
                    engrais_vert: None,
                    notes: String::new(),
                },
                AnneeRotation {
                    annee: 2025,
                    culture_principale: "Haricot".into(),
                    famille: FamilleBotanique::Fabacees,
                    engrais_vert: Some("Trèfle".into()),
                    notes: String::new(),
                },
                AnneeRotation {
                    annee: 2026,
                    culture_principale: "Carotte".into(),
                    famille: FamilleBotanique::Apiacees,
                    engrais_vert: None,
                    notes: String::new(),
                },
            ],
        }
    }

    #[test]
    fn test_detecte_meme_famille() {
        let violations = ValidateurRotation::valider(&plan_mauvais());
        assert!(!violations.is_empty());
    }

    #[test]
    fn test_plan_bon_sans_violation_famille() {
        let violations = ValidateurRotation::valider(&plan_bon());
        // Pas de violation de même famille consécutive
        let violations_famille: Vec<_> = violations
            .iter()
            .filter(|v| v.regle.contains("famille"))
            .collect();
        assert!(violations_famille.is_empty());
    }

    #[test]
    fn test_suggestion_apres_solanacees() {
        let historique = vec![AnneeRotation {
            annee: 2024,
            culture_principale: "Tomate".into(),
            famille: FamilleBotanique::Solanacees,
            engrais_vert: None,
            notes: String::new(),
        }];

        let suggestions = ValidateurRotation::suggerer_suivant(&historique);
        assert!(!suggestions.is_empty());
        // Solanacées ne devrait PAS être suggérée
        assert!(!suggestions.iter().any(|s| s.famille == FamilleBotanique::Solanacees));
    }

    #[test]
    fn test_legumineuse_prioritaire() {
        let historique = vec![
            AnneeRotation {
                annee: 2024,
                culture_principale: "Tomate".into(),
                famille: FamilleBotanique::Solanacees,
                engrais_vert: None,
                notes: String::new(),
            },
        ];

        let suggestions = ValidateurRotation::suggerer_suivant(&historique);
        // Les Fabacées devraient être en haut
        let fabacees_pos = suggestions.iter().position(|s| s.famille == FamilleBotanique::Fabacees);
        assert!(fabacees_pos.is_some());
        assert!(fabacees_pos.unwrap() < 3); // Dans le top 3
    }
}
