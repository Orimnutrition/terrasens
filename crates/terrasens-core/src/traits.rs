//! Traits partagés TERRASENS
//!
//! Chaque module implémente ces traits pour garantir :
//! - Explicabilité totale (Pilier 4)
//! - Déconnectabilité (Pilier 5)
//! - Interopérabilité souveraine (Pilier 7)

use crate::error::TerraResult;
use crate::types::{Confiance, Priorite};

/// Tout composant TERRASENS doit être explicable en langage simple
pub trait Explicable {
    /// Explication en français simple, compréhensible par un agriculteur
    fn expliquer(&self) -> String;

    /// Source scientifique de la recommandation
    fn source(&self) -> &str;

    /// Niveau de confiance de l'analyse
    fn confiance(&self) -> Confiance;
}

/// Tout module doit fonctionner hors ligne (Pilier 5)
pub trait HorsLigne {
    /// Vérifie si le module peut fonctionner sans connexion
    fn est_disponible_offline(&self) -> bool;

    /// Synchronise les données quand la connexion revient
    fn synchroniser(&mut self) -> TerraResult<usize>;
}

/// Persistance locale SQLite
pub trait Persistable {
    /// Nom de la table SQLite
    fn table_name() -> &'static str;

    /// Sauvegarde dans la base locale
    fn sauvegarder(&self) -> TerraResult<()>;

    /// Charge depuis la base locale
    fn charger(id: &str) -> TerraResult<Self>
    where
        Self: Sized;
}

/// Recommandation agronomique
pub trait Recommandation: Explicable {
    /// Priorité de la recommandation
    fn priorite(&self) -> Priorite;

    /// Action concrète à effectuer
    fn action(&self) -> String;

    /// Délai recommandé (en jours)
    fn delai_jours(&self) -> Option<u32>;
}

/// Export de données en format ouvert (Pilier 7 — interopérabilité souveraine)
pub trait Exportable {
    /// Export en JSON (format ouvert)
    fn to_json(&self) -> TerraResult<String>;

    /// Export en CSV (compatible tableur)
    fn to_csv(&self) -> TerraResult<String>;
}
