//! Service ML haut niveau — façade pour l'intégration avec M4 Santé Végétale
//!
//! [`MlService`] est le point d'entrée recommandé pour les autres modules AGRORIM.
//! Il gère le cycle de vie du modèle et expose une API orientée domaine.

use tracing::info;

use crate::engine::{InferenceEngine, LocalEngine};
use crate::error::MlError;
use crate::model::{ClassLabel, Detection, ModelConfig, ModelInfo};

/// Service ML principal
///
/// Façade haut niveau pour charger un modèle ONNX et exécuter des inférences
/// locales de diagnostic phytosanitaire. Compatible avec le module M4 (agrorim-health).
///
/// # Exemple
///
/// ```rust
/// use agrorim_ml::{MlService, ModelConfig};
///
/// let mut service = MlService::new();
/// assert!(!service.is_model_loaded());
///
/// let config = ModelConfig {
///     model_path: "modeles/yolov8n_phyto.onnx".into(),
///     input_width: 640,
///     input_height: 640,
///     num_classes: 7,
///     confidence_threshold: 0.45,
/// };
/// service.load_model(config).unwrap();
/// assert!(service.is_model_loaded());
/// assert!(service.model_info().is_some());
/// ```
pub struct MlService {
    engine: Option<LocalEngine>,
}

impl MlService {
    /// Crée un nouveau service sans modèle chargé
    #[must_use]
    pub fn new() -> Self {
        Self { engine: None }
    }

    /// Charge un modèle d'inférence
    ///
    /// En mode local (mock), le fichier `.onnx` n'est pas réellement ouvert.
    /// Avec ONNX Runtime (intégration future), cette méthode chargera le graphe
    /// de calcul en mémoire et allouera les tampons nécessaires.
    ///
    /// # Erreurs
    ///
    /// Retourne [`MlError::ModelNotFound`] si le chemin est invalide (futur ONNX).
    pub fn load_model(&mut self, config: ModelConfig) -> Result<(), MlError> {
        info!(
            modele = %config.model_path,
            "Chargement du modèle ML AGRORIM"
        );
        self.engine = Some(LocalEngine::new(config));
        Ok(())
    }

    /// Détecte les maladies végétales dans une image
    ///
    /// Délègue à [`LocalEngine::detect`]. Retourne une erreur si aucun modèle
    /// n'est chargé.
    ///
    /// # Erreurs
    ///
    /// - [`MlError::ModelNotFound`] si aucun modèle n'est chargé
    /// - [`MlError::InvalidInput`] si les dimensions ou le tampon sont invalides
    pub fn detect_disease(
        &self,
        image_data: &[u8],
        width: u32,
        height: u32,
    ) -> Result<Vec<Detection>, MlError> {
        match &self.engine {
            Some(engine) => engine.detect(image_data, width, height),
            None => Err(MlError::ModelNotFound {
                path: "(aucun modèle chargé — appelez load_model d'abord)".into(),
            }),
        }
    }

    /// Vérifie si un modèle est actuellement chargé
    #[must_use]
    pub fn is_model_loaded(&self) -> bool {
        self.engine.is_some()
    }

    /// Retourne les métadonnées du modèle chargé, ou `None` si absent
    #[must_use]
    pub fn model_info(&self) -> Option<ModelInfo> {
        self.engine.as_ref().map(|e| e.info())
    }

    /// Résumé textuel de l'état du service (utile pour les logs terrain)
    #[must_use]
    pub fn resume(&self) -> String {
        let mut s = String::from("=== ML AGRORIM ===\n\n");

        if let Some(info) = self.model_info() {
            s.push_str(&format!("Modele : {} v{}\n", info.name, info.version));
            s.push_str(&format!("Classes : {}\n", info.classes.join(", ")));
            s.push_str(&format!("Entree : {}x{}\n", info.input_size.0, info.input_size.1));
            s.push_str(&format!(
                "Quantifie : {}\n",
                if info.quantized { "Oui (INT8)" } else { "Non (FP32)" }
            ));
        } else {
            s.push_str("Aucun modele charge.\n");
            s.push_str("Utilisez load_model() avec un fichier .onnx.\n");
        }

        s
    }

    /// Classifie une image et retourne la classe dominante
    ///
    /// # Erreurs
    ///
    /// - [`MlError::ModelNotFound`] si aucun modèle n'est chargé
    /// - [`MlError::InvalidInput`] si les dimensions ou le tampon sont invalides
    pub fn classify_image(
        &self,
        image_data: &[u8],
        width: u32,
        height: u32,
    ) -> Result<ClassLabel, MlError> {
        match &self.engine {
            Some(engine) => engine.classify(image_data, width, height),
            None => Err(MlError::ModelNotFound {
                path: "(aucun modèle chargé — appelez load_model d'abord)".into(),
            }),
        }
    }
}

impl Default for MlService {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn config_test() -> ModelConfig {
        ModelConfig {
            model_path: "modeles/yolov8n_phyto.onnx".into(),
            input_width: 4,
            input_height: 4,
            num_classes: 7,
            confidence_threshold: 0.40,
        }
    }

    /// Image RGB24 uniforme 4x4 (48 octets)
    fn image_rgb(r: u8, g: u8, b: u8) -> Vec<u8> {
        vec![[r, g, b]; 16].into_iter().flatten().collect()
    }

    // ----------------------------------------------------------
    // 7. Service sans modèle chargé
    // ----------------------------------------------------------
    #[test]
    fn test_service_no_model() {
        let service = MlService::new();
        assert!(!service.is_model_loaded(), "Aucun modèle au démarrage");
        assert!(service.model_info().is_none(), "model_info() doit être None");

        // detect_disease doit retourner une erreur explicite
        let err = service
            .detect_disease(&image_rgb(100, 100, 100), 4, 4)
            .unwrap_err();
        assert!(
            matches!(err, MlError::ModelNotFound { .. }),
            "Doit retourner ModelNotFound sans modèle"
        );
    }

    // ----------------------------------------------------------
    // 8. Service avec modèle chargé
    // ----------------------------------------------------------
    #[test]
    fn test_service_with_model() {
        let mut service = MlService::new();
        service.load_model(config_test()).expect("chargement doit réussir");

        assert!(service.is_model_loaded(), "Modèle doit être chargé");

        let info = service.model_info().expect("model_info() doit être Some");
        assert_eq!(info.classes.len(), 7, "7 classes attendues");
        assert_eq!(info.input_size, (4, 4));
    }

    #[test]
    fn test_service_detect_maladie() {
        let mut service = MlService::new();
        service.load_model(config_test()).unwrap();

        // Image très sombre -> Mildiou
        let image = image_rgb(10, 10, 10);
        let detections = service.detect_disease(&image, 4, 4).expect("détection valide");
        assert!(!detections.is_empty(), "Mildiou doit être détecté");
    }

    #[test]
    fn test_service_classify_sain() {
        let mut service = MlService::new();
        service.load_model(config_test()).unwrap();

        // Vert dominant -> Sain
        let image = image_rgb(30, 200, 30);
        let classe = service.classify_image(&image, 4, 4).expect("classification valide");
        assert_eq!(classe, ClassLabel::Sain);
    }

    #[test]
    fn test_service_resume_sans_modele() {
        let service = MlService::new();
        assert!(service.resume().contains("Aucun modele"));
    }

    #[test]
    fn test_service_resume_avec_modele() {
        let mut service = MlService::new();
        service.load_model(config_test()).unwrap();
        let resume = service.resume();
        assert!(resume.contains("AGRORIM"));
        assert!(resume.contains("4x4"));
    }
}
