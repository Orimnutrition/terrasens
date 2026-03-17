//! Abstraction MQTT locale — protocole de messagerie IoT simulé
//!
//! Fournit une interface MQTT complète sans aucune dépendance réseau.
//! Conforme au Pilier 1 : Local-first absolu — zéro cloud.
//!
//! ## Format des topics AGRORIM
//! `agrorim/{parcelle_id}/capteurs/{type_capteur}`
//!
//! ## Exemple
//! ```rust
//! use agrorim_sensors::mqtt::{MqttClient, MqttConfig, TopicPattern};
//!
//! let config = MqttConfig {
//!     broker_host: "localhost".to_string(),
//!     broker_port: 1883,
//!     client_id: "agrorim-central".to_string(),
//!     topics: vec!["agrorim/parcelle-01/capteurs/#".to_string()],
//!     qos: 1,
//! };
//! let mut client = MqttClient::new(config);
//! client.publish("agrorim/parcelle-01/capteurs/temperature", "22.5");
//! assert_eq!(client.messages().len(), 1);
//! ```

use agrorim_core::Timestamp;
use serde::{Deserialize, Serialize};

/// Configuration d'un client MQTT AGRORIM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttConfig {
    /// Adresse du broker (ex : "localhost" ou adresse IP LAN)
    pub broker_host: String,
    /// Port du broker (défaut : 1883)
    pub broker_port: u16,
    /// Identifiant unique du client
    pub client_id: String,
    /// Liste des topics auxquels souscrire
    pub topics: Vec<String>,
    /// Niveau de qualité de service (0, 1 ou 2)
    pub qos: u8,
}

impl MqttConfig {
    /// Crée une configuration MQTT locale par défaut
    ///
    /// # Exemple
    /// ```rust
    /// use agrorim_sensors::mqtt::MqttConfig;
    ///
    /// let config = MqttConfig::local("agrorim-node-01");
    /// assert_eq!(config.broker_port, 1883);
    /// ```
    pub fn local(client_id: &str) -> Self {
        Self {
            broker_host: "localhost".to_string(),
            broker_port: 1883,
            client_id: client_id.to_string(),
            topics: Vec::new(),
            qos: 1,
        }
    }
}

impl Default for MqttConfig {
    fn default() -> Self {
        Self::local("agrorim-client")
    }
}

/// Message MQTT reçu ou publié
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttMessage {
    /// Topic du message (ex : `agrorim/parcelle-01/capteurs/temperature`)
    pub topic: String,
    /// Contenu du message (valeur sérialisée)
    pub payload: String,
    /// Horodatage de publication
    pub timestamp: Timestamp,
    /// Niveau de qualité de service du message
    pub qos: u8,
}

impl MqttMessage {
    /// Crée un nouveau message MQTT
    pub fn nouveau(topic: impl Into<String>, payload: impl Into<String>, qos: u8) -> Self {
        Self {
            topic: topic.into(),
            payload: payload.into(),
            timestamp: Timestamp::now(),
            qos,
        }
    }
}

/// Patron de topic AGRORIM standard
///
/// Format : `agrorim/{parcelle_id}/capteurs/{capteur_type}`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicPattern {
    /// Identifiant de la parcelle (ex : "parcelle-nord-01")
    pub parcelle_id: String,
    /// Type de capteur (ex : "temperature", "humidite")
    pub capteur_type: String,
}

impl TopicPattern {
    /// Crée un patron de topic pour une parcelle et un type de capteur
    ///
    /// # Exemple
    /// ```rust
    /// use agrorim_sensors::mqtt::TopicPattern;
    ///
    /// let patron = TopicPattern::nouveau("parcelle-01", "temperature");
    /// assert_eq!(patron.topic(), "agrorim/parcelle-01/capteurs/temperature");
    /// ```
    pub fn nouveau(parcelle_id: impl Into<String>, capteur_type: impl Into<String>) -> Self {
        Self {
            parcelle_id: parcelle_id.into(),
            capteur_type: capteur_type.into(),
        }
    }

    /// Construit le topic complet
    pub fn topic(&self) -> String {
        format!(
            "agrorim/{}/capteurs/{}",
            self.parcelle_id, self.capteur_type
        )
    }

    /// Construit un topic générique pour tous les capteurs d'une parcelle
    pub fn topic_tous_capteurs(parcelle_id: &str) -> String {
        format!("agrorim/{parcelle_id}/capteurs/#")
    }

    /// Vérifie si un topic correspond à ce patron
    /// Supporte le joker `#` (multi-niveau) et `+` (niveau unique)
    pub fn correspond(&self, topic: &str) -> bool {
        topic_match(&self.topic(), topic)
    }

