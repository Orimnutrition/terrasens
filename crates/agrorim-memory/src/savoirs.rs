//! Base de savoirs ancestraux
//!
//! Proverbes, dictons et pratiques transmis oralement.
//! Chaque savoir est accompagné d'une validation scientifique quand elle existe.

use crate::{CategorieSavoir, SavoirAncestral};

/// Base de savoirs ancestraux
pub struct BaseSavoirs {
    savoirs: Vec<SavoirAncestral>,
}

impl BaseSavoirs {
    pub fn nouvelle() -> Self {
        let mut s = Vec::new();

        s.push(SavoirAncestral {
            titre: "Saints de Glace".into(),
            description: "Ne jamais planter avant les Saints de Glace (11-13 mai). Risque de gelées tardives.".into(),
            categorie: CategorieSavoir::Semis,
            source_orale: "Tradition paysanne européenne, attestée depuis le Moyen Âge".into(),
            region: "Europe centrale et occidentale".into(),
            fiabilite: 0.85,
            validation_scientifique: Some("Statistiquement, les dernières gelées en plaine suisse surviennent avant mi-mai dans 90% des cas (MétéoSuisse, 2020)".into()),
            tags: vec!["gel".into(), "printemps".into(), "semis".into()],
        });

        s.push(SavoirAncestral {
            titre: "Lune et semis".into(),
            description: "Semer les légumes-feuilles en lune montante, les légumes-racines en lune descendante.".into(),
            categorie: CategorieSavoir::Lune,
            source_orale: "Almanach des agriculteurs, tradition centenaire".into(),
            region: "Europe".into(),
            fiabilite: 0.40,
            validation_scientifique: Some("Aucune preuve scientifique robuste. Études INRAE (2015) : pas de différence significative.".into()),
            tags: vec!["lune".into(), "semis".into(), "tradition".into()],
        });

        s.push(SavoirAncestral {
            titre: "Ortie indicatrice".into(),
            description: "L'ortie pousse sur des sols riches en azote. Sa présence indique un sol fertile.".into(),
            categorie: CategorieSavoir::Sol,
            source_orale: "Connaissance paysanne universelle".into(),
            region: "Europe".into(),
            fiabilite: 0.95,
            validation_scientifique: Some("Confirmé : Urtica dioica est nitrophile. Gérard Ducerf — L'encyclopédie des plantes bio-indicatrices (2008)".into()),
            tags: vec!["ortie".into(), "azote".into(), "bioindicateur".into()],
        });

        s.push(SavoirAncestral {
            titre: "Purin d'ortie".into(),
            description: "Faire macérer 1kg d'orties dans 10L d'eau pendant 2 semaines. Diluer à 10% pour fertiliser, 5% pour repousser les pucerons.".into(),
            categorie: CategorieSavoir::Protection,
            source_orale: "Pratique paysanne millénaire".into(),
            region: "Europe".into(),
            fiabilite: 0.80,
            validation_scientifique: Some("Effet fertilisant confirmé (GRAB 2018). Effet insectifuge partiel confirmé sur pucerons.".into()),
            tags: vec!["ortie".into(), "purin".into(), "fertilisant".into(), "puceron".into()],
        });

        s.push(SavoirAncestral {
            titre: "Ciel rouge le soir".into(),
            description: "Rouge le soir, beau temps ; rouge le matin, pluie en chemin.".into(),
            categorie: CategorieSavoir::Meteo,
            source_orale: "Dicton paysan universel".into(),
            region: "Hémisphère Nord".into(),
            fiabilite: 0.70,
            validation_scientifique: Some("Partiellement vrai dans les latitudes moyennes : les systèmes météo viennent de l'ouest. Un ciel rouge le soir indique un air sec à l'ouest.".into()),
            tags: vec!["météo".into(), "prévision".into(), "dicton".into()],
        });

        s.push(SavoirAncestral {
            titre: "Rotation triennale".into(),
            description: "Alterner chaque année : céréales, légumineuses, jachère. Système médiéval éprouvé.".into(),
            categorie: CategorieSavoir::General,
            source_orale: "Agriculture médiévale européenne (IXe siècle)".into(),
            region: "Europe".into(),
            fiabilite: 0.90,
            validation_scientifique: Some("Principe fondamental de l'agronomie moderne. La rotation prévient les maladies et maintient la fertilité (Agroscope, 2017)".into()),
            tags: vec!["rotation".into(), "fertilité".into(), "histoire".into()],
        });

        s.push(SavoirAncestral {
            titre: "Compagnonnage tomate-basilic".into(),
            description: "Planter du basilic au pied des tomates repousse les pucerons et améliore le goût.".into(),
            categorie: CategorieSavoir::Protection,
            source_orale: "Tradition méditerranéenne".into(),
            region: "Bassin méditerranéen".into(),
            fiabilite: 0.65,
            validation_scientifique: Some("Effet répulsif partiel confirmé sur aleurodes. L'amélioration du goût n'est pas scientifiquement prouvée.".into()),
            tags: vec!["compagnonnage".into(), "tomate".into(), "basilic".into()],
        });

        s.push(SavoirAncestral {
            titre: "Cendres de bois comme amendement".into(),
            description: "Épandre les cendres de bois non traité au potager : apport de potasse et de calcium, désacidification du sol.".into(),
            categorie: CategorieSavoir::Sol,
            source_orale: "Pratique ancestrale universelle".into(),
            region: "Mondial".into(),
            fiabilite: 0.90,
            validation_scientifique: Some("Confirmé : cendres riches en K2O (5-10%) et CaO (25-45%). pH 10-13. À utiliser avec modération. (INRAE, 2016)".into()),
            tags: vec!["cendres".into(), "potasse".into(), "calcium".into(), "amendement".into()],
        });

        s.push(SavoirAncestral {
            titre: "Hirondelles volant bas".into(),
            description: "Quand les hirondelles volent bas, il va pleuvoir.".into(),
            categorie: CategorieSavoir::Meteo,
            source_orale: "Observation paysanne ancestrale".into(),
            region: "Europe".into(),
            fiabilite: 0.75,
            validation_scientifique: Some("Vrai : l'humidité pré-orage fait descendre les insectes (ailes alourdies), les hirondelles suivent leurs proies.".into()),
            tags: vec!["météo".into(), "hirondelle".into(), "pluie".into()],
        });

        s.push(SavoirAncestral {
            titre: "Paillage au foin".into(),
            description: "Couvrir le sol avec du foin pour conserver l'humidité, nourrir le sol et limiter le désherbage.".into(),
            categorie: CategorieSavoir::Sol,
            source_orale: "Technique paysanne, popularisée par Dominique Soltner".into(),
            region: "Europe".into(),
            fiabilite: 0.95,
            validation_scientifique: Some("Largement confirmé : réduction de l'évaporation de 50-70%, activation biologique du sol, inhibition des adventices (INRAE, 2019)".into()),
            tags: vec!["paillage".into(), "humidité".into(), "sol vivant".into()],
        });

        Self { savoirs: s }
    }

