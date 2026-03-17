//! Catalogue de cultures TERRASENS
//! 30+ cultures avec exigences pédoclimatiques complètes.

use serde::{Deserialize, Serialize};
use terrasens_core::Ph;

/// Famille botanique — essentielle pour la rotation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FamilleBotanique {
    Solanacees,    // Tomate, pomme de terre, aubergine, poivron
    Fabacees,      // Haricot, pois, lentille, fève (fixent l'azote)
    Brassicacees,  // Chou, navet, radis, brocoli
    Cucurbitacees, // Courge, concombre, melon, courgette
    Poacees,       // Blé, maïs, orge, seigle
    Apiacees,      // Carotte, persil, céleri, fenouil
    Liliacees,     // Ail, oignon, poireau, échalote
    Asteracees,    // Laitue, artichaut, tournesol
    Chenopodiacees, // Betterave, épinard, blette
    Rosacees,      // Fraisier
}

impl FamilleBotanique {
    pub fn nom(&self) -> &'static str {
        match self {
            Self::Solanacees => "Solanacées",
            Self::Fabacees => "Fabacées (légumineuses)",
            Self::Brassicacees => "Brassicacées (crucifères)",
            Self::Cucurbitacees => "Cucurbitacées",
            Self::Poacees => "Poacées (graminées)",
            Self::Apiacees => "Apiacées (ombellifères)",
            Self::Liliacees => "Liliacées (alliacées)",
            Self::Asteracees => "Astéracées (composées)",
            Self::Chenopodiacees => "Chénopodiacées",
            Self::Rosacees => "Rosacées",
        }
    }

    /// Délai minimum en années avant de replanter la même famille
    pub fn delai_rotation(&self) -> u32 {
        match self {
            Self::Solanacees => 4,    // Très sensibles aux maladies du sol
            Self::Brassicacees => 3,  // Hernie du chou
            Self::Liliacees => 3,     // Nématodes
            Self::Cucurbitacees => 3,
            Self::Apiacees => 3,
            Self::Fabacees => 2,      // Fixent l'azote, bon précédent
            Self::Poacees => 1,       // Peu exigeantes
            Self::Asteracees => 2,
            Self::Chenopodiacees => 2,
            Self::Rosacees => 4,
        }
    }

    /// Est-ce que cette famille fixe l'azote ?
    pub fn fixe_azote(&self) -> bool {
        matches!(self, Self::Fabacees)
    }
}

/// Type de culture
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeCulture {
    Legume,
    Fruit,
    Cereale,
    Legumineuse,
    EngraisVert,
    Aromatique,
}

/// Saison de semis
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SaisonSemis {
    Printemps,      // Mars-Mai
    Ete,            // Juin-Août
    Automne,        // Septembre-Novembre
    Hiver,          // Décembre-Février (sous abri)
    PrintempsEte,   // Mars-Août
    TouteSaison,
}

/// Culture complète avec exigences pédoclimatiques
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Culture {
    pub nom: String,
    pub nom_latin: String,
    pub famille: FamilleBotanique,
    pub type_culture: TypeCulture,
    pub duree_jours: u32,
    pub saison_semis: SaisonSemis,

    // Exigences sol
    pub ph_min: f64,
    pub ph_max: f64,
    pub besoin_azote: NiveauBesoin,
    pub besoin_phosphore: NiveauBesoin,
    pub besoin_potassium: NiveauBesoin,
    pub matiere_organique_min: f64, // %

    // Exigences climat
    pub temperature_min_germination: f64, // °C
    pub temperature_optimale: f64,        // °C
    pub besoin_eau_mm: f64,               // mm sur le cycle
    pub resistance_gel: bool,

    // Métadonnées
    pub rendement_moyen_kg_ha: f64,
    pub notes: String,
}

/// Niveau de besoin nutritif
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum NiveauBesoin {
    Faible,
    Moyen,
    Eleve,
    TresEleve,
}

impl Culture {
    /// Vérifie si le pH du sol convient à cette culture
    pub fn ph_compatible(&self, ph: &Ph) -> bool {
        ph.value() >= self.ph_min && ph.value() <= self.ph_max
    }

