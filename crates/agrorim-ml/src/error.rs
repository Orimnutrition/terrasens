//! Erreurs du module ML AGRORIM

use thiserror::Error;

/// Erreurs possibles lors de l'inférence ML locale
#[derive(Debug, Clone, PartialEq, Error)]
pub enum MlError {
    /// Le fichier modèle ONNX n'a pas été trouvé au chemin indiqué
    #[error("Modèle introuvable : {path}")]
    ModelNotFound { path: String },

    /// Les données d'entrée sont invalides (dimensions nulles, tampon vide, etc.)
    #[error("Entrée invalide : {reason}")]
    InvalidInput { reason: String },

    /// L'inférence a échoué pendant l'exécution du modèle
    #[error("Échec de l'inférence : {details}")]
    InferenceFailed { details: String },

    /// Le format d'image fourni n'est pas pris en charge
    #[error("Format non supporté : {format}")]
    UnsupportedFormat { format: String },
}
