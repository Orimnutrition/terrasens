//! Types de données du module ML AGRORIM
//!
//! Structures partagées entre le moteur d'inférence et le service haut niveau.

use serde::{Deserialize, Serialize};

// ============================================================
// Configuration du modèle
// ============================================================

/// Configuration d'un modèle ONNX pour l'inférence locale
///
/// # Exemple
///
/// ```rust
/// use agrorim_ml::ModelConfig;
///
/// let config = ModelConfig {
///     model_path: "modeles/yolov8n_phyto.onnx".into(),
///     input_width: 640,
///     input_height: 640,
///     num_classes: 7,
///     confidence_threshold: 0.45,
/// };
/// assert_eq!(config.input_width, 640);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Chemin local vers le fichier modèle (.onnx)
    pub model_path: String,
    /// Largeur d'entrée attendue par le modèle (pixels)
    pub input_width: u32,
    /// Hauteur d'entrée attendue par le modèle (pixels)
    pub input_height: u32,
    /// Nombre de classes du modèle
    pub num_classes: usize,
    /// Seuil de confiance minimum pour retenir une détection (0.0 – 1.0)
    pub confidence_threshold: f32,
}

// ============================================================
// Boîte englobante
// ============================================================

/// Boîte englobante d'une détection (coordonnées relatives 0.0 – 1.0)
///
/// Toutes les valeurs sont normalisées par rapport aux dimensions de l'image
/// source pour rester indépendantes de la résolution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoundingBox {
    /// Position horizontale du coin supérieur gauche (0.0 – 1.0)
    pub x: f32,
    /// Position verticale du coin supérieur gauche (0.0 – 1.0)
    pub y: f32,
    /// Largeur de la boîte (0.0 – 1.0)
    pub width: f32,
    /// Hauteur de la boîte (0.0 – 1.0)
    pub height: f32,
}

impl BoundingBox {
    /// Surface relative de la boîte englobante
    #[must_use]
    pub fn area(&self) -> f32 {
        self.width * self.height
    }

    /// Vérifie que la boîte est dans les limites normalisées valides
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.x >= 0.0
            && self.y >= 0.0
            && self.width > 0.0
            && self.height > 0.0
            && (self.x + self.width) <= 1.0
            && (self.y + self.height) <= 1.0
    }
}

// ============================================================
// Étiquette de classe
// ============================================================

/// Classes de pathologies reconnues par les modèles YOLOv8 phytosanitaires AGRORIM
///
/// Alignées avec la base de connaissances du module M4 (agrorim-health).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClassLabel {
    /// Mildiou — champignon oomycète (Phytophthora infestans, Plasmopara viticola)
    Mildiou,
    /// Oïdium — champignon ascomycète (Erysiphe spp., Uncinula necator)
    Oidium,
    /// Rouille — champignon basidiomycète (Puccinia spp., Phakopsora spp.)
    Rouille,
    /// Alternariose — champignon ascomycète (Alternaria spp.)
    Alternariose,
    /// Puceron — insecte hémiptère (Aphididae)
    Puceron,
    /// Carence minérale (azote, fer, magnésium, etc.)
    Carence,
    /// Plant sain — aucune anomalie détectée
    Sain,
}

