//! # TERRASENS Core
//!
//! Types fondamentaux et traits partagés par tous les modules TERRASENS.
//! Chaque module (M1-M8) dépend de ce crate pour garantir la cohérence.
//!
//! ## Piliers inviolables
//! 1. Local-first absolu — zéro cloud
//! 2. Zéro extraction de données
//! 3. Authentification terrain
//! 4. Explicabilité totale
//! 5. Déconnectabilité garantie
//! 6. Open source protégé (AGPL-3.0)
//! 7. Interopérabilité souveraine

pub mod types;
pub mod geo;
pub mod error;
pub mod traits;
pub mod units;

pub use types::*;
pub use geo::*;
pub use error::*;
pub use traits::*;
pub use units::*;
