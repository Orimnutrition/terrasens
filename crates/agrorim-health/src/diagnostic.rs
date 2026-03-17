//! Base de connaissances phytosanitaires
//!
//! 15+ pathologies courantes avec symptômes, traitements bio et sources.
//! Sources : Agroscope — Protection des végétaux, INRAE, GRAB.

use serde::{Deserialize, Serialize};

use crate::{CategorieProbleme, Gravite};

/// Pathologie documentée
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pathologie {
    pub nom: String,
    pub categorie: CategorieProbleme,
    pub cultures_cibles: Vec<String>,
    pub symptomes: Vec<String>,
    pub conditions_favorables: String,
    pub traitement_bio: String,
    pub traitement_conventionnel: String,
    pub prevention: String,
    pub source: String,
}

/// Base de connaissances phytosanitaires
pub struct BasePathologies {
    pathologies: Vec<Pathologie>,
}

impl BasePathologies {
    pub fn nouvelle() -> Self {
        let mut p = Vec::new();

        // === FONGIQUES ===
        p.push(Pathologie {
            nom: "Mildiou".into(),
            categorie: CategorieProbleme::Fongique,
            cultures_cibles: vec!["Tomate".into(), "Pomme de terre".into(), "Vigne".into()],
            symptomes: vec!["Taches brunes huileuses sur feuilles".into(), "Duvet blanc sous les feuilles".into(), "Fruits pourris".into()],
            conditions_favorables: "Humidité >80%, température 15-25°C, pluies fréquentes".into(),
            traitement_bio: "Bouillie bordelaise (cuivre), décoction de prêle, purin d'ortie préventif".into(),
            traitement_conventionnel: "Fongicides systémiques (métalaxyl)".into(),
            prevention: "Aérer les plants, éviter l'arrosage sur feuillage, rotation 4 ans, variétés résistantes".into(),
            source: "Agroscope — Mildiou de la tomate (2020)".into(),
        });

        p.push(Pathologie {
            nom: "Oïdium".into(),
            categorie: CategorieProbleme::Fongique,
            cultures_cibles: vec!["Courgette".into(), "Concombre".into(), "Vigne".into(), "Pois".into()],
            symptomes: vec!["Poudre blanche sur feuilles".into(), "Feuilles jaunissent et sèchent".into()],
            conditions_favorables: "Temps sec et chaud le jour, rosée nocturne".into(),
            traitement_bio: "Bicarbonate de soude (5g/L), soufre mouillable, lait dilué (10%)".into(),
            traitement_conventionnel: "Fongicides triazoles".into(),
            prevention: "Espacer les plants, variétés résistantes, éviter l'excès d'azote".into(),
            source: "GRAB — Oïdium des cucurbitacées (2019)".into(),
        });

        p.push(Pathologie {
            nom: "Rouille".into(),
            categorie: CategorieProbleme::Fongique,
            cultures_cibles: vec!["Haricot".into(), "Poireau".into(), "Blé".into()],
            symptomes: vec!["Pustules orange-brun sous les feuilles".into(), "Affaiblissement progressif".into()],
            conditions_favorables: "Humidité, température 15-20°C".into(),
            traitement_bio: "Décoction de prêle, purin d'ail".into(),
            traitement_conventionnel: "Fongicides strobilurines".into(),
            prevention: "Rotation, détruire les résidus, variétés résistantes".into(),
            source: "Arvalis — Rouille des céréales (2018)".into(),
        });

        p.push(Pathologie {
            nom: "Botrytis (pourriture grise)".into(),
            categorie: CategorieProbleme::Fongique,
            cultures_cibles: vec!["Fraise".into(), "Tomate".into(), "Laitue".into(), "Vigne".into()],
            symptomes: vec!["Moisissure grise sur fruits/feuilles".into(), "Pourriture molle".into()],
            conditions_favorables: "Humidité >90%, température 15-20°C, blessures".into(),
            traitement_bio: "Bacillus subtilis, aération maximale, récolte rapide".into(),
            traitement_conventionnel: "Fongicides anti-botrytis".into(),
            prevention: "Aérer, ne pas blesser les fruits, paillage propre".into(),
            source: "INRAE — Botrytis cinerea (2017)".into(),
        });

        p.push(Pathologie {
            nom: "Fonte des semis".into(),
            categorie: CategorieProbleme::Fongique,
            cultures_cibles: vec!["Tomate".into(), "Laitue".into(), "Chou".into(), "Radis".into()],
            symptomes: vec!["Plantules qui s'effondrent à la base".into(), "Pourriture du collet".into()],
            conditions_favorables: "Sol froid et humide, semis trop dense".into(),
            traitement_bio: "Trichoderma, charbon de bois en poudre, semis en sol chaud".into(),
            traitement_conventionnel: "Traitement de semences (thirame)".into(),
            prevention: "Semis en sol drainé et réchauffé, ne pas semer trop dense".into(),
            source: "Agroscope — Maladies des semis (2016)".into(),
        });

        // === INSECTES ===
        p.push(Pathologie {
            nom: "Puceron".into(),
            categorie: CategorieProbleme::Insecte,
            cultures_cibles: vec!["Tomate".into(), "Laitue".into(), "Chou".into(), "Haricot".into(), "Courgette".into()],
            symptomes: vec!["Colonies sur jeunes pousses".into(), "Feuilles enroulées et collantes".into(), "Fumagine (noir)".into()],
            conditions_favorables: "Temps chaud et sec, excès d'azote".into(),
            traitement_bio: "Savon noir (30g/L), coccinelles, purin d'ortie, chrysopes".into(),
            traitement_conventionnel: "Insecticides néonicotinoïdes (interdits EU depuis 2018)".into(),
            prevention: "Favoriser les auxiliaires, haies, bandes fleuries, éviter l'excès d'azote".into(),
            source: "GRAB — Lutte biologique contre les pucerons (2020)".into(),
        });

        p.push(Pathologie {
            nom: "Doryphore".into(),
            categorie: CategorieProbleme::Insecte,
            cultures_cibles: vec!["Pomme de terre".into(), "Aubergine".into()],
            symptomes: vec!["Défoliation massive".into(), "Larves orange sur feuilles".into(), "Adultes rayés jaune/noir".into()],
            conditions_favorables: "Monoculture de solanacées, climat chaud".into(),
            traitement_bio: "Bacillus thuringiensis var. tenebrionis, ramassage manuel, spinosad".into(),
            traitement_conventionnel: "Insecticides pyréthrinoïdes".into(),
            prevention: "Rotation stricte (4 ans sans solanacées), paillage épais, associations".into(),
            source: "Agroscope — Doryphore (2019)".into(),
        });

        p.push(Pathologie {
            nom: "Mouche de la carotte".into(),
            categorie: CategorieProbleme::Insecte,
            cultures_cibles: vec!["Carotte".into(), "Panais".into(), "Céleri".into()],
            symptomes: vec!["Galeries dans la racine".into(), "Feuillage qui jaunit".into()],
            conditions_favorables: "Sol humide, proximité de haies".into(),
            traitement_bio: "Filet anti-insectes, association avec oignon/poireau, semis tardif".into(),
            traitement_conventionnel: "Insecticides du sol".into(),
            prevention: "Filet insect-proof obligatoire, rotation, association alliacées".into(),
            source: "INRAE — Psila rosae (2018)".into(),
        });

        p.push(Pathologie {
            nom: "Piéride du chou".into(),
            categorie: CategorieProbleme::Insecte,
            cultures_cibles: vec!["Chou".into(), "Brocoli".into(), "Navet".into()],
            symptomes: vec!["Chenilles vertes sur feuilles".into(), "Feuilles dévorées, nervures restantes".into()],
            conditions_favorables: "Printemps-été, brassicacées en plein champ".into(),
            traitement_bio: "Bacillus thuringiensis, filet anti-insectes, ramassage des œufs".into(),
            traitement_conventionnel: "Insecticides larvicides".into(),
            prevention: "Filet insect-proof, association avec tomate, inspection régulière".into(),
            source: "Agroscope — Ravageurs des choux (2017)".into(),
        });

        p.push(Pathologie {
            nom: "Limace".into(),
            categorie: CategorieProbleme::Insecte, // simplifié
            cultures_cibles: vec!["Laitue".into(), "Chou".into(), "Fraise".into(), "Épinard".into()],
            symptomes: vec!["Trous irréguliers dans les feuilles".into(), "Traces argentées".into()],
            conditions_favorables: "Temps humide, nuits douces, sol riche".into(),
            traitement_bio: "Phosphate ferrique, pièges à bière, cendre, coquilles d'œuf".into(),
            traitement_conventionnel: "Métaldéhyde (toxique pour la faune)".into(),
            prevention: "Pailler sec, favoriser les hérissons et carabes, arroser le matin".into(),
            source: "GRAB — Gestion des limaces en bio (2020)".into(),
        });

        // === CARENCES ===
        p.push(Pathologie {
            nom: "Carence en azote (N)".into(),
            categorie: CategorieProbleme::Carence,
            cultures_cibles: vec!["Tomate".into(), "Chou".into(), "Laitue".into(), "Blé".into()],
            symptomes: vec!["Jaunissement des vieilles feuilles".into(), "Croissance ralentie".into(), "Plants chétifs".into()],
            conditions_favorables: "Sol pauvre en MO, lessivage, pH extrême".into(),
            traitement_bio: "Purin d'ortie, compost mûr, engrais verts (trèfle, luzerne)".into(),
            traitement_conventionnel: "Ammonitrate, urée".into(),
            prevention: "Rotation avec légumineuses, compostage, engrais verts".into(),
            source: "INRAE — Nutrition azotée des cultures (2019)".into(),
        });

        p.push(Pathologie {
            nom: "Carence en fer (chlorose ferrique)".into(),
            categorie: CategorieProbleme::Carence,
            cultures_cibles: vec!["Tomate".into(), "Fraise".into(), "Vigne".into()],
            symptomes: vec!["Jaunissement entre les nervures (restent vertes)".into(), "Jeunes feuilles d'abord".into()],
            conditions_favorables: "Sol calcaire (pH > 7.5), excès de calcaire actif".into(),
            traitement_bio: "Chélate de fer EDDHA, acidification locale, soufre élémentaire".into(),
            traitement_conventionnel: "Chélates de fer synthétiques".into(),
            prevention: "Choix de variétés tolérantes au calcaire, porte-greffes adaptés".into(),
            source: "Agroscope — Chlorose ferrique (2018)".into(),
        });

        p.push(Pathologie {
            nom: "Nécrose apicale (cul noir)".into(),
            categorie: CategorieProbleme::Physiologique,
            cultures_cibles: vec!["Tomate".into(), "Poivron".into()],
            symptomes: vec!["Tache noire à la base du fruit".into(), "Fruit non consommable".into()],
            conditions_favorables: "Irrigation irrégulière, carence en calcium, stress hydrique".into(),
            traitement_bio: "Arrosage régulier, paillage, apport de lithothamne ou coquilles broyées".into(),
            traitement_conventionnel: "Chlorure de calcium foliaire".into(),
            prevention: "Irrigation régulière (goutte-à-goutte), paillage, sol bien drainé".into(),
            source: "GRAB — Blossom end rot (2019)".into(),
        });

        p.push(Pathologie {
            nom: "Alternariose".into(),
            categorie: CategorieProbleme::Fongique,
            cultures_cibles: vec!["Tomate".into(), "Pomme de terre".into(), "Carotte".into()],
            symptomes: vec!["Taches concentriques brunes sur feuilles".into(), "Aspect de cible".into()],
            conditions_favorables: "Alternance chaud/humide, rosée, plants affaiblis".into(),
            traitement_bio: "Bouillie bordelaise, décoction de prêle, renforcement des plants".into(),
            traitement_conventionnel: "Fongicides de contact".into(),
            prevention: "Rotation, détruire les résidus, variétés résistantes, fertilisation équilibrée".into(),
            source: "Arvalis — Alternaria (2019)".into(),
        });

        p.push(Pathologie {
            nom: "Hernie du chou".into(),
            categorie: CategorieProbleme::Fongique,
            cultures_cibles: vec!["Chou".into(), "Navet".into(), "Radis".into()],
            symptomes: vec!["Racines déformées en massue".into(), "Flétrissement en journée".into()],
            conditions_favorables: "Sol acide (pH < 6.5), humide, monoculture de brassicacées".into(),
            traitement_bio: "Chaulage (pH > 7.0), rotation 7 ans minimum, drainage".into(),
            traitement_conventionnel: "Aucun traitement curatif efficace".into(),
            prevention: "Rotation longue (7 ans), chaulage préventif, variétés résistantes, drainage".into(),
            source: "Agroscope — Plasmodiophora brassicae (2017)".into(),
        });

        Self { pathologies: p }
    }

