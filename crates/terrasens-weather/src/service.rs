//! Service M3 Météo — API haut niveau
//!
//! Gère l'historique, les prévisions et les alertes.

use crate::alertes::{AlerteMeteo, DetecteurAlertes};
use crate::previsions::{ModeleLocal, PrevisionJour, ResumeMeteo, TendanceMeteo};
use crate::{ObservationMeteo, SourceMeteo};
use terrasens_core::{Celsius, GeoPoint, Millimetres, Timestamp};

/// Service météo principal
pub struct MeteoService {
    observations: Vec<ObservationMeteo>,
    modele: ModeleLocal,
    position: GeoPoint,
}

impl MeteoService {
    pub fn new(position: GeoPoint) -> Self {
        Self {
            observations: Vec::new(),
            modele: ModeleLocal::nouveau(),
            position,
        }
    }

    /// Enregistrer une observation
    pub fn enregistrer(&mut self, temp: f64, humidite: f64, precip_mm: f64, vent: f64) {
        let obs = ObservationMeteo {
            timestamp: Timestamp::now(),
            position: self.position.clone(),
            temperature: Celsius::new(temp),
            humidite_pct: humidite,
            precipitation_mm: Millimetres::new(precip_mm).unwrap_or(Millimetres::new(0.0).unwrap()),
            vent_kmh: vent,
            direction_vent_deg: 0.0,
            pression_hpa: 1013.25,
            source: SourceMeteo::CapteurLocal("local".into()),
        };
        self.observations.push(obs);

        // Ajouter au modèle (simplifié : chaque obs = 1 jour)
        let date = format!("obs-{}", self.observations.len());
        self.modele.ajouter_jour(&date, temp - 3.0, temp + 3.0, precip_mm);
    }

    /// Prévisions sur N jours
    pub fn prevoir(&self, nb_jours: usize) -> Vec<PrevisionJour> {
        self.modele.prevoir(nb_jours)
    }

    /// Vérifier les alertes sur les dernières observations
    pub fn verifier_alertes(&self, mois: u32) -> Vec<AlerteMeteo> {
        let temps: Vec<(Timestamp, f64)> = self.observations
            .iter()
            .map(|o| (o.timestamp.clone(), o.temperature.value()))
            .collect();

        let mut alertes = DetecteurAlertes::analyser_temperatures(&temps, mois);

        // Vérifier sécheresse (grouper précipitations par semaine simplifiée)
        let chunk_size = 7.max(1);
        let precip_semaines: Vec<f64> = self.observations
            .chunks(chunk_size)
            .map(|chunk| chunk.iter().map(|o| o.precipitation_mm.value()).sum())
            .collect();

        if let Some(alerte_seche) = DetecteurAlertes::analyser_precipitations(&precip_semaines) {
            alertes.push(alerte_seche);
        }

        alertes
    }

    /// Résumé complet pour l'agriculteur
    pub fn resume(&self, mois: u32) -> String {
        let mut s = String::new();
        s.push_str("=== MÉTÉO TERRASENS ===\n\n");

        // Dernière observation
        if let Some(derniere) = self.observations.last() {
            s.push_str(&format!(
                "Dernière observation : {:.1}°C, humidité {:.0}%, vent {:.0} km/h\n\n",
                derniere.temperature.value(),
                derniere.humidite_pct,
                derniere.vent_kmh,
            ));
        }

        // Prévisions
        let previsions = self.prevoir(3);
        if !previsions.is_empty() {
            let resume_meteo = ResumeMeteo {
                previsions,
                tendance: self.calculer_tendance(),
            };
            s.push_str(&resume_meteo.resume());
        }

        // Alertes
        let alertes = self.verifier_alertes(mois);
        if !alertes.is_empty() {
            s.push_str("\n⚠ ALERTES :\n");
            for a in &alertes {
                s.push_str(&format!(
                    "  [{}/5] {} : {}\n",
                    a.severite.value(),
                    a.type_alerte.nom(),
                    a.message
                ));
            }
        }

        s.push_str(&format!("\nObservations enregistrées : {}\n", self.observations.len()));
        s
    }

    fn calculer_tendance(&self) -> TendanceMeteo {
        if self.observations.len() < 3 {
            return TendanceMeteo::Instable;
        }

        let n = self.observations.len();
        let recentes: Vec<f64> = self.observations[n.saturating_sub(5)..]
            .iter()
            .map(|o| o.temperature.value())
            .collect();

        if recentes.len() < 2 {
            return TendanceMeteo::Stable;
        }

        let premiere = recentes[0];
        let derniere = recentes[recentes.len() - 1];
        let diff = derniere - premiere;

        if diff > 3.0 {
            TendanceMeteo::Rechauffement
        } else if diff < -3.0 {
            TendanceMeteo::Refroidissement
        } else {
            TendanceMeteo::Stable
        }
    }

    pub fn nb_observations(&self) -> usize {
        self.observations.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        let service = MeteoService::new(GeoPoint::new(46.2, 6.1));
        assert_eq!(service.nb_observations(), 0);
    }

    #[test]
    fn test_enregistrement() {
        let mut service = MeteoService::new(GeoPoint::new(46.2, 6.1));
        service.enregistrer(15.0, 65.0, 0.0, 10.0);
        service.enregistrer(12.0, 70.0, 2.5, 15.0);
        assert_eq!(service.nb_observations(), 2);
    }

    #[test]
    fn test_alerte_gel() {
        let mut service = MeteoService::new(GeoPoint::new(46.2, 6.1));
        service.enregistrer(-3.0, 90.0, 0.0, 5.0);
        service.enregistrer(-1.0, 85.0, 0.0, 3.0);
        let alertes = service.verifier_alertes(1);
        assert!(!alertes.is_empty());
    }

    #[test]
    fn test_resume() {
        let mut service = MeteoService::new(GeoPoint::new(46.2, 6.1));
        for i in 0..5 {
            service.enregistrer(10.0 + i as f64, 60.0, 1.0, 8.0);
        }
        let resume = service.resume(3);
        assert!(resume.contains("MÉTÉO TERRASENS"));
    }

    #[test]
    fn test_tendance_rechauffement() {
        let mut service = MeteoService::new(GeoPoint::new(46.2, 6.1));
        service.enregistrer(5.0, 60.0, 0.0, 5.0);
        service.enregistrer(8.0, 60.0, 0.0, 5.0);
        service.enregistrer(12.0, 60.0, 0.0, 5.0);
        service.enregistrer(15.0, 60.0, 0.0, 5.0);
        assert_eq!(service.calculer_tendance(), TendanceMeteo::Rechauffement);
    }
}
