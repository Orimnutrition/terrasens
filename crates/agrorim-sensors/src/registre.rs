//! Registre de capteurs — gestion et surveillance de l'état des capteurs terrain
//!
//! Maintient un inventaire complet de tous les capteurs déployés, suit leur état
//! en temps réel et génère des alertes en français lorsque des anomalies sont détectées.
//!
//! ## Exemple
//! ```rust
//! use agrorim_sensors::TypeCapteur;
//! use agrorim_sensors::registre::RegistreCapteurs;
//!
//! let mut registre = RegistreCapteurs::nouveau();
//! registre.enregistrer("T-001", "Capteur Température Nord", TypeCapteur::Temperature, "Parcelle Nord");
//! registre.recevoir_lecture("T-001", 22.5, "°C");
//!
//! assert_eq!(registre.nb_capteurs(), 1);
//! assert_eq!(registre.capteurs_actifs().len(), 1);
//! ```

use crate::TypeCapteur;
use agrorim_core::Timestamp;
use serde::{Deserialize, Serialize};

/// Seuil de batterie faible (en %)
const SEUIL_BATTERIE_FAIBLE: f64 = 20.0;

/// Plages de valeurs acceptables par type de capteur
const TEMP_MIN: f64 = -20.0;
const TEMP_MAX: f64 = 60.0;
const HUMIDITE_MIN: f64 = 0.0;
const HUMIDITE_MAX: f64 = 100.0;
const HUMIDITE_SOL_MIN: f64 = 0.0;
const HUMIDITE_SOL_MAX: f64 = 100.0;
const PH_MIN: f64 = 3.0;
const PH_MAX: f64 = 10.0;
const LUX_MIN: f64 = 0.0;
const LUX_MAX: f64 = 150_000.0;
const PRESSION_MIN: f64 = 870.0;
const PRESSION_MAX: f64 = 1084.0;
const VENT_MIN: f64 = 0.0;
const VENT_MAX: f64 = 200.0;
const PLUIE_MIN: f64 = 0.0;
const PLUIE_MAX: f64 = 500.0;
const CO2_MIN: f64 = 300.0;
const CO2_MAX: f64 = 5000.0;

/// Etat opérationnel d'un capteur
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EtatCapteur {
    /// Capteur connecté et fonctionnel
    EnLigne,
    /// Capteur ne répondant plus
    HorsLigne,
    /// Batterie en dessous du seuil critique
    BatterieFaible,
    /// Erreur matérielle ou de communication
    Erreur,
}

impl EtatCapteur {
    /// Libellé français de l'état
    pub fn libelle(&self) -> &'static str {
        match self {
            Self::EnLigne => "En ligne",
            Self::HorsLigne => "Hors ligne",
            Self::BatterieFaible => "Batterie faible",
            Self::Erreur => "Erreur",
        }
    }

    /// Indique si le capteur est opérationnel
    pub fn est_operationnel(&self) -> bool {
        matches!(self, Self::EnLigne | Self::BatterieFaible)
    }
}

/// Informations complètes sur un capteur enregistré
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfoCapteur {
    /// Identifiant unique du capteur
    pub id: String,
    /// Nom lisible du capteur
    pub nom: String,
    /// Type de mesure
    pub type_capteur: TypeCapteur,
    /// Localisation terrain (ex : "Parcelle Nord — Secteur A3")
    pub localisation: String,
    /// Dernière lecture enregistrée
    pub derniere_lecture: Option<DerniereValeur>,
    /// Etat courant du capteur
    pub etat: EtatCapteur,
    /// Niveau de batterie en pourcentage (None si alimenté sur secteur)
    pub batterie_pct: Option<f64>,
    /// Date d'enregistrement dans le système
    pub enregistre_le: Timestamp,
}

/// Dernière valeur mesurée par un capteur
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerniereValeur {
    /// Valeur numérique
    pub valeur: f64,
    /// Unité de mesure
    pub unite: String,
    /// Horodatage de la mesure
    pub timestamp: Timestamp,
}