    /// Diagnostiquer selon les symptômes observés
    pub fn diagnostiquer(&self, culture: &str, symptomes_obs: &[&str]) -> Vec<DiagnosticResultat> {
        let mut resultats = Vec::new();

        for patho in &self.pathologies {
            // Vérifier si la culture est concernée
            let culture_match = patho.cultures_cibles.iter().any(|c| {
                c.to_lowercase().contains(&culture.to_lowercase())
            });

            if !culture_match {
                continue;
            }

            // Compter les symptômes correspondants
            let nb_match: usize = symptomes_obs.iter().filter(|obs| {
                patho.symptomes.iter().any(|s| {
                    let s_lower = s.to_lowercase();
                    let obs_lower = obs.to_lowercase();
                    s_lower.contains(&obs_lower) || obs_lower.contains(&s_lower)
                        || mots_communs(&s_lower, &obs_lower) >= 2
                })
            }).count();

            if nb_match > 0 {
                let confiance = (nb_match as f64 / patho.symptomes.len() as f64).min(1.0);
                resultats.push(DiagnosticResultat {
                    pathologie: patho.clone(),
                    confiance,
                    symptomes_matches: nb_match,
                    gravite: if confiance > 0.7 { Gravite::Forte } else if confiance > 0.4 { Gravite::Moderee } else { Gravite::Faible },
                });
            }
        }

        resultats.sort_by(|a, b| b.confiance.partial_cmp(&a.confiance).unwrap_or(std::cmp::Ordering::Equal));
        resultats
    }

