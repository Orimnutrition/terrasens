//! Service M6 Mémoire — API haut niveau

use crate::journal::JournalDeBord;
use crate::savoirs::BaseSavoirs;

/// Service mémoire ancestrale
pub struct MemoireService {
    journal: JournalDeBord,
    savoirs: BaseSavoirs,
}

impl MemoireService {
    pub fn new() -> Self {
        Self {
            journal: JournalDeBord::nouveau(),
            savoirs: BaseSavoirs::nouvelle(),
        }
    }

    /// Accès au journal
    pub fn journal(&mut self) -> &mut JournalDeBord {
        &mut self.journal
    }

    /// Rechercher un savoir ancestral
    pub fn chercher_savoir(&self, mot_cle: &str) -> Vec<String> {
        self.savoirs.rechercher(mot_cle).iter().map(|s| {
            format!("{} (fiabilité {:.0}%) — {}", s.titre, s.fiabilite * 100.0, s.description)
        }).collect()
    }

    /// Résumé complet
    pub fn resume(&self) -> String {
        let mut s = String::new();
        s.push_str("=== MÉMOIRE TERRASENS ===\n\n");
        s.push_str(&format!("Savoirs ancestraux : {}\n", self.savoirs.count()));
        s.push_str(&format!("Entrées journal : {}\n\n", self.journal.count()));

        // Top savoirs les plus fiables
        let fiables = self.savoirs.les_plus_fiables(0.85);
        if !fiables.is_empty() {
            s.push_str("Savoirs les plus fiables :\n");
            for savoir in fiables.iter().take(5) {
                s.push_str(&format!("  - {} ({:.0}%)\n", savoir.titre, savoir.fiabilite * 100.0));
            }
        }

        s
    }

    pub fn nb_savoirs(&self) -> usize {
        self.savoirs.count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::journal::TypeEntree;

    #[test]
    fn test_service_creation() {
        let service = MemoireService::new();
        assert!(service.nb_savoirs() >= 10);
    }

    #[test]
    fn test_recherche_savoir() {
        let service = MemoireService::new();
        let resultats = service.chercher_savoir("gel");
        assert!(!resultats.is_empty());
    }

    #[test]
    fn test_journal_via_service() {
        let mut service = MemoireService::new();
        service.journal().ajouter(TypeEntree::Note, "Test mémoire", None, None, &[]);
        assert_eq!(service.journal().count(), 1);
    }

    #[test]
    fn test_resume() {
        let service = MemoireService::new();
        let resume = service.resume();
        assert!(resume.contains("MÉMOIRE"));
    }
}