impl InfoCapteur {
    /// Crée une fiche capteur initiale
    fn nouveau(
        id: impl Into<String>,
        nom: impl Into<String>,
        type_capteur: TypeCapteur,
        localisation: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            nom: nom.into(),
            type_capteur,
            localisation: localisation.into(),
            derniere_lecture: None,
            etat: EtatCapteur::EnLigne,
            batterie_pct: Some(100.0),
            enregistre_le: Timestamp::now(),
        }
    }

    /// Résumé court en français
    pub fn resume_court(&self) -> String {
        let valeur_str = match &self.derniere_lecture {
            Some(v) => format!("{:.1} {}", v.valeur, v.unite),
            None => "aucune lecture".to_string(),
        };
        let batterie_str = match self.batterie_pct {
            Some(b) => format!(" | Batterie : {b:.0}%"),
            None => String::new(),
        };
        format!(
            "[{}] {} — {} ({}) — {} | {}{}",
            self.etat.libelle(),
            self.id,
            self.nom,
            self.localisation,
            valeur_str,
            self.type_capteur_libelle(),
            batterie_str
        )
    }

    fn type_capteur_libelle(&self) -> &'static str {
        match self.type_capteur {
            TypeCapteur::Temperature => "Température",
            TypeCapteur::Humidite => "Humidité air",
            TypeCapteur::HumiditeSol => "Humidité sol",
            TypeCapteur::Ph => "pH",
            TypeCapteur::Luminosite => "Luminosité",
            TypeCapteur::Pression => "Pression",
            TypeCapteur::Vent => "Vent",
            TypeCapteur::Pluie => "Pluviométrie",
            TypeCapteur::Co2 => "CO2",
        }
    }
}

/// Type d'alerte générée par le système
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeAlerte {
    /// Batterie en dessous du seuil critique
    BatterieFaible,
    /// Capteur ne répond plus
    HorsLigne,
    /// Valeur mesurée hors des plages normales
    ValeurAnormale,
    /// Calibration requise (dérive détectée)
    Calibration,
}

impl TypeAlerte {
    /// Libellé français du type d'alerte
    pub fn libelle(&self) -> &'static str {
        match self {
            Self::BatterieFaible => "Batterie faible",
            Self::HorsLigne => "Capteur hors ligne",
            Self::ValeurAnormale => "Valeur anormale",
            Self::Calibration => "Calibration requise",
        }
    }
}

/// Alerte générée par le registre de capteurs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alerte {
    /// Identifiant du capteur concerné
    pub capteur_id: String,
    /// Type d'alerte
    pub type_alerte: TypeAlerte,
    /// Message explicatif en français
    pub message: String,
    /// Horodatage de génération de l'alerte
    pub timestamp: Timestamp,
    /// Sévérité de 1 (information) à 5 (critique)
    pub severite: u8,
}

impl Alerte {
    fn nouvelle(
        capteur_id: impl Into<String>,
        type_alerte: TypeAlerte,
        message: impl Into<String>,
        severite: u8,
    ) -> Self {
        Self {
            capteur_id: capteur_id.into(),
            type_alerte,
            message: message.into(),
            timestamp: Timestamp::now(),
            severite: severite.clamp(1, 5),
        }
    }
}

/// Registre central de tous les capteurs déployés
///
/// Maintient l'état de chaque capteur et génère automatiquement des alertes
/// lorsqu'une anomalie est détectée (batterie faible, valeur anormale, hors ligne).
pub struct RegistreCapteurs {
    /// Inventaire des capteurs indexé par identifiant
    capteurs: std::collections::HashMap<String, InfoCapteur>,
    /// File d'alertes générées
    alertes: Vec<Alerte>,
}

