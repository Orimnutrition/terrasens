//! # AGRORIM Auth
//!
//! Authentification terrain — 4 couches souveraines.
//! L'agriculteur prouve qu'il connaît SA terre. Pas de mot de passe cloud.
//!
//! ## 4 Couches
//! 1. Géolocalisation (est-il sur sa parcelle ?)
//! 2. Questions terrain dynamiques (quel arbre au bout du chemin ?)
//! 3. Vérification sensorielle (photo du champ)
//! 4. Empreinte comportementale (habitudes d'utilisation)

use serde::{Deserialize, Serialize};
use agrorim_core::GeoPoint;

/// Résultat d'authentification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResultatAuth {
    pub authentifie: bool,
    pub score: f64,
    pub couches_validees: Vec<CoucheAuth>,
    pub raison_refus: Option<String>,
}

/// Les 4 couches d'authentification terrain
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoucheAuth {
    /// L'utilisateur est physiquement sur la parcelle
    Geolocalisation,
    /// L'utilisateur répond correctement aux questions sur son terrain
    QuestionsTerrain,
    /// Vérification par photo du terrain
    VerificationSensorielle,
    /// Pattern d'utilisation habituel
    EmpreinteComportementale,
}

/// Question de terrain pour authentification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionTerrain {
    pub question: String,
    pub reponse_attendue: String,
    pub tolerance: f64, // 0.0 = exact, 1.0 = très tolérant
}

/// Authentificateur terrain AGRORIM
pub struct Authentificateur {
    /// Position connue de la parcelle
    position_parcelle: GeoPoint,
    /// Rayon autorisé en mètres
    rayon_metres: f64,
    /// Questions terrain enregistrées
    questions: Vec<QuestionTerrain>,
}

impl Authentificateur {
    pub fn new(position_parcelle: GeoPoint, rayon_metres: f64) -> Self {
        Self {
            position_parcelle,
            rayon_metres,
            questions: Vec::new(),
        }
    }

    /// Ajoute une question terrain
    pub fn ajouter_question(&mut self, question: QuestionTerrain) {
        self.questions.push(question);
    }

    /// Vérifie la couche 1 : géolocalisation
    pub fn verifier_geolocalisation(&self, position: &GeoPoint) -> bool {
        let distance = self.position_parcelle.distance_to(position);
        distance <= self.rayon_metres
    }

    /// Vérifie la couche 2 : question terrain
    pub fn verifier_question(&self, index: usize, reponse: &str) -> bool {
        if let Some(q) = self.questions.get(index) {
            let reponse_norm = reponse.to_lowercase().trim().to_string();
            let attendu_norm = q.reponse_attendue.to_lowercase().trim().to_string();

            if q.tolerance == 0.0 {
                reponse_norm == attendu_norm
            } else {
                // Correspondance partielle
                reponse_norm.contains(&attendu_norm) || attendu_norm.contains(&reponse_norm)
            }
        } else {
            false
        }
    }

    /// Authentification complète
    pub fn authentifier(
        &self,
        position: Option<&GeoPoint>,
        reponses: &[(usize, String)],
    ) -> ResultatAuth {
        let mut couches_validees = Vec::new();
        let mut score = 0.0;

        // Couche 1 : Géolocalisation
        if let Some(pos) = position {
            if self.verifier_geolocalisation(pos) {
                couches_validees.push(CoucheAuth::Geolocalisation);
                score += 30.0;
            }
        }

        // Couche 2 : Questions terrain
        if !reponses.is_empty() {
            let mut bonnes = 0;
            for (idx, reponse) in reponses {
                if self.verifier_question(*idx, reponse) {
                    bonnes += 1;
                }
            }
            let ratio = bonnes as f64 / reponses.len() as f64;
            if ratio >= 0.7 {
                couches_validees.push(CoucheAuth::QuestionsTerrain);
                score += 40.0 * ratio;
            }
        }

        // Seuil : au moins 50% pour être authentifié
        let authentifie = score >= 50.0;

        ResultatAuth {
            authentifie,
            score,
            couches_validees,
            raison_refus: if !authentifie {
                Some("Score insuffisant — vérifiez votre position et vos réponses".into())
            } else {
                None
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn auth_test() -> Authentificateur {
        let position = GeoPoint::new(46.2044, 6.1432); // Genève
        let mut auth = Authentificateur::new(position, 500.0);

        auth.ajouter_question(QuestionTerrain {
            question: "Quel arbre est au bout du chemin ?".into(),
            reponse_attendue: "chêne".into(),
            tolerance: 0.5,
        });

        auth.ajouter_question(QuestionTerrain {
            question: "Quelle rivière borde votre parcelle ?".into(),
            reponse_attendue: "arve".into(),
            tolerance: 0.5,
        });

        auth
    }

    #[test]
    fn test_geolocalisation_ok() {
        let auth = auth_test();
        let pos = GeoPoint::new(46.2045, 6.1433); // Très proche
        assert!(auth.verifier_geolocalisation(&pos));
    }

    #[test]
    fn test_geolocalisation_trop_loin() {
        let auth = auth_test();
        let pos = GeoPoint::new(47.3769, 8.5417); // Zurich
        assert!(!auth.verifier_geolocalisation(&pos));
    }

    #[test]
    fn test_question_terrain() {
        let auth = auth_test();
        assert!(auth.verifier_question(0, "chêne"));
        assert!(auth.verifier_question(0, "Chêne")); // Case insensitive
        assert!(!auth.verifier_question(0, "peuplier"));
    }

    #[test]
    fn test_authentification_complete() {
        let auth = auth_test();
        let pos = GeoPoint::new(46.2045, 6.1433);
        let reponses = vec![(0, "chêne".into()), (1, "arve".into())];

        let resultat = auth.authentifier(Some(&pos), &reponses);
        assert!(resultat.authentifie);
        assert!(resultat.score >= 50.0);
    }

    #[test]
    fn test_authentification_echouee() {
        let auth = auth_test();
        let pos = GeoPoint::new(47.3769, 8.5417); // Trop loin
        let reponses = vec![(0, "peuplier".into())]; // Mauvaise réponse

        let resultat = auth.authentifier(Some(&pos), &reponses);
        assert!(!resultat.authentifie);
        assert!(resultat.raison_refus.is_some());
    }
}
