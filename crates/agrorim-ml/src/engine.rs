//! Moteurs d'inférence AGRORIM
//!
//! Trait [`InferenceEngine`] et implémentation locale [`LocalEngine`] (simulation réaliste).
//!
//! ## Heuristiques de simulation du moteur local
//!
//! | Condition sur le tampon RGB | Classe retournée |
//! |-----------------------------|-----------------|
//! | Luminosité globale < 60     | Mildiou |
//! | Luminosité < 110, R > G     | Alternariose |
//! | G > R + 20 ET G > B + 20   | Sain |
//! | R > G + 25                  | Rouille |
//! | B > G + 15                  | Carence |
//! | Variance canal G > 2500     | Puceron |
//! | Sinon                       | Oïdium |
//!
//! Quand ONNX Runtime sera intégré, un `OnnxEngine` implémentera le même trait.

use tracing::debug;

use crate::error::MlError;
use crate::model::{BoundingBox, ClassLabel, Detection, ModelConfig, ModelInfo};

// ============================================================
// Trait central
// ============================================================

/// Contrat d'inférence ML pour AGRORIM
///
/// Toute implémentation doit fonctionner 100% hors ligne (pilier n°1 AGRORIM).
/// La méthode [`detect`](InferenceEngine::detect) attend un tampon RGB24
/// (3 octets par pixel, sans en-tête).
pub trait InferenceEngine {
    /// Métadonnées du modèle chargé
    fn info(&self) -> ModelInfo;

    /// Détection d'objets (mode YOLOv8) — retourne les détections dépassant le seuil
    ///
    /// # Arguments
    ///
    /// * `image_data` — Tampon RGB24 brut (3 octets par pixel)
    /// * `width`      — Largeur en pixels
    /// * `height`     — Hauteur en pixels
    ///
    /// # Erreurs
    ///
    /// [`MlError::InvalidInput`] si dimensions nulles ou tampon vide/incohérent.
    fn detect(
        &self,
        image_data: &[u8],
        width: u32,
        height: u32,
    ) -> Result<Vec<Detection>, MlError>;

    /// Classification globale — retourne la classe dominante de l'image
    ///
    /// # Erreurs
    ///
    /// [`MlError::InvalidInput`] si dimensions nulles ou tampon vide/incohérent.
    fn classify(
        &self,
        image_data: &[u8],
        width: u32,
        height: u32,
    ) -> Result<ClassLabel, MlError>;
}

// ============================================================
// Statistiques internes
// ============================================================

#[derive(Debug, Default)]
struct ImageStats {
    moy_r: f32,
    moy_g: f32,
    moy_b: f32,
    luminosite: f32,
    variance_g: f32,
}

impl ImageStats {
    /// Calcule les statistiques à partir d'un tampon RGB24 valide (len % 3 == 0, len > 0)
    fn depuis_tampon(data: &[u8]) -> Self {
        let n = data.len() / 3;
        if n == 0 {
            return Self::default();
        }

        let mut somme_r: u64 = 0;
        let mut somme_g: u64 = 0;
        let mut somme_b: u64 = 0;

        for pixel in data.chunks_exact(3) {
            somme_r += u64::from(pixel[0]);
            somme_g += u64::from(pixel[1]);
            somme_b += u64::from(pixel[2]);
        }

        let moy_r = somme_r as f32 / n as f32;
        let moy_g = somme_g as f32 / n as f32;
        let moy_b = somme_b as f32 / n as f32;
        let luminosite = (moy_r + moy_g + moy_b) / 3.0;

        // Variance sur le canal vert (indicateur de texture foliaire)
        let var_g: f32 = data
            .chunks_exact(3)
            .map(|p| {
                let diff = p[1] as f32 - moy_g;
                diff * diff
            })
            .sum::<f32>()
            / n as f32;

        Self { moy_r, moy_g, moy_b, luminosite, variance_g: var_g }
    }
}

// ============================================================
// Moteur local simulé
// ============================================================