impl RegistreCapteurs {
    /// Crée un nouveau registre vide
    ///
    /// # Exemple
    /// ```rust
    /// use agrorim_sensors::registre::RegistreCapteurs;
    ///
    /// let registre = RegistreCapteurs::nouveau();
    /// assert_eq!(registre.nb_capteurs(), 0);
    /// ```
    pub fn nouveau() -> Self {
        Self {
            capteurs: std::collections::HashMap::new(),
            alertes: Vec::new(),
        }
    }

    /// Enregistre un nouveau capteur dans le registre
    ///
    /// Si un capteur avec le même identifiant existe déjà, il est mis à jour.
    ///
    /// # Exemple
    /// ```rust
    /// use agrorim_sensors::{TypeCapteur};
    /// use agrorim_sensors::registre::RegistreCapteurs;
    ///
    /// let mut registre = RegistreCapteurs::nouveau();
    /// registre.enregistrer("H-001", "Hygromètre Serre", TypeCapteur::HumiditeSol, "Serre A");
    /// assert_eq!(registre.nb_capteurs(), 1);
    /// ```
    pub fn enregistrer(
        &mut self,
        id: &str,
        nom: &str,
        type_capteur: TypeCapteur,
        localisation: &str,
    ) {
        tracing::debug!(
            capteur_id = %id,
            nom = %nom,
            localisation = %localisation,
            "Nouveau capteur enregistré"
        );
        self.capteurs.insert(
            id.to_string(),
            InfoCapteur::nouveau(id, nom, type_capteur, localisation),
        );
    }

    /// Enregistre une nouvelle lecture pour un capteur
    ///
    /// Met à jour l'état du capteur et génère les alertes nécessaires si
    /// la valeur est hors des plages normales.
    ///
    /// # Exemple
    /// ```rust
    /// use agrorim_sensors::TypeCapteur;
    /// use agrorim_sensors::registre::RegistreCapteurs;
    ///
    /// let mut registre = RegistreCapteurs::nouveau();
    /// registre.enregistrer("T-001", "Capteur Temp", TypeCapteur::Temperature, "Nord");
    /// registre.recevoir_lecture("T-001", 22.5, "°C");
    ///
    /// let info = registre.etat_capteur("T-001").unwrap();
    /// assert!(info.derniere_lecture.is_some());
    /// ```
    pub fn recevoir_lecture(&mut self, capteur_id: &str, valeur: f64, unite: &str) {
        if let Some(capteur) = self.capteurs.get_mut(capteur_id) {
            capteur.derniere_lecture = Some(DerniereValeur {
                valeur,
                unite: unite.to_string(),
                timestamp: Timestamp::now(),
            });

            // Remettre en ligne si le capteur était hors ligne
            if capteur.etat == EtatCapteur::HorsLigne {
                capteur.etat = EtatCapteur::EnLigne;
            }

            // Vérifier si la valeur est dans la plage normale
            let type_capteur = capteur.type_capteur.clone();
            if let Some(alerte) = verifier_plage_valeur(capteur_id, &type_capteur, valeur) {
                self.alertes.push(alerte);
            }

            tracing::debug!(
                capteur_id = %capteur_id,
                valeur = valeur,
                unite = %unite,
                "Lecture capteur enregistrée"
            );
        } else {
            tracing::warn!(
                capteur_id = %capteur_id,
                "Lecture reçue pour un capteur inconnu"
            );
        }
    }

    /// Met à jour le niveau de batterie d'un capteur
    ///
    /// Génère une alerte si la batterie passe en dessous du seuil critique.
    pub fn mettre_a_jour_batterie(&mut self, capteur_id: &str, batterie_pct: f64) {
        if let Some(capteur) = self.capteurs.get_mut(capteur_id) {
            capteur.batterie_pct = Some(batterie_pct.clamp(0.0, 100.0));

            if batterie_pct <= SEUIL_BATTERIE_FAIBLE {
                capteur.etat = EtatCapteur::BatterieFaible;
                let message = format!(
                    "Batterie du capteur '{}' à {batterie_pct:.0}% — remplacement recommandé sous 48 h.",
                    capteur.nom
                );
                self.alertes.push(Alerte::nouvelle(
                    capteur_id,
                    TypeAlerte::BatterieFaible,
                    message,
                    3,
                ));
                tracing::warn!(
                    capteur_id = %capteur_id,
                    batterie_pct = batterie_pct,
                    "Alerte batterie faible générée"
                );
            }
        }
    }

