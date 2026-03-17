//! Service capteurs AGRORIM — API de haut niveau
//!
//! Point d'entrée principal du module capteurs. Combine le client MQTT simulé
//! et le registre de capteurs en une interface unifiée et ergonomique.
//!
//! ## Exemple complet
//! ```rust
//! use agrorim_sensors::TypeCapteur;
//! use agrorim_sensors::mqtt::MqttConfig;
//! use agrorim_sensors::service::CapteurService;
//!
//! let mut service = CapteurService::new();
//!
//! let config = MqttConfig::local("agrorim-ferme-alpha");
//! service.configurer_mqtt(config);
//!
//! service.ajouter_capteur("T-001", "Capteur Température Serre", TypeCapteur::Temperature, "Serre A");
//! service.simuler_lecture("T-001", 23.7, "°C");
//!
//! assert_eq!(service.nb_capteurs(), 1);
//! assert_eq!(service.nb_alertes(), 0);
//! println!("{}", service.resume());
//! ```

use crate::mqtt::{MqttClient, MqttConfig, TopicPattern};
use crate::registre::RegistreCapteurs;
use crate::TypeCapteur;

/// Service principal de gestion des capteurs AGRORIM
///
/// Orchestre le client MQTT simulé et le registre de capteurs.
/// Chaque simulation de lecture publie automatiquement un message MQTT
/// et met à jour l'état du capteur dans le registre.
pub struct CapteurService {
    /// Client MQTT local (simulation)
    mqtt: MqttClient,
    /// Registre de l'état des capteurs
    registre: RegistreCapteurs,
}

impl CapteurService {
    /// Crée un nouveau service capteurs avec configuration MQTT par défaut
    ///
    /// # Exemple
    /// ```rust
    /// use agrorim_sensors::service::CapteurService;
    ///
    /// let service = CapteurService::new();
    /// assert_eq!(service.nb_capteurs(), 0);
    /// ```
    pub fn new() -> Self {
        let config = MqttConfig::local("agrorim-service-capteurs");
        Self {
            mqtt: MqttClient::new(config),
            registre: RegistreCapteurs::nouveau(),
        }
    }

    /// Reconfigure le client MQTT avec de nouveaux paramètres
    ///
    /// # Exemple
    /// ```rust
    /// use agrorim_sensors::mqtt::MqttConfig;
    /// use agrorim_sensors::service::CapteurService;
    ///
    /// let mut service = CapteurService::new();
    /// let config = MqttConfig {
    ///     broker_host: "192.168.10.1".to_string(),
    ///     broker_port: 1883,
    ///     client_id: "agrorim-ferme-nord".to_string(),
    ///     topics: vec!["agrorim/#".to_string()],
    ///     qos: 1,
    /// };
    /// service.configurer_mqtt(config);
    /// ```
    pub fn configurer_mqtt(&mut self, config: MqttConfig) {
        self.mqtt = MqttClient::new(config);
    }

    /// Enregistre un nouveau capteur dans le service
    ///
    /// Souscrit automatiquement au topic MQTT correspondant.
    ///
    /// # Exemple
    /// ```rust
    /// use agrorim_sensors::TypeCapteur;
    /// use agrorim_sensors::service::CapteurService;
    ///
    /// let mut service = CapteurService::new();
    /// service.ajouter_capteur("PH-001", "pH Mètre Parcelle Ouest", TypeCapteur::Ph, "Parcelle Ouest");
    /// assert_eq!(service.nb_capteurs(), 1);
    /// ```
    pub fn ajouter_capteur(
        &mut self,
        id: &str,
        nom: &str,
        type_capteur: TypeCapteur,
        localisation: &str,
    ) {
        // Souscription au topic MQTT générique de la parcelle
        let topic = format!("agrorim/+/capteurs/{}", type_capteur_slug(&type_capteur));
        self.mqtt.subscribe(&topic);

        self.registre.enregistrer(id, nom, type_capteur, localisation);
    }