    /// Pathologies possibles pour une culture
    pub fn pour_culture(&self, culture: &str) -> Vec<&Pathologie> {
        self.pathologies.iter().filter(|p| {
            p.cultures_cibles.iter().any(|c| c.to_lowercase() == culture.to_lowercase())
        }).collect()
    }

    pub fn count(&self) -> usize {
        self.pathologies.len()
    }
}

/// Résultat de diagnostic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticResultat {
    pub pathologie: Pathologie,
    pub confiance: f64,
    pub symptomes_matches: usize,
    pub gravite: Gravite,
}

/// Compte les mots en commun entre deux chaînes
fn mots_communs(a: &str, b: &str) -> usize {
    let mots_a: Vec<&str> = a.split_whitespace().filter(|w| w.len() > 3).collect();
    let mots_b: Vec<&str> = b.split_whitespace().filter(|w| w.len() > 3).collect();
    mots_a.iter().filter(|w| mots_b.contains(w)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_non_vide() {
        let base = BasePathologies::nouvelle();
        assert!(base.count() >= 15);
    }

    #[test]
    fn test_diagnostic_mildiou_tomate() {
        let base = BasePathologies::nouvelle();
        let resultats = base.diagnostiquer("Tomate", &["taches brunes", "duvet blanc"]);
        assert!(!resultats.is_empty());
        assert_eq!(resultats[0].pathologie.nom, "Mildiou");
    }

    #[test]
    fn test_diagnostic_puceron() {
        let base = BasePathologies::nouvelle();
        let resultats = base.diagnostiquer("Laitue", &["colonies sur jeunes pousses", "feuilles collantes"]);
        assert!(!resultats.is_empty());
    }

    #[test]
    fn test_cultures_cibles() {
        let base = BasePathologies::nouvelle();
        let pathos_tomate = base.pour_culture("Tomate");
        assert!(pathos_tomate.len() >= 4); // mildiou, oïdium, pucerons, carence N, nécrose...
    }

    #[test]
    fn test_mots_communs() {
        assert_eq!(mots_communs("taches brunes sur feuilles", "feuilles avec taches"), 2);
        assert_eq!(mots_communs("bonjour", "au revoir"), 0);
    }

    #[test]
    fn test_aucun_match() {
        let base = BasePathologies::nouvelle();
        let resultats = base.diagnostiquer("Ananas", &["symptôme inconnu"]);
        assert!(resultats.is_empty());
    }
}