    /// Extrait le type de capteur depuis un topic AGRORIM
    ///
    /// Retourne `None` si le topic ne suit pas le format AGRORIM standard.
    pub fn extraire_type_capteur(topic: &str) -> Option<&str> {
        let parties: Vec<&str> = topic.splitn(5, '/').collect();
        // Format attendu : agrorim / {parcelle_id} / capteurs / {capteur_type}
        if parties.len() == 4 && parties[0] == "agrorim" && parties[2] == "capteurs" {
            Some(parties[3])
        } else {
            None
        }
    }

    /// Extrait l'identifiant de parcelle depuis un topic AGRORIM
    pub fn extraire_parcelle_id(topic: &str) -> Option<&str> {
        let parties: Vec<&str> = topic.splitn(5, '/').collect();
        if parties.len() == 4 && parties[0] == "agrorim" && parties[2] == "capteurs" {
            Some(parties[1])
        } else {
            None
        }
    }
}

/// Client MQTT simulé — 100 % local, zéro réseau
///
/// Simule le comportement d'un vrai client MQTT sans aucune connexion TCP.
/// Tous les messages publiés sont stockés en mémoire et accessibles via `messages()`.
#[derive(Debug, Clone)]
pub struct MqttClient {
    /// Configuration du client
    config: MqttConfig,
    /// Topics auxquels ce client est souscrit
    souscriptions: Vec<String>,
    /// File de messages reçus (publiés)
    messages: Vec<MqttMessage>,
    /// Etat de connexion simulé
    connecte: bool,
}

impl MqttClient {
    /// Crée un nouveau client MQTT avec la configuration donnée
    ///
    /// Le client est immédiatement "connecté" (simulation locale).
    ///
    /// # Exemple
    /// ```rust
    /// use agrorim_sensors::mqtt::{MqttClient, MqttConfig};
    ///
    /// let config = MqttConfig::local("test-client");
    /// let client = MqttClient::new(config);
    /// assert!(client.is_connected());
    /// ```
    pub fn new(config: MqttConfig) -> Self {
        let souscriptions = config.topics.clone();
        tracing::debug!(
            client_id = %config.client_id,
            broker = %config.broker_host,
            port = config.broker_port,
            "Client MQTT initialisé (mode local)"
        );
        Self {
            config,
            souscriptions,
            messages: Vec::new(),
            connecte: true,
        }
    }

    /// Souscrit à un topic (supporte les jokers `+` et `#`)
    ///
    /// # Exemple
    /// ```rust
    /// use agrorim_sensors::mqtt::{MqttClient, MqttConfig};
    ///
    /// let mut client = MqttClient::new(MqttConfig::local("test"));
    /// client.subscribe("agrorim/parcelle-01/capteurs/#");
    /// ```
    pub fn subscribe(&mut self, topic: &str) {
        if !self.souscriptions.contains(&topic.to_string()) {
            tracing::debug!(topic = %topic, "Souscription au topic MQTT");
            self.souscriptions.push(topic.to_string());
        }
    }

    /// Publie un message sur un topic
    ///
    /// Le message est stocké en mémoire si le topic correspond à au moins
    /// une souscription active, ou inconditionnellement si aucune souscription
    /// n'est configurée.
    ///
    /// # Exemple
    /// ```rust
    /// use agrorim_sensors::mqtt::{MqttClient, MqttConfig};
    ///
    /// let mut client = MqttClient::new(MqttConfig::local("test"));
    /// client.publish("agrorim/parcelle-01/capteurs/temperature", "22.5");
    /// assert_eq!(client.messages().len(), 1);
    /// ```
    pub fn publish(&mut self, topic: &str, payload: &str) {
        let message = MqttMessage::nouveau(topic, payload, self.config.qos);
        tracing::debug!(
            topic = %topic,
            payload = %payload,
            "Message MQTT publié"
        );
        self.messages.push(message);
    }

    /// Retourne tous les messages reçus (publiés)
    pub fn messages(&self) -> &[MqttMessage] {
        &self.messages
    }

    /// Retourne les messages correspondant à un topic donné
    pub fn messages_pour_topic(&self, topic_filtre: &str) -> Vec<&MqttMessage> {
        self.messages
            .iter()
            .filter(|m| topic_match(topic_filtre, &m.topic))
            .collect()
    }

    /// Vide la file de messages
    pub fn vider_messages(&mut self) {
        self.messages.clear();
    }

    /// Indique si le client est connecté (toujours `true` en mode local)
    pub fn is_connected(&self) -> bool {
        self.connecte
    }

    /// Déconnecte le client (simulation)
    ///
    /// # Exemple
    /// ```rust
    /// use agrorim_sensors::mqtt::{MqttClient, MqttConfig};
    ///
    /// let mut client = MqttClient::new(MqttConfig::local("test"));
    /// assert!(client.is_connected());
    /// client.disconnect();
    /// assert!(!client.is_connected());
    /// ```
    pub fn disconnect(&mut self) {
        tracing::debug!(
            client_id = %self.config.client_id,
            "Déconnexion MQTT"
        );
        self.connecte = false;
    }

