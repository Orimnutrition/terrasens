//! Associations de cultures (compagnonnage)
//!
//! Certaines plantes se protègent mutuellement, d'autres se nuisent.
//! Sources : recherches en agroécologie, INRAE, GRAB, permaculture.

use serde::{Deserialize, Serialize};

/// Type d'association entre deux cultures
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeAssociation {
    /// Les deux plantes se bénéficient mutuellement
    Benefique,
    /// Les deux plantes se nuisent
    Incompatible,
    /// Aucun effet notable
    Neutre,
}

/// Association documentée entre deux cultures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Association {
    pub culture_a: String,
    pub culture_b: String,
    pub type_association: TypeAssociation,
    pub raison: String,
    pub source: String,
}

/// Table des associations connues
pub struct TableAssociations {
    associations: Vec<Association>,
}

impl TableAssociations {
    pub fn nouvelle() -> Self {
        let mut assocs = Vec::new();

        // === BÉNÉFIQUES ===
        assocs.push(Association {
            culture_a: "Tomate".into(),
            culture_b: "Carotte".into(),
            type_association: TypeAssociation::Benefique,
            raison: "La tomate repousse la mouche de la carotte. La carotte ameublit le sol pour la tomate.".into(),
            source: "GRAB — Associations de cultures (2018)".into(),
        });

        assocs.push(Association {
            culture_a: "Haricot".into(),
            culture_b: "Courgette".into(),
            type_association: TypeAssociation::Benefique,
            raison: "Le haricot fixe l'azote pour la courgette gourmande. Association des trois soeurs (Milpa).".into(),
            source: "INRAE — Systèmes agroécologiques (2020)".into(),
        });

        assocs.push(Association {
            culture_a: "Carotte".into(),
            culture_b: "Oignon".into(),
            type_association: TypeAssociation::Benefique,
            raison: "L'oignon repousse la mouche de la carotte, la carotte repousse la mouche de l'oignon.".into(),
            source: "Fiches GRAB — Compagnonnage (2019)".into(),
        });

        assocs.push(Association {
            culture_a: "Laitue".into(),
            culture_b: "Radis".into(),
            type_association: TypeAssociation::Benefique,
            raison: "Le radis marque les rangs et se récolte avant que la laitue ait besoin d'espace.".into(),
            source: "Permaculture Design — Guildes végétales (2017)".into(),
        });

        assocs.push(Association {
            culture_a: "Chou".into(),
            culture_b: "Épinard".into(),
            type_association: TypeAssociation::Benefique,
            raison: "L'épinard couvre le sol et conserve l'humidité pour le chou.".into(),
            source: "INRAE — Cultures associées (2019)".into(),
        });

        // === INCOMPATIBLES ===
        assocs.push(Association {
            culture_a: "Tomate".into(),
            culture_b: "Chou".into(),
            type_association: TypeAssociation::Incompatible,
            raison: "Les deux sont très gourmandes et se concurrencent pour les nutriments.".into(),
            source: "GRAB — Associations à éviter (2018)".into(),
        });

        assocs.push(Association {
            culture_a: "Haricot".into(),
            culture_b: "Oignon".into(),
            type_association: TypeAssociation::Incompatible,
            raison: "Les alliacées inhibent les rhizobiums du haricot (fixation azote bloquée).".into(),
            source: "Agroscope — Interactions plantes (2016)".into(),
        });

        assocs.push(Association {
            culture_a: "Pomme de terre".into(),
            culture_b: "Tomate".into(),
            type_association: TypeAssociation::Incompatible,
            raison: "Même famille (Solanacées) = mêmes maladies (mildiou). Propagation croisée.".into(),
            source: "INRAE — Phytopathologie (2015)".into(),
        });

        assocs.push(Association {
            culture_a: "Carotte".into(),
            culture_b: "Épinard".into(),
            type_association: TypeAssociation::Incompatible,
            raison: "L'épinard peut inhiber la germination de la carotte (allélopathie).".into(),
            source: "Arvalis — Allélopathie (2020)".into(),
        });

        Self { associations: assocs }
    }

    /// Cherche l'association entre deux cultures
    pub fn chercher(&self, culture_a: &str, culture_b: &str) -> Option<&Association> {
        self.associations.iter().find(|a| {
            (a.culture_a == culture_a && a.culture_b == culture_b)
                || (a.culture_a == culture_b && a.culture_b == culture_a)
        })
    }

    /// Toutes les associations bénéfiques pour une culture
    pub fn benefiques_pour(&self, culture: &str) -> Vec<&Association> {
        self.associations
            .iter()
            .filter(|a| {
                a.type_association == TypeAssociation::Benefique
                    && (a.culture_a == culture || a.culture_b == culture)
            })
            .collect()
    }

    /// Toutes les incompatibilités pour une culture
    pub fn incompatibles_pour(&self, culture: &str) -> Vec<&Association> {
        self.associations
            .iter()
            .filter(|a| {
                a.type_association == TypeAssociation::Incompatible
                    && (a.culture_a == culture || a.culture_b == culture)
            })
            .collect()
    }

    pub fn count(&self) -> usize {
        self.associations.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_non_vide() {
        let table = TableAssociations::nouvelle();
        assert!(table.count() >= 9);
    }

    #[test]
    fn test_tomate_carotte_benefique() {
        let table = TableAssociations::nouvelle();
        let assoc = table.chercher("Tomate", "Carotte");
        assert!(assoc.is_some());
        assert_eq!(assoc.unwrap().type_association, TypeAssociation::Benefique);
    }

    #[test]
    fn test_symetrie() {
        let table = TableAssociations::nouvelle();
        let a = table.chercher("Tomate", "Carotte");
        let b = table.chercher("Carotte", "Tomate");
        assert_eq!(a.unwrap().type_association, b.unwrap().type_association);
    }

    #[test]
    fn test_tomate_chou_incompatible() {
        let table = TableAssociations::nouvelle();
        let assoc = table.chercher("Tomate", "Chou");
        assert!(assoc.is_some());
        assert_eq!(assoc.unwrap().type_association, TypeAssociation::Incompatible);
    }

    #[test]
    fn test_benefiques_pour_carotte() {
        let table = TableAssociations::nouvelle();
        let benefiques = table.benefiques_pour("Carotte");
        assert!(!benefiques.is_empty());
    }
}