    /// Rechercher par mot-clé
    pub fn rechercher(&self, mot_cle: &str) -> Vec<&SavoirAncestral> {
        let mc = mot_cle.to_lowercase();
        self.savoirs.iter().filter(|s| {
            s.titre.to_lowercase().contains(&mc)
                || s.description.to_lowercase().contains(&mc)
                || s.tags.iter().any(|t| t.to_lowercase().contains(&mc))
        }).collect()
    }

    /// Filtrer par catégorie
    pub fn par_categorie(&self, cat: CategorieSavoir) -> Vec<&SavoirAncestral> {
        self.savoirs.iter().filter(|s| s.categorie == cat).collect()
    }

    /// Savoirs les plus fiables
    pub fn les_plus_fiables(&self, min_fiabilite: f64) -> Vec<&SavoirAncestral> {
        self.savoirs.iter().filter(|s| s.fiabilite >= min_fiabilite).collect()
    }

    pub fn count(&self) -> usize {
        self.savoirs.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_non_vide() {
        let base = BaseSavoirs::nouvelle();
        assert!(base.count() >= 10);
    }

    #[test]
    fn test_recherche_ortie() {
        let base = BaseSavoirs::nouvelle();
        let resultats = base.rechercher("ortie");
        assert!(resultats.len() >= 2); // indicatrice + purin
    }

    #[test]
    fn test_categorie_meteo() {
        let base = BaseSavoirs::nouvelle();
        let meteo = base.par_categorie(CategorieSavoir::Meteo);
        assert!(meteo.len() >= 2);
    }

    #[test]
    fn test_fiabilite_haute() {
        let base = BaseSavoirs::nouvelle();
        let fiables = base.les_plus_fiables(0.90);
        assert!(!fiables.is_empty());
        for s in &fiables {
            assert!(s.fiabilite >= 0.90);
        }
    }

    #[test]
    fn test_validation_scientifique() {
        let base = BaseSavoirs::nouvelle();
        // La plupart des savoirs ont une validation
        let avec_validation = base.savoirs.iter().filter(|s| s.validation_scientifique.is_some()).count();
        assert!(avec_validation >= 8);
    }
}
