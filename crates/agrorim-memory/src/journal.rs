//! Journal de bord de l'agriculteur
//!
//! Enregistrement chronologique des observations, actions et événements.
//! Transmission intergénérationnelle des connaissances locales.

use serde::{Deserialize, Serialize};
use agrorim_core::Timestamp;

/// Type d'entrée dans le journal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeEntree {
    Observation,
    Action,
    Recolte,
    Probleme,
    Meteo,
    Note,
}

impl TypeEntree {
    pub fn icone(&self) -> &str {
        match self {
            Self::Observation => "[OBS]",
            Self::Action => "[ACT]",
            Self::Recolte => "[REC]",
            Self::Probleme => "[PBM]",
            Self::Meteo => "[MET]",
            Self::Note => "[NOT]",
        }
    }
}

/// Entrée de journal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntreeJournal {
    pub timestamp: Timestamp,
    pub type_entree: TypeEntree,
    pub parcelle: Option<String>,
    pub culture: Option<String>,
    pub texte: String,
    pub tags: Vec<String>,
}

/// Journal de bord
pub struct JournalDeBord {
    entrees: Vec<EntreeJournal>,
}

impl JournalDeBord {
    pub fn nouveau() -> Self {
        Self { entrees: Vec::new() }
    }

    /// Ajouter une entrée
    pub fn ajouter(&mut self, type_entree: TypeEntree, texte: &str, parcelle: Option<&str>, culture: Option<&str>, tags: &[&str]) {
        self.entrees.push(EntreeJournal {
            timestamp: Timestamp::now(),
            type_entree,
            parcelle: parcelle.map(|s| s.to_string()),
            culture: culture.map(|s| s.to_string()),
            texte: texte.to_string(),
            tags: tags.iter().map(|t| t.to_string()).collect(),
        });
    }

    /// Rechercher dans le journal
    pub fn rechercher(&self, mot_cle: &str) -> Vec<&EntreeJournal> {
        let mc = mot_cle.to_lowercase();
        self.entrees.iter().filter(|e| {
            e.texte.to_lowercase().contains(&mc)
                || e.tags.iter().any(|t| t.to_lowercase().contains(&mc))
        }).collect()
    }

    /// Filtrer par type
    pub fn par_type(&self, type_entree: TypeEntree) -> Vec<&EntreeJournal> {
        self.entrees.iter().filter(|e| e.type_entree == type_entree).collect()
    }

    /// Filtrer par parcelle
    pub fn par_parcelle(&self, parcelle: &str) -> Vec<&EntreeJournal> {
        self.entrees.iter().filter(|e| {
            e.parcelle.as_deref() == Some(parcelle)
        }).collect()
    }

    /// Résumé du journal
    pub fn resume(&self) -> String {
        let mut s = String::new();
        s.push_str("=== JOURNAL DE BORD ===\n\n");
        s.push_str(&format!("Total entrées : {}\n\n", self.entrees.len()));

        for entree in self.entrees.iter().rev().take(10) {
            s.push_str(&format!(
                "{} {} {}\n",
                entree.type_entree.icone(),
                entree.parcelle.as_deref().unwrap_or("-"),
                entree.texte,
            ));
        }
        s
    }

    pub fn count(&self) -> usize {
        self.entrees.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_journal_creation() {
        let journal = JournalDeBord::nouveau();
        assert_eq!(journal.count(), 0);
    }

    #[test]
    fn test_ajout_entree() {
        let mut journal = JournalDeBord::nouveau();
        journal.ajouter(TypeEntree::Observation, "Premiers bourgeons sur les tomates", Some("P1"), Some("Tomate"), &["printemps"]);
        assert_eq!(journal.count(), 1);
    }

    #[test]
    fn test_recherche() {
        let mut journal = JournalDeBord::nouveau();
        journal.ajouter(TypeEntree::Probleme, "Pucerons sur les fèves", Some("P2"), Some("Fève"), &["ravageur"]);
        journal.ajouter(TypeEntree::Action, "Traitement savon noir", Some("P2"), Some("Fève"), &["bio"]);

        let resultats = journal.rechercher("pucerons");
        assert_eq!(resultats.len(), 1);
    }

    #[test]
    fn test_filtrage_type() {
        let mut journal = JournalDeBord::nouveau();
        journal.ajouter(TypeEntree::Recolte, "15kg de tomates", Some("P1"), Some("Tomate"), &[]);
        journal.ajouter(TypeEntree::Observation, "Sol sec", Some("P1"), None, &[]);
        journal.ajouter(TypeEntree::Recolte, "8kg de courgettes", Some("P3"), Some("Courgette"), &[]);

        let recoltes = journal.par_type(TypeEntree::Recolte);
        assert_eq!(recoltes.len(), 2);
    }

    #[test]
    fn test_resume() {
        let mut journal = JournalDeBord::nouveau();
        journal.ajouter(TypeEntree::Note, "Première saison sur cette parcelle", None, None, &[]);
        let resume = journal.resume();
        assert!(resume.contains("JOURNAL"));
    }
}
