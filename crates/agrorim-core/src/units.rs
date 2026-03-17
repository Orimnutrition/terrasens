//! Unités de mesure typées — impossible de confondre mg/kg et %

use serde::{Deserialize, Serialize};
use std::fmt;

/// pH du sol (0.0 — 14.0)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Ph(f64);

impl Ph {
    pub fn new(value: f64) -> Option<Self> {
        if (0.0..=14.0).contains(&value) {
            Some(Self(value))
        } else {
            None
        }
    }

    pub fn value(&self) -> f64 {
        self.0
    }

    pub fn is_acide(&self) -> bool {
        self.0 < 6.5
    }

    pub fn is_neutre(&self) -> bool {
        (6.5..=7.5).contains(&self.0)
    }

    pub fn is_basique(&self) -> bool {
        self.0 > 7.5
    }

    pub fn categorie(&self) -> &'static str {
        match self.0 {
            v if v < 4.5 => "Très acide",
            v if v < 5.5 => "Acide",
            v if v < 6.5 => "Légèrement acide",
            v if v < 7.5 => "Neutre",
            v if v < 8.5 => "Légèrement basique",
            _ => "Très basique",
        }
    }
}

impl fmt::Display for Ph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pH {:.1} ({})", self.0, self.categorie())
    }
}

/// Concentration en mg/kg (ppm)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct MgParKg(f64);

impl MgParKg {
    pub fn new(value: f64) -> Option<Self> {
        if value >= 0.0 {
            Some(Self(value))
        } else {
            None
        }
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl fmt::Display for MgParKg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1} mg/kg", self.0)
    }
}

/// Pourcentage (0.0 — 100.0)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Pourcentage(f64);

impl Pourcentage {
    pub fn new(value: f64) -> Option<Self> {
        if (0.0..=100.0).contains(&value) {
            Some(Self(value))
        } else {
            None
        }
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl fmt::Display for Pourcentage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}%", self.0)
    }
}

/// Température en degrés Celsius
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Celsius(f64);

impl Celsius {
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl fmt::Display for Celsius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}°C", self.0)
    }
}

/// Millimètres (précipitations, irrigation)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Millimetres(f64);

impl Millimetres {
    pub fn new(value: f64) -> Option<Self> {
        if value >= 0.0 {
            Some(Self(value))
        } else {
            None
        }
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl fmt::Display for Millimetres {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1} mm", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ph_validation() {
        assert!(Ph::new(7.0).is_some());
        assert!(Ph::new(15.0).is_none());
        assert!(Ph::new(-1.0).is_none());
    }

    #[test]
    fn test_ph_categories() {
        assert!(Ph::new(4.0).unwrap().is_acide());
        assert!(Ph::new(7.0).unwrap().is_neutre());
        assert!(Ph::new(8.5).unwrap().is_basique());
    }

    #[test]
    fn test_mg_par_kg_validation() {
        assert!(MgParKg::new(150.0).is_some());
        assert!(MgParKg::new(-10.0).is_none());
    }

    #[test]
    fn test_pourcentage_validation() {
        assert!(Pourcentage::new(50.0).is_some());
        assert!(Pourcentage::new(101.0).is_none());
        assert!(Pourcentage::new(-1.0).is_none());
    }
}
