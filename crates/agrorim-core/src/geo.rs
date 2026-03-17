//! Utilitaires géospatiaux AGRORIM
//!
//! Coordonnées WGS84, polygones de parcelles, calculs de surface.

use serde::{Deserialize, Serialize};

use crate::types::GeoPoint;

/// Polygone de parcelle (liste de points WGS84)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoPolygon {
    pub points: Vec<GeoPoint>,
}

impl GeoPolygon {
    pub fn new(points: Vec<GeoPoint>) -> Self {
        Self { points }
    }

    /// Surface approximative en hectares (formule de Shoelace sur projection plate)
    pub fn surface_hectares(&self) -> f64 {
        if self.points.len() < 3 {
            return 0.0;
        }

        let n = self.points.len();
        let mut area = 0.0;

        // Conversion degrés → mètres (approximation locale)
        let lat_mid = self.points.iter().map(|p| p.latitude).sum::<f64>() / n as f64;
        let m_per_deg_lat = 111_320.0;
        let m_per_deg_lon = 111_320.0 * lat_mid.to_radians().cos();

        for i in 0..n {
            let j = (i + 1) % n;
            let xi = self.points[i].longitude * m_per_deg_lon;
            let yi = self.points[i].latitude * m_per_deg_lat;
            let xj = self.points[j].longitude * m_per_deg_lon;
            let yj = self.points[j].latitude * m_per_deg_lat;
            area += xi * yj - xj * yi;
        }

        (area.abs() / 2.0) / 10_000.0 // m² → hectares
    }

    /// Centre géométrique (centroïde)
    pub fn centroid(&self) -> GeoPoint {
        let n = self.points.len() as f64;
        let lat = self.points.iter().map(|p| p.latitude).sum::<f64>() / n;
        let lon = self.points.iter().map(|p| p.longitude).sum::<f64>() / n;
        GeoPoint::new(lat, lon)
    }

    /// Vérifie si un point est à l'intérieur du polygone (ray casting)
    pub fn contains(&self, point: &GeoPoint) -> bool {
        let n = self.points.len();
        let mut inside = false;

        let mut j = n - 1;
        for i in 0..n {
            let pi = &self.points[i];
            let pj = &self.points[j];

            if ((pi.latitude > point.latitude) != (pj.latitude > point.latitude))
                && (point.longitude
                    < (pj.longitude - pi.longitude) * (point.latitude - pi.latitude)
                        / (pj.latitude - pi.latitude)
                        + pi.longitude)
            {
                inside = !inside;
            }
            j = i;
        }

        inside
    }
}

/// Bounding box pour requêtes spatiales rapides
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GeoBounds {
    pub min: GeoPoint,
    pub max: GeoPoint,
}

impl GeoBounds {
    pub fn from_polygon(polygon: &GeoPolygon) -> Self {
        let min_lat = polygon
            .points
            .iter()
            .map(|p| p.latitude)
            .fold(f64::MAX, f64::min);
        let min_lon = polygon
            .points
            .iter()
            .map(|p| p.longitude)
            .fold(f64::MAX, f64::min);
        let max_lat = polygon
            .points
            .iter()
            .map(|p| p.latitude)
            .fold(f64::MIN, f64::max);
        let max_lon = polygon
            .points
            .iter()
            .map(|p| p.longitude)
            .fold(f64::MIN, f64::max);

        Self {
            min: GeoPoint::new(min_lat, min_lon),
            max: GeoPoint::new(max_lat, max_lon),
        }
    }

    pub fn contains(&self, point: &GeoPoint) -> bool {
        point.latitude >= self.min.latitude
            && point.latitude <= self.max.latitude
            && point.longitude >= self.min.longitude
            && point.longitude <= self.max.longitude
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parcelle_test() -> GeoPolygon {
        // Rectangle ~1 hectare près de Genève
        GeoPolygon::new(vec![
            GeoPoint::new(46.2000, 6.1400),
            GeoPoint::new(46.2000, 6.1413),
            GeoPoint::new(46.2009, 6.1413),
            GeoPoint::new(46.2009, 6.1400),
        ])
    }

    #[test]
    fn test_surface_approximation() {
        let poly = parcelle_test();
        let surface = poly.surface_hectares();
        // ~1 hectare, tolérance large pour l'approximation
        assert!(surface > 0.5 && surface < 2.0, "Surface: {surface}");
    }

    #[test]
    fn test_centroid() {
        let poly = parcelle_test();
        let center = poly.centroid();
        assert!((center.latitude - 46.20045).abs() < 0.001);
        assert!((center.longitude - 6.14065).abs() < 0.001);
    }

    #[test]
    fn test_contains_point() {
        let poly = parcelle_test();
        let inside = GeoPoint::new(46.2005, 6.1406);
        let outside = GeoPoint::new(46.3000, 6.2000);
        assert!(poly.contains(&inside));
        assert!(!poly.contains(&outside));
    }
}