    /// Simule une lecture capteur — publie le message MQTT et met à jour le registre
    ///
    /// La valeur est publiée sur le topic AGRORIM standard :
    /// `agrorim/{capteur_id}/capteurs/{type}`
    ///
    /// # Exemple
    /// ```rust
    /// use agrorim_sensors::TypeCapteur;
    /// use agrorim_sensors::service::CapteurService;
    ///
    /// let mut service = CapteurService::new();
    /// service.ajouter_capteur("H-001", "Hygromètre", TypeCapteur::Humidite, "Station Météo");
    /// service.simuler_lecture("H-001", 68.0, "%");
    ///
    /// // Le message MQTT a été publié
    /// assert_eq!(service.nb_messages_mqtt(), 1);
    /// ```
    pub fn simuler_lecture(&mut self, capteur_id: &str, valeur: f64, unite: &str) {
        // Publier le message MQTT
        if let Some(info) = self.registre.etat_capteur(capteur_id) {
            let type_slug = type_capteur_slug(&info.type_capteur.clone());
            let topic = TopicPattern::nouveau(capteur_id, type_slug).topic();
            let payload = format!(
                r#"{{"capteur_id":"{capteur_id}","valeur":{valeur:.4},"unite":"{unite}"}}"#
            );
            self.mqtt.publish(&topic, &payload);
        }

        // Mettre à jour le registre
        self.registre.recevoir_lecture(capteur_id, valeur, unite);
    }