    /// Marque un capteur comme hors ligne
    ///
    /// Génère une alerte de sévérité 4.
    pub fn marquer_hors_ligne(&mut self, capteur_id: &str) {
        if let Some(capteur) = self.capteurs.get_mut(capteur_id) {
            capteur.etat = EtatCapteur::HorsLigne;
            let message = format!(
                "Capteur '{}' ({}) ne répond plus — vérifier la connexion et l'alimentation.",
                capteur.nom, capteur.localisation
            );
            self.alertes.push(Alerte::nouvelle(
                capteur_id,
                TypeAlerte::HorsLigne,
                message,
                4,
            ));
            tracing::warn!(
                capteur_id = %capteur_id,
                "Capteur marqué hors ligne"
            );
        }
    }

    /// Retourne les informations d'un capteur par son identifiant
    pub fn etat_capteur(&self, id: &str) -> Option<&InfoCapteur> {
        self.capteurs.get(id)
    }

    /// Retourne tous les capteurs en ligne (EnLigne ou BatterieFaible)
    pub fn capteurs_actifs(&self) -> Vec<&InfoCapteur> {
        self.capteurs
            .values()
            .filter(|c| c.etat.est_operationnel())
            .collect()
    }

    /// Retourne tous les capteurs hors ligne ou en erreur
    pub fn capteurs_hors_ligne(&self) -> Vec<&InfoCapteur> {
        self.capteurs
            .values()
            .filter(|c| !c.etat.est_operationnel())
            .collect()
    }

    /// Retourne toutes les alertes générées
    pub fn alertes(&self) -> &[Alerte] {
        &self.alertes
    }

    /// Retourne les alertes par type
    pub fn alertes_par_type(&self, type_alerte: TypeAlerte) -> Vec<&Alerte> {
        self.alertes
            .iter()
            .filter(|a| a.type_alerte == type_alerte)
            .collect()
    }

    /// Vide l'historique des alertes
    pub fn vider_alertes(&mut self) {
        self.alertes.clear();
    }

    /// Nombre total de capteurs enregistrés
    pub fn nb_capteurs(&self) -> usize {
        self.capteurs.len()
    }

    /// Génère un résumé complet en français de l'état du parc de capteurs
    ///
    /// # Exemple
    /// ```rust
    /// use agrorim_sensors::TypeCapteur;
    /// use agrorim_sensors::registre::RegistreCapteurs;
    ///
    /// let mut registre = RegistreCapteurs::nouveau();
    /// registre.enregistrer("T-001", "Capteur Temp", TypeCapteur::Temperature, "Nord");
    /// let resume = registre.resume();
    /// assert!(resume.contains("capteur"));
    /// ```
    pub fn resume(&self) -> String {
        let total = self.capteurs.len();
        let actifs = self.capteurs_actifs().len();
        let hors_ligne = self.capteurs_hors_ligne().len();
        let nb_alertes = self.alertes.len();
        let alertes_critiques = self.alertes.iter().filter(|a| a.severite >= 4).count();

        let mut lignes = vec![
            "=== Registre AGRORIM — Etat du parc de capteurs ===".to_string(),
            format!("Total capteurs   : {total}"),
            format!("En ligne         : {actifs}"),
            format!("Hors ligne       : {hors_ligne}"),
            format!("Alertes actives  : {nb_alertes} ({alertes_critiques} critiques)"),
            String::new(),
        ];

        if total > 0 {
            lignes.push("--- Détail des capteurs ---".to_string());
            let mut capteurs: Vec<&InfoCapteur> = self.capteurs.values().collect();
            capteurs.sort_by(|a, b| a.id.cmp(&b.id));
            for capteur in capteurs {
                lignes.push(format!("  {}", capteur.resume_court()));
            }
        }

        if !self.alertes.is_empty() {
            lignes.push(String::new());
            lignes.push("--- Alertes récentes ---".to_string());
            for alerte in self.alertes.iter().rev().take(5) {
                lignes.push(format!(
                    "  [Sev. {}] [{}] {} — {}",
                    alerte.severite,
                    alerte.type_alerte.libelle(),
                    alerte.capteur_id,
                    alerte.message
                ));
            }
        }

        lignes.join("\n")
    }
}