/// Moteur d'inférence local simulé
///
/// Produit des résultats réalistes à partir de statistiques simples extraites
/// du tampon RGB24 (luminosité, déséquilibre des canaux, variance de texture).
/// Aucune dépendance externe n'est requise : conçu pour le développement et les
/// tests avant intégration du runtime ONNX.
#[derive(Debug, Clone)]
pub struct LocalEngine {
    config: ModelConfig,
}

impl LocalEngine {
    /// Crée un nouveau moteur local à partir d'une [`ModelConfig`]
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use agrorim_ml::{InferenceEngine, LocalEngine, ModelConfig};
    ///
    /// let config = ModelConfig {
    ///     model_path: "modeles/yolov8n_phyto.onnx".into(),
    ///     input_width: 640,
    ///     input_height: 640,
    ///     num_classes: 7,
    ///     confidence_threshold: 0.45,
    /// };
    /// let engine = LocalEngine::new(config);
    /// assert_eq!(engine.info().input_size, (640, 640));
    /// ```
    #[must_use]
    pub fn new(config: ModelConfig) -> Self {
        debug!(
            modele = %config.model_path,
            seuil = config.confidence_threshold,
            "Moteur AGRORIM-ML local initialisé (mode simulation)"
        );
        Self { config }
    }

    /// Valide les paramètres d'entrée avant toute inférence
    fn valider_entree(image_data: &[u8], width: u32, height: u32) -> Result<(), MlError> {
        if width == 0 || height == 0 {
            return Err(MlError::InvalidInput {
                reason: format!(
                    "Dimensions nulles : {}x{} — largeur et hauteur doivent être positives",
                    width, height
                ),
            });
        }
        if image_data.is_empty() {
            return Err(MlError::InvalidInput {
                reason: "Tampon d'image vide".into(),
            });
        }
        let attendu = width as usize * height as usize * 3;
        if image_data.len() != attendu {
            return Err(MlError::InvalidInput {
                reason: format!(
                    "Taille tampon incorrecte : {} octets fournis, {} attendus ({}x{}×3 RGB24)",
                    image_data.len(),
                    attendu,
                    width,
                    height
                ),
            });
        }
        Ok(())
    }

    /// Sélectionne la classe la plus probable selon les statistiques image
    fn classe_depuis_stats(s: &ImageStats) -> ClassLabel {
        if s.luminosite < 60.0 {
            // Zones très sombres : nécroses typiques du mildiou
            ClassLabel::Mildiou
        } else if s.luminosite < 110.0 && s.moy_r > s.moy_g {
            // Sombre avec dominante rouge : taches alternariose
            ClassLabel::Alternariose
        } else if s.moy_g > s.moy_r + 20.0 && s.moy_g > s.moy_b + 20.0 {
            // Vert dominant : feuillage sain
            ClassLabel::Sain
        } else if s.moy_r > s.moy_g + 25.0 {
            // Rouge dominant : pustules orangées (rouille)
            ClassLabel::Rouille
        } else if s.moy_b > s.moy_g + 15.0 {
            // Bleu dominant : décoloration chlorotique (carence)
            ClassLabel::Carence
        } else if s.variance_g > 2500.0 {
            // Forte texture : colonies d'insectes (pucerons)
            ClassLabel::Puceron
        } else {
            // Par défaut : poudre blanche uniforme (oïdium)
            ClassLabel::Oidium
        }
    }

    /// Calcule un score de confiance simulé corrélé au signal dominant
    fn confiance_depuis_stats(s: &ImageStats, classe: &ClassLabel) -> f32 {
        let signal = match classe {
            ClassLabel::Mildiou => 1.0 - (s.luminosite / 255.0),
            ClassLabel::Alternariose => (s.moy_r / (s.moy_g + 1.0)).min(2.0) / 2.0,
            ClassLabel::Sain => (s.moy_g / (s.moy_r.max(s.moy_b) + 1.0)).min(2.0) / 2.0,
            ClassLabel::Rouille => (s.moy_r / (s.moy_g + 1.0)).min(2.0) / 2.0,
            ClassLabel::Carence => (s.moy_b / (s.moy_g + 1.0)).min(2.0) / 2.0,
            ClassLabel::Puceron => (s.variance_g / 5_000.0).min(1.0),
            ClassLabel::Oidium => 0.50 + (s.luminosite / 255.0) * 0.30,
        };
        // Borner entre 0.50 et 0.95 pour rester réaliste
        (signal * 0.70 + 0.30).clamp(0.50, 0.95)
    }

