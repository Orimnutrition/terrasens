//! Service M2 Cultures — API haut niveau
//! Point d'entrée principal avec intrication M1 Sol (ENT-001)

use terrasens_core::Ph;

use crate::associations::TableAssociations;
use crate::calendrier::{CalendrierSemis, ZoneClimatique};
use crate::catalogue::CatalogueCultures;
use crate::rotation::{AnneeRotation, PlanRotation, ValidateurRotation, ViolationRotation};

/// Service principal du module Cultures
pub struct CulturesService {
    catalogue: CatalogueCultures,
    associations: TableAssociations,
    calendrier: CalendrierSemis,
}

impl CulturesService {
    pub fn new(zone: ZoneClimatique) -> Self {
        Self {
            catalogue: CatalogueCultures::nouveau(),
            associations: TableAssociations::nouvelle(),
            calendrier: CalendrierSemis::nouveau(zone),
        }
    }

    /// INTRICATION ENT-001 : recommander des cultures compatibles avec le sol
    pub fn recommander_pour_sol(
        &self,
        ph: Option<&Ph>,
        mo_pct: Option<f64>,
        mois: Option<u32>,
    ) -> RapportRecommandation {
        // Cultures compatibles avec le sol
        let compatibles = self.catalogue.recommander_pour_sol(ph, mo_pct);

        // Filtrer par saison si mois donné
        let semables_noms: Option<Vec<String>> = mois.map(|m| {
            self.calendrier
                .semables_en(m)
                .iter()
                .map(|p| p.culture.clone())
                .collect()
        });

        let mut recommandations: Vec<RecommandationCulture> = Vec::new();

        for (culture, score_sol) in &compatibles {
            // Filtrer par saison
            let en_saison = match &semables_noms {
                Some(noms) => noms.iter().any(|n| n == &culture.nom),
                None => true,
            };

            if !en_saison {
                continue;
            }

            // Chercher les associations bénéfiques
            let benefiques = self.associations.benefiques_pour(&culture.nom);
            let incompatibles = self.associations.incompatibles_pour(&culture.nom);

            recommandations.push(RecommandationCulture {
                culture: culture.nom.clone(),
                famille: culture.famille.nom().to_string(),
                score_sol: *score_sol,
                en_saison,
                compagnons: benefiques.iter().map(|a| {
                    if a.culture_a == culture.nom {
                        a.culture_b.clone()
                    } else {
                        a.culture_a.clone()
                    }
                }).collect(),
                a_eviter: incompatibles.iter().map(|a| {
                    if a.culture_a == culture.nom {
                        a.culture_b.clone()
                    } else {
                        a.culture_a.clone()
                    }
                }).collect(),
                notes: culture.notes.clone(),
            });
        }

        // Trier par score sol décroissant
        recommandations.sort_by(|a, b| b.score_sol.partial_cmp(&a.score_sol).unwrap_or(std::cmp::Ordering::Equal));

        RapportRecommandation {
            ph_analyse: ph.map(|p| p.value()),
            mo_analyse: mo_pct,
            mois,
            recommandations,
        }
    }

    /// Valider un plan de rotation
    pub fn valider_rotation(&self, plan: &PlanRotation) -> Vec<ViolationRotation> {
        ValidateurRotation::valider(plan)
    }

    /// Suggérer la prochaine culture
    pub fn suggerer_suivant(&self, historique: &[AnneeRotation]) -> Vec<crate::rotation::SuggestionRotation> {
        ValidateurRotation::suggerer_suivant(historique)
    }
}

/// Recommandation de culture avec contexte complet
#[derive(Debug, Clone)]
pub struct RecommandationCulture {
    pub culture: String,
    pub famille: String,
    pub score_sol: f64,
    pub en_saison: bool,
    pub compagnons: Vec<String>,
    pub a_eviter: Vec<String>,
    pub notes: String,
}

/// Rapport de recommandation complet
#[derive(Debug)]
pub struct RapportRecommandation {
    pub ph_analyse: Option<f64>,
    pub mo_analyse: Option<f64>,
    pub mois: Option<u32>,
    pub recommandations: Vec<RecommandationCulture>,
}

impl RapportRecommandation {
    /// Résumé en langage simple pour l'agriculteur
    pub fn resume(&self) -> String {
        let mut s = String::new();

        s.push_str("=== RECOMMANDATIONS DE CULTURES ===\n\n");

        if let Some(ph) = self.ph_analyse {
            s.push_str(&format!("pH de votre sol : {:.1}\n", ph));
        }
        if let Some(mo) = self.mo_analyse {
            s.push_str(&format!("Matière organique : {:.1}%\n", mo));
        }
        if let Some(mois) = self.mois {
            let nom_mois = match mois {
                1 => "janvier", 2 => "février", 3 => "mars",
                4 => "avril", 5 => "mai", 6 => "juin",
                7 => "juillet", 8 => "août", 9 => "septembre",
                10 => "octobre", 11 => "novembre", 12 => "décembre",
                _ => "?",
            };
            s.push_str(&format!("Période : {}\n", nom_mois));
        }
        s.push('\n');

        if self.recommandations.is_empty() {
            s.push_str("Aucune culture adaptée trouvée pour ces conditions.\n");
        } else {
            s.push_str(&format!("{} culture(s) recommandée(s) :\n\n", self.recommandations.len()));

            for (i, reco) in self.recommandations.iter().take(5).enumerate() {
                s.push_str(&format!(
                    "{}. {} ({}) — Score sol : {:.0}%\n",
                    i + 1,
                    reco.culture,
                    reco.famille,
                    reco.score_sol * 100.0
                ));

                if !reco.compagnons.is_empty() {
                    s.push_str(&format!("   Bons compagnons : {}\n", reco.compagnons.join(", ")));
                }
                if !reco.a_eviter.is_empty() {
                    s.push_str(&format!("   À éviter avec : {}\n", reco.a_eviter.join(", ")));
                }
                s.push_str(&format!("   {}\n\n", reco.notes));
            }
        }

        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        let service = CulturesService::new(ZoneClimatique::Continental);
        let rapport = service.recommander_pour_sol(None, None, None);
        assert!(!rapport.recommandations.is_empty());
    }

    #[test]
    fn test_intrication_sol_acide() {
        let service = CulturesService::new(ZoneClimatique::Continental);
        let ph_acide = Ph::new(5.0).unwrap();
        let rapport = service.recommander_pour_sol(Some(&ph_acide), Some(1.5), None);
        // Avec un sol acide et pauvre, les scores devraient être bas
        for reco in &rapport.recommandations {
            assert!(reco.score_sol <= 1.0);
        }
    }

    #[test]
    fn test_filtrage_saison() {
        let service = CulturesService::new(ZoneClimatique::Continental);
        let ph = Ph::new(6.5).unwrap();
        let rapport_mars = service.recommander_pour_sol(Some(&ph), Some(3.0), Some(3));
        let rapport_tous = service.recommander_pour_sol(Some(&ph), Some(3.0), None);
        // En mars, moins de cultures qu'en toute saison
        assert!(rapport_mars.recommandations.len() <= rapport_tous.recommandations.len());
    }

    #[test]
    fn test_resume_lisible() {
        let service = CulturesService::new(ZoneClimatique::Continental);
        let ph = Ph::new(6.5).unwrap();
        let rapport = service.recommander_pour_sol(Some(&ph), Some(3.0), Some(5));
        let resume = rapport.resume();
        assert!(resume.contains("RECOMMANDATIONS"));
        assert!(resume.contains("mai"));
    }
}