impl Default for RegistreCapteurs {
    fn default() -> Self {
        Self::nouveau()
    }
}

/// Vérifie si une valeur est dans la plage normale pour son type de capteur.
/// Retourne une `Alerte` si la valeur est anormale, sinon `None`.
fn verifier_plage_valeur(capteur_id: &str, type_capteur: &TypeCapteur, valeur: f64) -> Option<Alerte> {
    let (min, max, unite, libelle) = match type_capteur {
        TypeCapteur::Temperature   => (TEMP_MIN,         TEMP_MAX,         "°C",  "Température"),
        TypeCapteur::Humidite      => (HUMIDITE_MIN,      HUMIDITE_MAX,     "%",   "Humidité de l'air"),
        TypeCapteur::HumiditeSol   => (HUMIDITE_SOL_MIN,  HUMIDITE_SOL_MAX, "%",   "Humidité du sol"),
        TypeCapteur::Ph            => (PH_MIN,            PH_MAX,           "",    "pH"),
        TypeCapteur::Luminosite    => (LUX_MIN,           LUX_MAX,          "lux", "Luminosité"),
        TypeCapteur::Pression      => (PRESSION_MIN,      PRESSION_MAX,     "hPa", "Pression atmosphérique"),
        TypeCapteur::Vent          => (VENT_MIN,          VENT_MAX,         "km/h","Vitesse du vent"),
        TypeCapteur::Pluie         => (PLUIE_MIN,         PLUIE_MAX,        "mm",  "Pluviométrie"),
        TypeCapteur::Co2           => (CO2_MIN,           CO2_MAX,          "ppm", "Concentration CO2"),
    };

    if valeur < min || valeur > max {
        let message = format!(
            "{libelle} anormale : {valeur:.2} {unite} (plage normale : {min:.1}..{max:.1} {unite}). \
             Vérifier le capteur ou les conditions terrain."
        );
        Some(Alerte::nouvelle(capteur_id, TypeAlerte::ValeurAnormale, message, 3))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enregistrer_capteur() {
        let mut registre = RegistreCapteurs::nouveau();
        registre.enregistrer(
            "T-001",
            "Capteur Température Serre A",
            TypeCapteur::Temperature,
            "Serre A — Rangée 3",
        );

        assert_eq!(registre.nb_capteurs(), 1);
        let info = registre.etat_capteur("T-001").unwrap();
        assert_eq!(info.id, "T-001");
        assert_eq!(info.etat, EtatCapteur::EnLigne);
        assert!(info.derniere_lecture.is_none());
    }

    #[test]
    fn test_recevoir_lecture() {
        let mut registre = RegistreCapteurs::nouveau();
        registre.enregistrer("H-001", "Hygromètre Sol", TypeCapteur::HumiditeSol, "Parcelle B");
        registre.recevoir_lecture("H-001", 45.0, "%");

        let info = registre.etat_capteur("H-001").unwrap();
        let lecture = info.derniere_lecture.as_ref().unwrap();
        assert!((lecture.valeur - 45.0).abs() < f64::EPSILON);
        assert_eq!(lecture.unite, "%");
    }

    #[test]
    fn test_capteur_hors_ligne() {
        let mut registre = RegistreCapteurs::nouveau();
        registre.enregistrer(
            "P-001",
            "Pluviomètre Toit",
            TypeCapteur::Pluie,
            "Toit Station Météo",
        );
        assert_eq!(registre.capteurs_hors_ligne().len(), 0);

        registre.marquer_hors_ligne("P-001");

        assert_eq!(registre.capteurs_hors_ligne().len(), 1);
        assert_eq!(registre.capteurs_actifs().len(), 0);
        let info = registre.etat_capteur("P-001").unwrap();
        assert_eq!(info.etat, EtatCapteur::HorsLigne);
    }

    #[test]
    fn test_alertes_batterie() {
        let mut registre = RegistreCapteurs::nouveau();
        registre.enregistrer("C-001", "Capteur CO2 Serre", TypeCapteur::Co2, "Serre B");

        // Batterie encore correcte — pas d'alerte
        registre.mettre_a_jour_batterie("C-001", 50.0);
        assert_eq!(registre.alertes().len(), 0);

        // Batterie faible — alerte générée
        registre.mettre_a_jour_batterie("C-001", 15.0);
        assert_eq!(registre.alertes().len(), 1);
        assert_eq!(registre.alertes()[0].type_alerte, TypeAlerte::BatterieFaible);
        assert_eq!(registre.alertes()[0].severite, 3);

        let info = registre.etat_capteur("C-001").unwrap();
        assert_eq!(info.etat, EtatCapteur::BatterieFaible);
    }

    #[test]
    fn test_alerte_valeur_anormale() {
        let mut registre = RegistreCapteurs::nouveau();
        registre.enregistrer("T-002", "Capteur Temp Extérieur", TypeCapteur::Temperature, "Extérieur");

        // Valeur normale — pas d'alerte
        registre.recevoir_lecture("T-002", 25.0, "°C");
        assert_eq!(registre.alertes().len(), 0);

        // Valeur hors plage — alerte générée
        registre.recevoir_lecture("T-002", 85.0, "°C");
        assert_eq!(registre.alertes().len(), 1);
        assert_eq!(registre.alertes()[0].type_alerte, TypeAlerte::ValeurAnormale);
    }

    #[test]
    fn test_capteurs_actifs_multiples() {
        let mut registre = RegistreCapteurs::nouveau();
        registre.enregistrer("T-010", "Temp A", TypeCapteur::Temperature, "Zone A");
        registre.enregistrer("T-011", "Temp B", TypeCapteur::Temperature, "Zone B");
        registre.enregistrer("T-012", "Temp C", TypeCapteur::Temperature, "Zone C");

        registre.marquer_hors_ligne("T-011");

        assert_eq!(registre.capteurs_actifs().len(), 2);
        assert_eq!(registre.capteurs_hors_ligne().len(), 1);
    }

    #[test]
    fn test_resume_format() {
        let mut registre = RegistreCapteurs::nouveau();
        registre.enregistrer("T-001", "Capteur Temp", TypeCapteur::Temperature, "Nord");
        registre.recevoir_lecture("T-001", 18.5, "°C");

        let resume = registre.resume();
        assert!(resume.contains("Etat du parc de capteurs"));
        assert!(resume.contains("Total capteurs"));
        assert!(resume.contains("T-001"));
    }

    #[test]
    fn test_reconnexion_capteur() {
        let mut registre = RegistreCapteurs::nouveau();
        registre.enregistrer("W-001", "Anémomètre", TypeCapteur::Vent, "Mat Météo");

        registre.marquer_hors_ligne("W-001");
        assert_eq!(registre.etat_capteur("W-001").unwrap().etat, EtatCapteur::HorsLigne);

        // Une lecture remet le capteur en ligne
        registre.recevoir_lecture("W-001", 15.0, "km/h");
        assert_eq!(registre.etat_capteur("W-001").unwrap().etat, EtatCapteur::EnLigne);
    }
}
