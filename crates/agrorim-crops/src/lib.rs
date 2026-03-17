//! # AGRORIM M2 — Planification des Cultures
//!
//! Rotation, associations, calendrier de semis adapté au sol et au climat local.
//! Intriqué avec M1 Sol (ENT-001) : l'analyse de sol filtre les cultures compatibles.
//!
//! ## Fonctionnalités
//! - Catalogue de 30+ cultures avec exigences pédoclimatiques
//! - Rotation automatique (pas de même famille 2 ans de suite)
//! - Associations bénéfiques / incompatibles (compagnonnage)
//! - Calendrier de semis par zone climatique
//! - Recommandations explicables avec sources

pub mod catalogue;
pub mod rotation;
pub mod associations;
pub mod calendrier;
pub mod service;

pub use catalogue::*;
pub use rotation::*;
pub use associations::*;
pub use calendrier::*;
pub use service::*;
