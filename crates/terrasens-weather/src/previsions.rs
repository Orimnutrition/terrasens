//! Prévisions météo par moyennes mobiles
//!
//! Modèle statistique local : pas de cloud, pas d'API externe.
//! Basé sur l'historique des observations locales.

use serde::{Deserialize, Serialize};

/// Prévision pour un jour donné
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrevisionJour {
    pub date: String,
    pub temp_min: f64,
    pub temp_max: f64,
    pub precipitation_prob: f64,   // 0.0 - 1.0
    pub precipitation_mm: f64,
    pub vent_moyen_kmh: f64,
    pub confiance: f64,            // 0.0 - 1.0
}

/// Modèle de prévision statistique local
pub struct ModeleLocal {
    historique_temp: Vec<(String, f64, f64)>, // date, min, max
    historique_precip: Vec<(String, f64)>,
}

impl ModeleLocal {
    pub fn nouveau() -> Self {
        Self {
            historique_temp: Vec::new(),
            historique_precip: Vec::new(),
        }
    }

    /// Ajouter une observation journalière
    pub fn ajouter_jour(&mut self, date: &str, temp_min: f64, temp_max: f64, precip_mm: f64) {
        self.historique_temp.push((date.to_string(), temp_min, temp_max));
        self.historique_precip.push((date.to_string(), precip_mm));
    }

    /// Prévision par moyenne mobile sur N jours
    pub fn prevoir(&self, nb_jours: usize) -> Vec<PrevisionJour> {
        let n = self.historique_temp.len();
        if n < 3 {
            return Vec::new();
        }

        let fenetre = n.min(7);
        let derniers = &self.historique_temp[n - fenetre..];

        let moy_min: f64 = derniers.iter().map(|(_, min, _)| min).sum::<f64>() / fenetre as f64;
        let moy_max: f64 = derniers.iter().map(|(_, _, max)| max).sum::<f64>() / fenetre as f64;

        let derniers_p = &self.historique_precip[n - fenetre..];
        let moy_precip: f64 = derniers_p.iter().map(|(_, p)| p).sum::<f64>() / fenetre as f64;
        let jours_pluie = derniers_p.iter().filter(|(_, p)| *p > 0.5).count();
        let prob_pluie = jours_pluie as f64 / fenetre as f64;

        (0..nb_jours)
            .map(|i| {
                let confiance = (1.0 - (i as f64 * 0.15)).max(0.2);
                PrevisionJour {
                    date: format!("J+{}", i + 1),
                    temp_min: moy_min + (i as f64 * 0.1), // légère dérive
                    temp_max: moy_max + (i as f64 * 0.1),
                    precipitation_prob: prob_pluie,
                    precipitation_mm: moy_precip,
                    vent_moyen_kmh: 10.0,
                    confiance,
                }
            })
            .collect()
    }

    pub fn nb_observations(&self) -> usize {
        self.historique_temp.len()
    }
}

/// Résumé météo pour l'agriculteur
#[derive(Debug)]
pub struct ResumeMeteo {
    pub previsions: Vec<PrevisionJour>,
    pub tendance: TendanceMeteo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TendanceMeteo {
    Stable,
    Rechauffement,
    Refroidissement,
    Instable,
}

impl TendanceMeteo {
    pub fn nom(&self) -> &str {
        match self {
            Self::Stable => "Stable",
            Self::Rechauffement => "Réchauffement",
            Self::Refroidissement => "Refroidissement",
            Self::Instable => "Instable",
        }
    }
}

impl ResumeMeteo {
    pub fn resume(&self) -> String {
        let mut s = String::new();
        s.push_str("=== PRÉVISIONS MÉTÉO LOCALES ===\n\n");
        s.push_str(&format!("Tendance : {}\n\n", self.tendance.nom()));

        for p in &self.previsions {
            s.push_str(&format!(
                "{} : {:.0}°C → {:.0}°C | Pluie : {:.0}% ({:.1}mm) | Confiance : {:.0}%\n",
                p.date, p.temp_min, p.temp_max,
                p.precipitation_prob * 100.0, p.precipitation_mm,
                p.confiance * 100.0
            ));
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn modele_test() -> ModeleLocal {
        let mut m = ModeleLocal::nouveau();
        m.ajouter_jour("2026-03-10", 2.0, 12.0, 0.0);
        m.ajouter_jour("2026-03-11", 3.0, 14.0, 2.5);
        m.ajouter_jour("2026-03-12", 1.0, 11.0, 0.0);
        m.ajouter_jour("2026-03-13", 4.0, 15.0, 5.0);
        m.ajouter_jour("2026-03-14", 3.0, 13.0, 0.0);
        m
    }

    #[test]
    fn test_prevision_3_jours() {
        let m = modele_test();
        let prev = m.prevoir(3);
        assert_eq!(prev.len(), 3);
    }

    #[test]
    fn test_confiance_decroissante() {
        let m = modele_test();
        let prev = m.prevoir(5);
        assert!(prev[0].confiance > prev[4].confiance);
    }

    #[test]
    fn test_pas_assez_donnees() {
        let mut m = ModeleLocal::nouveau();
        m.ajouter_jour("2026-03-10", 5.0, 15.0, 0.0);
        assert!(m.prevoir(3).is_empty());
    }

    #[test]
    fn test_resume_lisible() {
        let m = modele_test();
        let resume = ResumeMeteo {
            previsions: m.prevoir(3),
            tendance: TendanceMeteo::Stable,
        };
        let texte = resume.resume();
        assert!(texte.contains("PRÉVISIONS"));
        assert!(texte.contains("Stable"));
    }
}
