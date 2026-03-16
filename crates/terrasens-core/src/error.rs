//! Erreurs TERRASENS — toutes explicables, jamais cryptiques

use thiserror::Error;

/// Erreur principale TERRASENS
#[derive(Error, Debug)]
pub enum TerraError {
    #[error("Données manquantes : {champ} est requis pour {contexte}")]
    DonneeManquante { champ: String, contexte: String },

    #[error("Valeur hors limites : {champ} = {valeur}, attendu entre {min} et {max}")]
    HorsLimites {
        champ: String,
        valeur: f64,
        min: f64,
        max: f64,
    },

    #[error("Parcelle inconnue : aucune parcelle trouvée avec cet identifiant")]
    ParcelleInconnue,

    #[error("Authentification refusée : {raison}")]
    AuthRefusee { raison: String },

    #[error("Base de données : {0}")]
    Database(String),

    #[error("Capteur non répondant : {capteur_id} — dernière lecture il y a {minutes_depuis} min")]
    CapteurHorsLigne {
        capteur_id: String,
        minutes_depuis: u64,
    },

    #[error("Règle agronomique invalide : {0}")]
    RegleInvalide(String),

    #[error("Erreur interne : {0}")]
    Interne(String),
}

/// Résultat TERRASENS
pub type TerraResult<T> = Result<T, TerraError>;
