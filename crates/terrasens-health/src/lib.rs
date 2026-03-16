//! # TERRASENS M4 — Santé Végétale
//! Détection maladies et ravageurs. Vision par ONNX (YOLOv8 nano).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticPlante {
    pub maladie: Option<String>,
    pub confiance: f64,
    pub traitement_bio: String,
    pub source: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        assert!(true);
    }
}