    /// Génère une boîte englobante simulée reproductible à partir des stats
    fn bbox_depuis_stats(s: &ImageStats) -> BoundingBox {
        let cx = (s.moy_r / 255.0 * 0.60 + 0.20).clamp(0.15, 0.75_f32);
        let cy = (s.moy_g / 255.0 * 0.60 + 0.20).clamp(0.15, 0.75_f32);
        let w = (0.25 + s.variance_g / 10_000.0).clamp(0.15, 0.55_f32);
        let h = (0.20 + s.moy_b / 1_000.0).clamp(0.15, 0.50_f32);
        let x = (cx - w / 2.0).clamp(0.0, 1.0 - w);
        let y = (cy - h / 2.0).clamp(0.0, 1.0 - h);
        BoundingBox { x, y, width: w, height: h }
    }
}

impl InferenceEngine for LocalEngine {
    fn info(&self) -> ModelInfo {
        ModelInfo {
            name: "YOLOv8n-phyto-AGRORIM".into(),
            version: "0.1.0-mock".into(),
            classes: ClassLabel::toutes()
                .iter()
                .map(|c| c.nom().to_string())
                .collect(),
            input_size: (self.config.input_width, self.config.input_height),
            quantized: false,
        }
    }

    fn detect(
        &self,
        image_data: &[u8],
        width: u32,
        height: u32,
    ) -> Result<Vec<Detection>, MlError> {
        Self::valider_entree(image_data, width, height)?;

        let stats = ImageStats::depuis_tampon(image_data);
        let classe = Self::classe_depuis_stats(&stats);
        let confiance = Self::confiance_depuis_stats(&stats, &classe);

        debug!(
            classe = classe.nom(),
            confiance,
            luminosite = stats.luminosite,
            "Inférence locale terminée"
        );

        // Plantes saines : aucune détection de maladie à retourner
        if classe.est_sain() {
            return Ok(Vec::new());
        }

        // Filtrer par seuil de confiance
        if confiance < self.config.confidence_threshold {
            return Ok(Vec::new());
        }

        let bbox = Self::bbox_depuis_stats(&stats);
        Ok(vec![Detection::new(classe, confiance, bbox)])
    }