impl ClassLabel {
    /// Nom lisible de la classe en français
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use agrorim_ml::ClassLabel;
    ///
    /// assert_eq!(ClassLabel::Mildiou.nom(), "Mildiou");
    /// assert_eq!(ClassLabel::Sain.nom(), "Sain");
    /// ```
    #[must_use]
    pub fn nom(&self) -> &'static str {
        match self {
            Self::Mildiou => "Mildiou",
            Self::Oidium => "Oïdium",
            Self::Rouille => "Rouille",
            Self::Alternariose => "Alternariose",
            Self::Puceron => "Puceron",
            Self::Carence => "Carence",
            Self::Sain => "Sain",
        }
    }

    /// Indique si la classe représente un état sain (sans intervention requise)
    #[must_use]
    pub fn est_sain(&self) -> bool {
        matches!(self, Self::Sain)
    }

    /// Retourne l'index de classe correspondant à l'ordre d'entraînement du modèle
    #[must_use]
    pub fn index(&self) -> usize {
        match self {
            Self::Mildiou => 0,
            Self::Oidium => 1,
            Self::Rouille => 2,
            Self::Alternariose => 3,
            Self::Puceron => 4,
            Self::Carence => 5,
            Self::Sain => 6,
        }
    }

    /// Construit une `ClassLabel` à partir de son index de classe
    #[must_use]
    pub fn depuis_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::Mildiou),
            1 => Some(Self::Oidium),
            2 => Some(Self::Rouille),
            3 => Some(Self::Alternariose),
            4 => Some(Self::Puceron),
            5 => Some(Self::Carence),
            6 => Some(Self::Sain),
            _ => None,
        }
    }

    /// Toutes les classes disponibles, dans l'ordre d'index
    #[must_use]
    pub fn toutes() -> &'static [ClassLabel] {
        &[
            Self::Mildiou,
            Self::Oidium,
            Self::Rouille,
            Self::Alternariose,
            Self::Puceron,
            Self::Carence,
            Self::Sain,
        ]
    }
}

// ============================================================
// Résultat de détection
// ============================================================

/// Détection individuelle retournée par le moteur d'inférence
///
/// Chaque détection correspond à une région de l'image où le modèle
/// a identifié une pathologie avec une confiance suffisante.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Detection {
    /// Nom de la classe détectée
    pub class_name: String,
    /// Étiquette de classe typée
    pub class_label: ClassLabel,
    /// Score de confiance du modèle (0.0 – 1.0)
    pub confidence: f32,
    /// Boîte englobante de la détection dans l'image
    pub bbox: BoundingBox,
}

impl Detection {
    /// Crée une nouvelle détection
    #[must_use]
    pub fn new(class_label: ClassLabel, confidence: f32, bbox: BoundingBox) -> Self {
        Self {
            class_name: class_label.nom().to_string(),
            class_label,
            confidence,
            bbox,
        }
    }
}

// ============================================================
// Informations du modèle chargé
// ============================================================

/// Métadonnées du modèle ONNX chargé en mémoire
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Nom du modèle (ex. "YOLOv8n-phyto-AGRORIM")
    pub name: String,
    /// Version sémantique du modèle (ex. "1.0.0")
    pub version: String,
    /// Liste des classes reconnues par le modèle
    pub classes: Vec<String>,
    /// Taille d'entrée attendue (largeur, hauteur) en pixels
    pub input_size: (u32, u32),
    /// Indique si le modèle est quantifié INT8 pour l'embarqué
    pub quantized: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_label_nom() {
        assert_eq!(ClassLabel::Mildiou.nom(), "Mildiou");
        assert_eq!(ClassLabel::Oidium.nom(), "Oïdium");
        assert_eq!(ClassLabel::Sain.nom(), "Sain");
    }

    #[test]
    fn test_class_label_index_roundtrip() {
        for label in ClassLabel::toutes() {
            let idx = label.index();
            let retour = ClassLabel::depuis_index(idx).expect("index valide");
            assert_eq!(*label, retour);
        }
    }

    #[test]
    fn test_bounding_box_valid() {
        let boite = BoundingBox { x: 0.1, y: 0.1, width: 0.5, height: 0.5 };
        assert!(boite.is_valid());
        assert!((boite.area() - 0.25).abs() < 1e-6);
    }

    #[test]
    fn test_bounding_box_invalid() {
        let boite = BoundingBox { x: 0.8, y: 0.8, width: 0.5, height: 0.5 };
        assert!(!boite.is_valid()); // dépasse les limites
    }

    #[test]
    fn test_detection_creation() {
        let bbox = BoundingBox { x: 0.1, y: 0.1, width: 0.3, height: 0.3 };
        let det = Detection::new(ClassLabel::Mildiou, 0.87, bbox);
        assert_eq!(det.class_name, "Mildiou");
        assert!((det.confidence - 0.87).abs() < 1e-6);
    }
}