    /// Simule une mise à jour du niveau de batterie d'un capteur
    pub fn simuler_batterie(&mut self, capteur_id: &str, batterie_pct: f64) {
        let topic = format!("agrorim/{capteur_id}/batterie");
        let payload = format!(r#"{{"capteur_id":"{capteur_id}","batterie_pct":{batterie_pct:.1}}}"#);
        self.mqtt.publish(&topic, &payload);
        self.registre.mettre_a_jour_batterie(capteur_id, batterie_pct);
    }

    /// Simule une déconnexion capteur
    pub fn simuler_hors_ligne(&mut self, capteur_id: &str) {
        let topic = format!("agrorim/{capteur_id}/statut");
        let payload = format!(r#"{{"capteur_id":"{capteur_id}","statut":"hors_ligne"}}"#);
        self.mqtt.publish(&topic, &payload);
        self.registre.marquer_hors_ligne(capteur_id);
    }

    /// Retourne un résumé complet du service en français
    ///
    /// # Exemple
    /// ```rust
    /// use agrorim_sensors::service::CapteurService;
    ///
    /// let service = CapteurService::new();
    /// let resume = service.resume();
    /// assert!(resume.contains("AGRORIM"));
    /// ```
    pub fn resume(&self) -> String {
        let mqtt_connecte = if self.mqtt.is_connected() {
            "connecté (local)"
        } else {
            "déconnecté"
        };
        let entete = format!(
            "=== Service Capteurs AGRORIM ===\nClient MQTT : {} | Messages : {}\n",
            mqtt_connecte,
            self.mqtt.nb_messages()
        );
        format!("{}\n{}", entete, self.registre.resume())
    }

    /// Nombre de capteurs enregistrés
    pub fn nb_capteurs(&self) -> usize {
        self.registre.nb_capteurs()
    }

    /// Nombre d'alertes actives
    pub fn nb_alertes(&self) -> usize {
        self.registre.alertes().len()
    }

    /// Nombre de messages MQTT publiés
    pub fn nb_messages_mqtt(&self) -> usize {
        self.mqtt.nb_messages()
    }

    /// Accès en lecture au registre de capteurs
    pub fn registre(&self) -> &RegistreCapteurs {
        &self.registre
    }

    /// Accès en lecture au client MQTT
    pub fn mqtt(&self) -> &MqttClient {
        &self.mqtt
    }
}

impl Default for CapteurService {
    fn default() -> Self {
        Self::new()
    }
}

/// Convertit un `TypeCapteur` en slug de topic MQTT lowercase
fn type_capteur_slug(type_capteur: &TypeCapteur) -> &'static str {
    match type_capteur {
        TypeCapteur::Temperature => "temperature",
        TypeCapteur::Humidite    => "humidite",
        TypeCapteur::HumiditeSol => "humidite-sol",
        TypeCapteur::Ph          => "ph",
        TypeCapteur::Luminosite  => "luminosite",
        TypeCapteur::Pression    => "pression",
        TypeCapteur::Vent        => "vent",
        TypeCapteur::Pluie       => "pluie",
        TypeCapteur::Co2         => "co2",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mqtt::MqttConfig;

    #[test]
    fn test_service_creation() {
        let service = CapteurService::new();
        assert_eq!(service.nb_capteurs(), 0);
        assert_eq!(service.nb_alertes(), 0);
        assert_eq!(service.nb_messages_mqtt(), 0);
    }

    #[test]
    fn test_simulation_complete() {
        let mut service = CapteurService::new();

        // Configurer MQTT
        service.configurer_mqtt(MqttConfig::local("ferme-test-01"));

        // Enregistrer plusieurs capteurs
        service.ajouter_capteur("T-001", "Temp Serre A", TypeCapteur::Temperature, "Serre A");
        service.ajouter_capteur("H-001", "Humidite Sol B", TypeCapteur::HumiditeSol, "Parcelle B");
        service.ajouter_capteur("PH-001", "pH Bassin", TypeCapteur::Ph, "Bassin Irrigation");

        assert_eq!(service.nb_capteurs(), 3);

        // Simuler des lectures normales
        service.simuler_lecture("T-001", 24.5, "°C");
        service.simuler_lecture("H-001", 52.0, "%");
        service.simuler_lecture("PH-001", 6.8, "");

        // 3 messages MQTT publiés
        assert_eq!(service.nb_messages_mqtt(), 3);

        // Aucune alerte — valeurs dans les plages normales
        assert_eq!(service.nb_alertes(), 0);

        // Les capteurs sont actifs
        assert_eq!(service.registre().capteurs_actifs().len(), 3);
    }

    #[test]
    fn test_simulation_batterie_faible() {
        let mut service = CapteurService::new();
        service.ajouter_capteur("CO2-001", "Capteur CO2", TypeCapteur::Co2, "Serre C");

        service.simuler_batterie("CO2-001", 10.0);

        assert_eq!(service.nb_alertes(), 1);
        assert_eq!(service.nb_messages_mqtt(), 1);
    }

    #[test]
    fn test_simulation_hors_ligne() {
        let mut service = CapteurService::new();
        service.ajouter_capteur("L-001", "Lux Mètre", TypeCapteur::Luminosite, "Extérieur Est");

        service.simuler_hors_ligne("L-001");

        assert_eq!(service.registre().capteurs_hors_ligne().len(), 1);
        assert_eq!(service.nb_alertes(), 1);
    }

    #[test]
    fn test_resume_service() {
        let mut service = CapteurService::new();
        service.ajouter_capteur("V-001", "Anémomètre", TypeCapteur::Vent, "Toit");
        service.simuler_lecture("V-001", 30.0, "km/h");

        let resume = service.resume();
        assert!(resume.contains("Service Capteurs AGRORIM"));
        assert!(resume.contains("connecté"));
        assert!(resume.contains("V-001"));
    }

    #[test]
    fn test_mqtt_topic_publie_correctement() {
        let mut service = CapteurService::new();
        service.ajouter_capteur("T-009", "Temp Ext", TypeCapteur::Temperature, "Extérieur");
        service.simuler_lecture("T-009", 18.0, "°C");

        let messages = service.mqtt().messages();
        assert_eq!(messages.len(), 1);
        // Le topic doit suivre le format AGRORIM standard
        let topic = &messages[0].topic;
        assert!(topic.starts_with("agrorim/T-009/capteurs/"));
        assert!(topic.ends_with("temperature"));
    }
}
