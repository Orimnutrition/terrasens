//! # AGRORIM ML — Module d'inférence locale
//!
//! Couche d'abstraction pour l'inférence ML embarquée sur les modèles YOLOv8 nano.
//! Conçu pour fonctionner hors ligne sur Raspberry Pi et matériel équivalent.
//!
//! ## Architecture
//!
//! - [`InferenceEngine`] : trait central d'inférence, implémentable avec ONNX Runtime
//! - [`LocalEngine`] : moteur simulé (mock réaliste basé sur statistiques image)
//! - [`MlService`] : façade haut niveau pour l'intégration avec M4 (agrorim-health)
//!
//! ## Principes AGRORIM respectés
//! 1. Local-first absolu — aucun appel réseau
//! 2. Zéro extraction de données
//! 3. Explicabilité totale des résultats
//!
//! ## Exemple
//!
//! ```rust
//! use agrorim_ml::{MlService, ModelConfig, ClassLabel};
//!
//! let mut service = MlService::new();
//! assert!(!service.is_model_loaded());
//!
//! let config = ModelConfig {
//!     model_path: "modeles/yolov8n_phyto.onnx".into(),
//!     input_width: 640,
//!     input_height: 640,
//!     num_classes: 7,
//!     confidence_threshold: 0.45,
//! };
//! service.load_model(config).expect("chargement du modèle");
//! assert!(service.is_model_loaded());
//! ```

pub mod engine;
pub mod error;
pub mod model;
pub mod service;

pub use engine::{InferenceEngine, LocalEngine};
pub use error::MlError;
pub use model::{BoundingBox, ClassLabel, Detection, ModelConfig, ModelInfo};
pub use service::MlService;