    fn classify(
        &self,
        image_data: &[u8],
        width: u32,
        height: u32,
    ) -> Result<ClassLabel, MlError> {
        Self::valider_entree(image_data, width, height)?;
        let stats = ImageStats::depuis_tampon(image_data);
        Ok(Self::classe_depuis_stats(&stats))
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
    // 1. Création du moteur
    // ----------------------------------------------------------
    #[test]
    fn test_engine_creation() {
        let engine = LocalEngine::new(config_test());
        let info = engine.info();
        assert!(info.name.contains("AGRORIM"), "Le nom doit mentionner AGRORIM");
        assert_eq!(info.classes.len(), 7, "7 classes attendues");
        assert_eq!(info.input_size, (4, 4));
    }

    // ----------------------------------------------------------
    // 2. Informations modèle
    // ----------------------------------------------------------
    #[test]
    fn test_model_info() {
        let engine = LocalEngine::new(config_test());
        let info = engine.info();
        assert!(!info.version.is_empty());
        assert!(!info.quantized);
        assert!(info.classes.contains(&"Mildiou".to_string()));
        assert!(info.classes.contains(&"Sain".to_string()));
    }

    // ----------------------------------------------------------
    // 3. Détection simulée (image très sombre -> Mildiou)
    // ----------------------------------------------------------
    #[test]
    fn test_mock_detection() {
        let engine = LocalEngine::new(config_test());
        // Luminosité ~10 -> Mildiou
        let image = image_rgb(10, 10, 10);
        let detections = engine.detect(&image, 4, 4).expect("détection valide");
        assert!(!detections.is_empty(), "Mildiou doit être détecté");
        assert_eq!(detections[0].class_label, ClassLabel::Mildiou);
        assert!(detections[0].confidence >= 0.40);
        assert!(detections[0].bbox.is_valid());
    }

    // ----------------------------------------------------------
    // 4. Classification simulée (vert dominant -> Sain)
    // ----------------------------------------------------------
    #[test]
    fn test_mock_classification() {
        let engine = LocalEngine::new(config_test());
        // G très dominant -> Sain
        let image = image_rgb(30, 200, 30);
        let classe = engine.classify(&image, 4, 4).expect("classification valide");
        assert_eq!(classe, ClassLabel::Sain);
    }

    // ----------------------------------------------------------
    // 5. Seuil de confiance trop élevé -> aucune détection
    // ----------------------------------------------------------
    #[test]
    fn test_confidence_threshold() {
        let mut config = config_test();
        config.confidence_threshold = 0.99;
        let engine = LocalEngine::new(config);
        // Image sombre (serait Mildiou) mais le seuil filtre tout
        let image = image_rgb(10, 10, 10);
        let detections = engine.detect(&image, 4, 4).expect("pas d'erreur d'entrée");
        assert!(
            detections.is_empty(),
            "Seuil 0.99 doit filtrer toutes les détections"
        );
    }

    // ----------------------------------------------------------
    // 6. Noms des classes
    // ----------------------------------------------------------
    #[test]
    fn test_class_label_names() {
        assert_eq!(ClassLabel::Mildiou.nom(), "Mildiou");
        assert_eq!(ClassLabel::Oidium.nom(), "Oïdium");
        assert_eq!(ClassLabel::Rouille.nom(), "Rouille");
        assert_eq!(ClassLabel::Alternariose.nom(), "Alternariose");
        assert_eq!(ClassLabel::Puceron.nom(), "Puceron");
        assert_eq!(ClassLabel::Carence.nom(), "Carence");
        assert_eq!(ClassLabel::Sain.nom(), "Sain");
    }

    // ----------------------------------------------------------
    // 7. Entrée invalide : dimensions nulles
    // ----------------------------------------------------------
    #[test]
    fn test_invalid_input_zero_dimensions() {
        let engine = LocalEngine::new(config_test());
        let err = engine.detect(&[0u8; 48], 0, 4).unwrap_err();
        assert!(
            matches!(err, MlError::InvalidInput { .. }),
            "Doit retourner InvalidInput pour dimensions nulles"
        );
    }

    // ----------------------------------------------------------
    // 8. Entrée invalide : tampon vide
    // ----------------------------------------------------------
    #[test]
    fn test_invalid_input_empty() {
        let engine = LocalEngine::new(config_test());
        let err = engine.detect(&[], 4, 4).unwrap_err();
        assert!(
            matches!(err, MlError::InvalidInput { .. }),
            "Doit retourner InvalidInput pour tampon vide"
        );
    }

    // Vérifications supplémentaires
    #[test]
    fn test_rouille_classification() {
        let engine = LocalEngine::new(config_test());
        // Rouge très dominant (R=220, G=80, B=50) -> Rouille
        let image = image_rgb(220, 80, 50);
        let classe = engine.classify(&image, 4, 4).expect("classification valide");
        assert_eq!(classe, ClassLabel::Rouille);
    }

    #[test]
    fn test_carence_classification() {
        let engine = LocalEngine::new(config_test());
        // Bleu dominant (R=60, G=80, B=200) -> Carence
        let image = image_rgb(60, 80, 200);
        let classe = engine.classify(&image, 4, 4).expect("classification valide");
        assert_eq!(classe, ClassLabel::Carence);
    }
}