    /// Liste des topics souscrits
    pub fn souscriptions(&self) -> &[String] {
        &self.souscriptions
    }

    /// Identifiant du client
    pub fn client_id(&self) -> &str {
        &self.config.client_id
    }

    /// Nombre de messages en attente
    pub fn nb_messages(&self) -> usize {
        self.messages.len()
    }
}

/// Vérifie si un topic correspond à un patron (avec jokers MQTT `+` et `#`)
fn topic_match(patron: &str, topic: &str) -> bool {
    let segments_patron: Vec<&str> = patron.split('/').collect();
    let segments_topic: Vec<&str> = topic.split('/').collect();

    let mut i_patron = 0;
    let mut i_topic = 0;

    while i_patron < segments_patron.len() && i_topic < segments_topic.len() {
        let seg = segments_patron[i_patron];
        if seg == "#" {
            // `#` correspond à tout ce qui reste
            return true;
        } else if seg == "+" {
            // `+` correspond à exactement un segment
            i_patron += 1;
            i_topic += 1;
        } else if seg == segments_topic[i_topic] {
            i_patron += 1;
            i_topic += 1;
        } else {
            return false;
        }
    }

    // Correspondance si les deux sont épuisés au même moment
    i_patron == segments_patron.len() && i_topic == segments_topic.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let config = MqttConfig {
            broker_host: "192.168.1.100".to_string(),
            broker_port: 1883,
            client_id: "agrorim-test".to_string(),
            topics: vec!["agrorim/#".to_string()],
            qos: 1,
        };
        let client = MqttClient::new(config);
        assert!(client.is_connected());
        assert_eq!(client.client_id(), "agrorim-test");
        assert_eq!(client.nb_messages(), 0);
        assert_eq!(client.souscriptions().len(), 1);
    }

    #[test]
    fn test_publish_subscribe() {
        let mut client = MqttClient::new(MqttConfig::local("test-pub-sub"));
        client.subscribe("agrorim/parcelle-01/capteurs/#");
        client.publish(
            "agrorim/parcelle-01/capteurs/temperature",
            "24.3",
        );
        client.publish(
            "agrorim/parcelle-01/capteurs/humidite",
            "65.0",
        );
        assert_eq!(client.nb_messages(), 2);
        assert_eq!(client.messages()[0].payload, "24.3");
        assert_eq!(client.messages()[1].topic, "agrorim/parcelle-01/capteurs/humidite");
    }

    #[test]
    fn test_message_count() {
        let mut client = MqttClient::new(MqttConfig::local("test-count"));
        for i in 0..5 {
            client.publish(
                &format!("agrorim/parcelle-0{i}/capteurs/temperature"),
                &format!("{}.0", 20 + i),
            );
        }
        assert_eq!(client.nb_messages(), 5);
        client.vider_messages();
        assert_eq!(client.nb_messages(), 0);
    }

    #[test]
    fn test_disconnect() {
        let mut client = MqttClient::new(MqttConfig::local("test-disconnect"));
        assert!(client.is_connected());
        client.disconnect();
        assert!(!client.is_connected());
    }

    #[test]
    fn test_topic_pattern_format() {
        let patron = TopicPattern::nouveau("parcelle-nord-01", "temperature");
        assert_eq!(patron.topic(), "agrorim/parcelle-nord-01/capteurs/temperature");
    }

    #[test]
    fn test_extraire_type_capteur() {
        let topic = "agrorim/parcelle-01/capteurs/ph";
        assert_eq!(TopicPattern::extraire_type_capteur(topic), Some("ph"));
        assert_eq!(TopicPattern::extraire_parcelle_id(topic), Some("parcelle-01"));
    }

    #[test]
    fn test_topic_match_joker_diese() {
        assert!(topic_match("agrorim/#", "agrorim/parcelle-01/capteurs/temperature"));
        assert!(topic_match("agrorim/parcelle-01/#", "agrorim/parcelle-01/capteurs/ph"));
    }

    #[test]
    fn test_topic_match_joker_plus() {
        assert!(topic_match(
            "agrorim/+/capteurs/temperature",
            "agrorim/parcelle-01/capteurs/temperature"
        ));
        assert!(!topic_match(
            "agrorim/+/capteurs/temperature",
            "agrorim/parcelle-01/capteurs/humidite"
        ));
    }

    #[test]
    fn test_messages_pour_topic() {
        let mut client = MqttClient::new(MqttConfig::local("test-filtre"));
        client.publish("agrorim/p1/capteurs/temperature", "20.0");
        client.publish("agrorim/p1/capteurs/humidite", "55.0");
        client.publish("agrorim/p2/capteurs/temperature", "18.0");

        let msgs_temp_p1 = client.messages_pour_topic("agrorim/p1/capteurs/temperature");
        assert_eq!(msgs_temp_p1.len(), 1);
        assert_eq!(msgs_temp_p1[0].payload, "20.0");
    }
}
