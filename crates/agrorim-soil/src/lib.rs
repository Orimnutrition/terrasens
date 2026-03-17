//! # AGRORIM M1 — Analyse de Sol
//!
//! Module principal d'analyse de sol. Pilier de AGRORIM.
//! Transforme une analyse labo (ou capteur) en recommandations
//! explicables en langage simple.
//!
//! ## Fonctionnalités
//! - Saisie/import d'analyses de sol
//! - Évaluation automatique par règles agronomiques
//! - Recommandations explicables avec sources
//! - Historique et tendances par parcelle
//! - Export données ouvert (JSON, CSV)

pub mod models;
pub mod analysis;
pub mod service;

pub use models::*;
pub use analysis::*;
pub use service::*;