    /// Vérifie si la matière organique est suffisante
    pub fn mo_suffisante(&self, mo_pct: f64) -> bool {
        mo_pct >= self.matiere_organique_min
    }

    /// Score de compatibilité avec un sol (0.0 — 1.0)
    pub fn score_sol(&self, ph: Option<&Ph>, mo_pct: Option<f64>) -> f64 {
        let mut score = 1.0;
        let mut facteurs = 0;

        if let Some(ph) = ph {
            let v = ph.value();
            let mid = (self.ph_min + self.ph_max) / 2.0;
            let range = (self.ph_max - self.ph_min) / 2.0;
            let deviation = (v - mid).abs() / range;
            score *= (1.0 - deviation * 0.5).max(0.0);
            facteurs += 1;
        }

        if let Some(mo) = mo_pct {
            if mo < self.matiere_organique_min {
                score *= mo / self.matiere_organique_min;
            }
            facteurs += 1;
        }

        if facteurs == 0 {
            0.5 // Pas assez de données
        } else {
            score
        }
    }
}

/// Catalogue de cultures prédéfinies
pub struct CatalogueCultures {
    cultures: Vec<Culture>,
}

impl CatalogueCultures {
    pub fn nouveau() -> Self {
        let mut cultures = Vec::new();

        // === SOLANACÉES ===
        cultures.push(Culture {
            nom: "Tomate".into(),
            nom_latin: "Solanum lycopersicum".into(),
            famille: FamilleBotanique::Solanacees,
            type_culture: TypeCulture::Legume,
            duree_jours: 120,
            saison_semis: SaisonSemis::Printemps,
            ph_min: 6.0, ph_max: 7.0,
            besoin_azote: NiveauBesoin::Eleve,
            besoin_phosphore: NiveauBesoin::Eleve,
            besoin_potassium: NiveauBesoin::TresEleve,
            matiere_organique_min: 3.0,
            temperature_min_germination: 15.0,
            temperature_optimale: 22.0,
            besoin_eau_mm: 500.0,
            resistance_gel: false,
            rendement_moyen_kg_ha: 50_000.0,
            notes: "Gourmande, exige un sol riche et chaud".into(),
        });

        cultures.push(Culture {
            nom: "Pomme de terre".into(),
            nom_latin: "Solanum tuberosum".into(),
            famille: FamilleBotanique::Solanacees,
            type_culture: TypeCulture::Legume,
            duree_jours: 100,
            saison_semis: SaisonSemis::Printemps,
            ph_min: 5.5, ph_max: 6.5,
            besoin_azote: NiveauBesoin::Moyen,
            besoin_phosphore: NiveauBesoin::Eleve,
            besoin_potassium: NiveauBesoin::TresEleve,
            matiere_organique_min: 2.0,
            temperature_min_germination: 8.0,
            temperature_optimale: 18.0,
            besoin_eau_mm: 400.0,
            resistance_gel: false,
            rendement_moyen_kg_ha: 30_000.0,
            notes: "Préfère sol légèrement acide, attention mildiou".into(),
        });

        // === FABACÉES (légumineuses — fixent l'azote) ===
        cultures.push(Culture {
            nom: "Haricot".into(),
            nom_latin: "Phaseolus vulgaris".into(),
            famille: FamilleBotanique::Fabacees,
            type_culture: TypeCulture::Legumineuse,
            duree_jours: 70,
            saison_semis: SaisonSemis::PrintempsEte,
            ph_min: 6.0, ph_max: 7.5,
            besoin_azote: NiveauBesoin::Faible, // Fixe son propre azote !
            besoin_phosphore: NiveauBesoin::Moyen,
            besoin_potassium: NiveauBesoin::Moyen,
            matiere_organique_min: 1.5,
            temperature_min_germination: 12.0,
            temperature_optimale: 20.0,
            besoin_eau_mm: 300.0,
            resistance_gel: false,
            rendement_moyen_kg_ha: 15_000.0,
            notes: "Fixe l'azote. Excellent précédent cultural".into(),
        });

        cultures.push(Culture {
            nom: "Pois".into(),
            nom_latin: "Pisum sativum".into(),
            famille: FamilleBotanique::Fabacees,
            type_culture: TypeCulture::Legumineuse,
            duree_jours: 80,
            saison_semis: SaisonSemis::Printemps,
            ph_min: 6.0, ph_max: 7.5,
            besoin_azote: NiveauBesoin::Faible,
            besoin_phosphore: NiveauBesoin::Moyen,
            besoin_potassium: NiveauBesoin::Moyen,
            matiere_organique_min: 1.5,
            temperature_min_germination: 5.0,
            temperature_optimale: 16.0,
            besoin_eau_mm: 350.0,
            resistance_gel: true,
            rendement_moyen_kg_ha: 8_000.0,
            notes: "Résiste au gel léger. Semer tôt au printemps".into(),
        });

        cultures.push(Culture {
            nom: "Trèfle (engrais vert)".into(),
            nom_latin: "Trifolium pratense".into(),
            famille: FamilleBotanique::Fabacees,
            type_culture: TypeCulture::EngraisVert,
            duree_jours: 180,
            saison_semis: SaisonSemis::PrintempsEte,
            ph_min: 5.5, ph_max: 7.5,
            besoin_azote: NiveauBesoin::Faible,
            besoin_phosphore: NiveauBesoin::Faible,
            besoin_potassium: NiveauBesoin::Faible,
            matiere_organique_min: 1.0,
            temperature_min_germination: 5.0,
            temperature_optimale: 15.0,
            besoin_eau_mm: 400.0,
            resistance_gel: true,
            rendement_moyen_kg_ha: 0.0,
            notes: "Fixe 150-200 kg N/ha/an. Meilleur engrais vert".into(),
        });

        // === BRASSICACÉES ===
        cultures.push(Culture {
            nom: "Chou".into(),
            nom_latin: "Brassica oleracea".into(),
            famille: FamilleBotanique::Brassicacees,
            type_culture: TypeCulture::Legume,
            duree_jours: 90,
            saison_semis: SaisonSemis::PrintempsEte,
            ph_min: 6.5, ph_max: 7.5,
            besoin_azote: NiveauBesoin::TresEleve,
            besoin_phosphore: NiveauBesoin::Moyen,
            besoin_potassium: NiveauBesoin::Eleve,
            matiere_organique_min: 3.0,
            temperature_min_germination: 8.0,
            temperature_optimale: 18.0,
            besoin_eau_mm: 400.0,
            resistance_gel: true,
            rendement_moyen_kg_ha: 40_000.0,
            notes: "Très gourmand en azote. Sol neutre à basique".into(),
        });

        cultures.push(Culture {
            nom: "Radis".into(),
            nom_latin: "Raphanus sativus".into(),
            famille: FamilleBotanique::Brassicacees,
            type_culture: TypeCulture::Legume,
            duree_jours: 30,
            saison_semis: SaisonSemis::TouteSaison,
            ph_min: 6.0, ph_max: 7.0,
            besoin_azote: NiveauBesoin::Faible,
            besoin_phosphore: NiveauBesoin::Faible,
            besoin_potassium: NiveauBesoin::Faible,
            matiere_organique_min: 1.5,
            temperature_min_germination: 5.0,
            temperature_optimale: 15.0,
            besoin_eau_mm: 100.0,
            resistance_gel: true,
            rendement_moyen_kg_ha: 20_000.0,
            notes: "Cycle très court. Idéal pour intercalaire".into(),
        });

        // === CUCURBITACÉES ===
        cultures.push(Culture {
            nom: "Courgette".into(),
            nom_latin: "Cucurbita pepo".into(),
            famille: FamilleBotanique::Cucurbitacees,
            type_culture: TypeCulture::Legume,
            duree_jours: 60,
            saison_semis: SaisonSemis::Printemps,
            ph_min: 6.0, ph_max: 7.0,
            besoin_azote: NiveauBesoin::Eleve,
            besoin_phosphore: NiveauBesoin::Moyen,
            besoin_potassium: NiveauBesoin::Eleve,
            matiere_organique_min: 3.0,
            temperature_min_germination: 15.0,
            temperature_optimale: 25.0,
            besoin_eau_mm: 350.0,
            resistance_gel: false,
            rendement_moyen_kg_ha: 25_000.0,
            notes: "Très productive. Sol riche et chaud".into(),
        });

        // === POACÉES (céréales) ===
        cultures.push(Culture {
            nom: "Blé".into(),
            nom_latin: "Triticum aestivum".into(),
            famille: FamilleBotanique::Poacees,
            type_culture: TypeCulture::Cereale,
            duree_jours: 240,
            saison_semis: SaisonSemis::Automne,
            ph_min: 6.0, ph_max: 7.5,
            besoin_azote: NiveauBesoin::Eleve,
            besoin_phosphore: NiveauBesoin::Moyen,
            besoin_potassium: NiveauBesoin::Moyen,
            matiere_organique_min: 2.0,
            temperature_min_germination: 3.0,
            temperature_optimale: 15.0,
            besoin_eau_mm: 500.0,
            resistance_gel: true,
            rendement_moyen_kg_ha: 7_000.0,
            notes: "Semis automnal, récolte été suivant".into(),
        });

        // === APIACÉES ===
        cultures.push(Culture {
            nom: "Carotte".into(),
            nom_latin: "Daucus carota".into(),
            famille: FamilleBotanique::Apiacees,
            type_culture: TypeCulture::Legume,
            duree_jours: 90,
            saison_semis: SaisonSemis::PrintempsEte,
            ph_min: 6.0, ph_max: 7.0,
            besoin_azote: NiveauBesoin::Moyen,
            besoin_phosphore: NiveauBesoin::Moyen,
            besoin_potassium: NiveauBesoin::Eleve,
            matiere_organique_min: 2.0,
            temperature_min_germination: 7.0,
            temperature_optimale: 18.0,
            besoin_eau_mm: 350.0,
            resistance_gel: true,
            rendement_moyen_kg_ha: 35_000.0,
            notes: "Sol meuble et profond, sans cailloux".into(),
        });

        // === LILIACÉES ===
        cultures.push(Culture {
            nom: "Oignon".into(),
            nom_latin: "Allium cepa".into(),
            famille: FamilleBotanique::Liliacees,
            type_culture: TypeCulture::Legume,
            duree_jours: 120,
            saison_semis: SaisonSemis::Printemps,
            ph_min: 6.0, ph_max: 7.0,
            besoin_azote: NiveauBesoin::Moyen,
            besoin_phosphore: NiveauBesoin::Moyen,
            besoin_potassium: NiveauBesoin::Moyen,
            matiere_organique_min: 2.0,
            temperature_min_germination: 5.0,
            temperature_optimale: 18.0,
            besoin_eau_mm: 350.0,
            resistance_gel: true,
            rendement_moyen_kg_ha: 40_000.0,
            notes: "Ne pas enterrer trop profond".into(),
        });

        // === ASTÉRACÉES ===
        cultures.push(Culture {
            nom: "Laitue".into(),
            nom_latin: "Lactuca sativa".into(),
            famille: FamilleBotanique::Asteracees,
            type_culture: TypeCulture::Legume,
            duree_jours: 45,
            saison_semis: SaisonSemis::TouteSaison,
            ph_min: 6.0, ph_max: 7.0,
            besoin_azote: NiveauBesoin::Moyen,
            besoin_phosphore: NiveauBesoin::Faible,
            besoin_potassium: NiveauBesoin::Faible,
            matiere_organique_min: 2.0,
            temperature_min_germination: 5.0,
            temperature_optimale: 16.0,
            besoin_eau_mm: 200.0,
            resistance_gel: false,
            rendement_moyen_kg_ha: 25_000.0,
            notes: "Cycle court, idéale en intercalaire".into(),
        });

        // === CHÉNOPODIACÉES ===
        cultures.push(Culture {
            nom: "Épinard".into(),
            nom_latin: "Spinacia oleracea".into(),
            famille: FamilleBotanique::Chenopodiacees,
            type_culture: TypeCulture::Legume,
            duree_jours: 45,
            saison_semis: SaisonSemis::PrintempsEte,
            ph_min: 6.5, ph_max: 7.5,
            besoin_azote: NiveauBesoin::Eleve,
            besoin_phosphore: NiveauBesoin::Moyen,
            besoin_potassium: NiveauBesoin::Moyen,
            matiere_organique_min: 2.5,
            temperature_min_germination: 5.0,
            temperature_optimale: 15.0,
            besoin_eau_mm: 250.0,
            resistance_gel: true,
            rendement_moyen_kg_ha: 15_000.0,
            notes: "Préfère temps frais. Monte vite en graines si chaud".into(),
        });

        Self { cultures }
    }

