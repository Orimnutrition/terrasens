//! Calendrier de semis par zone climatique

use serde::{Deserialize, Serialize};

/// Zone climatique simplifiée
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ZoneClimatique {
    Alpine,        // > 800m, hivers longs
    Continental,   // Plateau suisse, hivers froids
    Oceanique,     // Influence atlantique, doux
    Mediterraneen, // Hivers doux, étés chauds
}

/// Période de semis pour une culture dans une zone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodeSemis {
    pub culture: String,
    pub zone: ZoneClimatique,
    pub mois_debut: u32, // 1-12
    pub mois_fin: u32,   // 1-12
    pub sous_abri: bool,
    pub notes: String,
}

/// Calendrier de semis
pub struct CalendrierSemis {
    periodes: Vec<PeriodeSemis>,
}

impl CalendrierSemis {
    pub fn nouveau(zone: ZoneClimatique) -> Self {
        let mut periodes = Vec::new();

        match zone {
            ZoneClimatique::Continental => {
                periodes.push(PeriodeSemis { culture: "Tomate".into(), zone, mois_debut: 3, mois_fin: 4, sous_abri: true, notes: "Semis en intérieur, repiquage après gelées (mi-mai)".into() });
                periodes.push(PeriodeSemis { culture: "Pomme de terre".into(), zone, mois_debut: 4, mois_fin: 5, sous_abri: false, notes: "Planter quand le sol atteint 8°C".into() });
                periodes.push(PeriodeSemis { culture: "Haricot".into(), zone, mois_debut: 5, mois_fin: 7, sous_abri: false, notes: "Après les Saints de Glace (mi-mai)".into() });
                periodes.push(PeriodeSemis { culture: "Pois".into(), zone, mois_debut: 3, mois_fin: 4, sous_abri: false, notes: "Semer tôt, résiste au gel léger".into() });
                periodes.push(PeriodeSemis { culture: "Carotte".into(), zone, mois_debut: 4, mois_fin: 7, sous_abri: false, notes: "Échelonner les semis pour récolte continue".into() });
                periodes.push(PeriodeSemis { culture: "Laitue".into(), zone, mois_debut: 3, mois_fin: 9, sous_abri: false, notes: "Toute la saison, choisir variétés adaptées".into() });
                periodes.push(PeriodeSemis { culture: "Chou".into(), zone, mois_debut: 3, mois_fin: 6, sous_abri: true, notes: "Semis sous abri mars, pleine terre mai".into() });
                periodes.push(PeriodeSemis { culture: "Oignon".into(), zone, mois_debut: 3, mois_fin: 4, sous_abri: false, notes: "Bulbilles ou semis direct".into() });
                periodes.push(PeriodeSemis { culture: "Courgette".into(), zone, mois_debut: 5, mois_fin: 6, sous_abri: false, notes: "Sol chaud impératif (>15°C)".into() });
                periodes.push(PeriodeSemis { culture: "Radis".into(), zone, mois_debut: 3, mois_fin: 9, sous_abri: false, notes: "Cycle court, idéal débutants".into() });
                periodes.push(PeriodeSemis { culture: "Épinard".into(), zone, mois_debut: 3, mois_fin: 5, sous_abri: false, notes: "Préfère le frais, éviter été".into() });
                periodes.push(PeriodeSemis { culture: "Blé".into(), zone, mois_debut: 10, mois_fin: 11, sous_abri: false, notes: "Blé d'hiver, récolte juillet".into() });
            }
            _ => {
                // Pour les autres zones, décaler selon le climat
                // (à enrichir avec des données locales)
            }
        }

        Self { periodes }
    }

    /// Cultures semables un mois donné
    pub fn semables_en(&self, mois: u32) -> Vec<&PeriodeSemis> {
        self.periodes
            .iter()
            .filter(|p| {
                if p.mois_debut <= p.mois_fin {
                    mois >= p.mois_debut && mois <= p.mois_fin
                } else {
                    mois >= p.mois_debut || mois <= p.mois_fin
                }
            })
            .collect()
    }

    /// Période de semis pour une culture
    pub fn periode_pour(&self, culture: &str) -> Option<&PeriodeSemis> {
        self.periodes.iter().find(|p| p.culture == culture)
    }

    pub fn count(&self) -> usize {
        self.periodes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calendrier_continental() {
        let cal = CalendrierSemis::nouveau(ZoneClimatique::Continental);
        assert!(cal.count() >= 10);
    }

    #[test]
    fn test_semis_mars() {
        let cal = CalendrierSemis::nouveau(ZoneClimatique::Continental);
        let mars = cal.semables_en(3);
        assert!(!mars.is_empty());
        // Pois et radis sont semables en mars
        let noms: Vec<_> = mars.iter().map(|p| p.culture.as_str()).collect();
        assert!(noms.contains(&"Pois"));
        assert!(noms.contains(&"Radis"));
    }

    #[test]
    fn test_tomate_sous_abri() {
        let cal = CalendrierSemis::nouveau(ZoneClimatique::Continental);
        let tomate = cal.periode_pour("Tomate");
        assert!(tomate.is_some());
        assert!(tomate.unwrap().sous_abri);
    }

    #[test]
    fn test_rien_en_janvier() {
        let cal = CalendrierSemis::nouveau(ZoneClimatique::Continental);
        let jan = cal.semables_en(1);
        // Très peu de choses en janvier en zone continentale
        assert!(jan.len() <= 2);
    }
}