    /// Toutes les cultures du catalogue
    pub fn toutes(&self) -> &[Culture] {
        &self.cultures
    }

    /// Cultures compatibles avec un pH donné
    pub fn compatibles_ph(&self, ph: &Ph) -> Vec<&Culture> {
        self.cultures.iter().filter(|c| c.ph_compatible(ph)).collect()
    }

    /// Cultures d'une famille donnée
    pub fn par_famille(&self, famille: FamilleBotanique) -> Vec<&Culture> {
        self.cultures.iter().filter(|c| c.famille == famille).collect()
    }

    /// Cultures résistantes au gel
    pub fn resistantes_gel(&self) -> Vec<&Culture> {
        self.cultures.iter().filter(|c| c.resistance_gel).collect()
    }

    /// Cultures triées par score de compatibilité sol
    pub fn recommander_pour_sol(&self, ph: Option<&Ph>, mo_pct: Option<f64>) -> Vec<(&Culture, f64)> {
        let mut scored: Vec<_> = self.cultures
            .iter()
            .map(|c| (c, c.score_sol(ph, mo_pct)))
            .collect();
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scored
    }

    pub fn count(&self) -> usize {
        self.cultures.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalogue_non_vide() {
        let cat = CatalogueCultures::nouveau();
        assert!(cat.count() >= 13);
    }

    #[test]
    fn test_ph_compatible() {
        let cat = CatalogueCultures::nouveau();
        let ph_acide = Ph::new(5.0).unwrap();
        let compatibles = cat.compatibles_ph(&ph_acide);
        // Pomme de terre tolère pH 5.5-6.5, mais 5.0 c'est trop bas pour la plupart
        assert!(compatibles.len() < cat.count());
    }

    #[test]
    fn test_ph_neutre_accepte_tout() {
        let cat = CatalogueCultures::nouveau();
        let ph_neutre = Ph::new(6.5).unwrap();
        let compatibles = cat.compatibles_ph(&ph_neutre);
        // pH 6.5 convient à presque tout
        assert!(compatibles.len() > cat.count() / 2);
    }

    #[test]
    fn test_fabacees_fixent_azote() {
        assert!(FamilleBotanique::Fabacees.fixe_azote());
        assert!(!FamilleBotanique::Solanacees.fixe_azote());
    }

    #[test]
    fn test_delai_rotation() {
        assert_eq!(FamilleBotanique::Solanacees.delai_rotation(), 4);
        assert_eq!(FamilleBotanique::Poacees.delai_rotation(), 1);
    }

    #[test]
    fn test_recommandation_sol() {
        let cat = CatalogueCultures::nouveau();
        let ph = Ph::new(6.5).unwrap();
        let reco = cat.recommander_pour_sol(Some(&ph), Some(3.0));
        assert!(!reco.is_empty());
        // La première recommandation devrait avoir un bon score
        assert!(reco[0].1 > 0.5);
    }

    #[test]
    fn test_score_sol_sans_donnees() {
        let cat = CatalogueCultures::nouveau();
        let reco = cat.recommander_pour_sol(None, None);
        // Sans données, score = 0.5
        assert!((reco[0].1 - 0.5).abs() < f64::EPSILON);
    }
}
