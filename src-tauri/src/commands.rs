//! Commandes IPC Tauri — pont entre les 8 modules Rust et le frontend
//!
//! Chaque commande est invocable depuis le JavaScript via `invoke("nom_commande", { ... })`.
//! Toutes les données restent locales (pilier n°1).

use serde::{Deserialize, Serialize};
use tauri::command;

use agrorim_core::{GeoPoint, MgParKg, ParcelleId, Ph, Pourcentage};
use agrorim_soil::{AnalyseSol, SolService};
use agrorim_crops::{CulturesService, ZoneClimatique};
use agrorim_weather::MeteoService;
use agrorim_health::SanteService;
use agrorim_water::{HydriqueService, TypeSolHydrique};
use agrorim_memory::MemoireService;
use agrorim_accounting::{ComptaService, CategorieCout, Devise, CanalVente};
use agrorim_biodiversity::{BiodiversiteService, CategorieBio, RoleEcologique};

// ═══════════════════════════════════════════════
// M1 — SOL
// ═══════════════════════════════════════════════

#[derive(Deserialize)]
pub struct SolInput {
    pub ph: Option<f64>,
    pub matiere_organique: Option<f64>,
    pub phosphore: Option<f64>,
    pub potassium: Option<f64>,
    pub cec: Option<f64>,
    pub argile: Option<f64>,
    pub limon: Option<f64>,
    pub sable: Option<f64>,
}

#[derive(Serialize)]
pub struct SolOutput {
    pub score: f64,
    pub texture: Option<String>,
    pub resume: String,
    pub nb_recommandations: usize,
}

#[command]
pub fn analyser_sol(input: SolInput) -> Result<SolOutput, String> {
    let mut service = SolService::new();
    let mut sol = AnalyseSol::nouvelle(ParcelleId::new());

    if let Some(v) = input.ph { sol.ph = Ph::new(v); }
    if let Some(v) = input.matiere_organique { sol.matiere_organique = Pourcentage::new(v); }
    if let Some(v) = input.phosphore { sol.phosphore_olsen = MgParKg::new(v); }
    if let Some(v) = input.potassium { sol.potassium_echangeable = MgParKg::new(v); }
    sol.cec = input.cec;
    if let Some(v) = input.argile { sol.argile = Pourcentage::new(v); }
    if let Some(v) = input.limon { sol.limon = Pourcentage::new(v); }
    if let Some(v) = input.sable { sol.sable = Pourcentage::new(v); }

    let rapport = service.analyse_complete(sol);

    Ok(SolOutput {
        score: rapport.score_global,
        texture: rapport.texture.map(|t| t.description().to_string()),
        resume: rapport.resume_agriculteur(),
        nb_recommandations: rapport.recommandations.len(),
    })
}

// ═══════════════════════════════════════════════
// M2 — CULTURES
// ═══════════════════════════════════════════════

#[derive(Deserialize)]
pub struct CulturesInput {
    pub zone: String,
    pub ph: Option<f64>,
    pub matiere_organique: Option<f64>,
    pub mois: Option<u32>,
}

#[command]
pub fn recommander_cultures(input: CulturesInput) -> Result<String, String> {
    let zone = match input.zone.as_str() {
        "oceanique" => ZoneClimatique::Oceanique,
        "mediterraneen" => ZoneClimatique::Mediterraneen,
        "montagnard" | "alpin" => ZoneClimatique::Alpine,
        _ => ZoneClimatique::Continental,
    };
    let service = CulturesService::new(zone);
    let ph = input.ph.and_then(|v| Ph::new(v));
    let rapport = service.recommander_pour_sol(
        ph.as_ref(),
        input.matiere_organique,
        input.mois,
    );
    Ok(rapport.resume())
}

// ═══════════════════════════════════════════════
// M3 — MÉTÉO
// ═══════════════════════════════════════════════

#[derive(Deserialize)]
pub struct MeteoInput {
    pub latitude: f64,
    pub longitude: f64,
    pub observations: Vec<MeteoObs>,
    pub mois: u32,
}

#[derive(Deserialize)]
pub struct MeteoObs {
    pub temperature: f64,
    pub humidite: f64,
    pub precipitation: f64,
    pub vent: f64,
}

#[command]
pub fn resume_meteo(input: MeteoInput) -> Result<String, String> {
    let mut service = MeteoService::new(GeoPoint::new(input.latitude, input.longitude));
    for obs in &input.observations {
        service.enregistrer(obs.temperature, obs.humidite, obs.precipitation, obs.vent);
    }
    Ok(service.resume(input.mois))
}

// ═══════════════════════════════════════════════
// M4 — SANTÉ VÉGÉTALE
// ═══════════════════════════════════════════════

#[derive(Deserialize)]
pub struct SanteInput {
    pub culture: String,
    pub symptomes: Vec<String>,
}

#[command]
pub fn diagnostiquer_sante(input: SanteInput) -> Result<String, String> {
    let mut service = SanteService::new();
    let symptomes_refs: Vec<&str> = input.symptomes.iter().map(|s| s.as_str()).collect();
    let rapport = service.diagnostiquer(&input.culture, &symptomes_refs);
    Ok(rapport.resume())
}

// ═══════════════════════════════════════════════
// M5 — EAU
// ═══════════════════════════════════════════════

#[derive(Deserialize)]
pub struct EauInput {
    pub type_sol: String,
    pub surface_ha: f64,
    pub culture: String,
    pub jours: Vec<EauJour>,
}

#[derive(Deserialize)]
pub struct EauJour {
    pub date: String,
    pub pluie_mm: f64,
    pub irrigation_mm: f64,
    pub temperature: f64,
}

#[command]
pub fn bilan_hydrique(input: EauInput) -> Result<String, String> {
    let type_sol = match input.type_sol.as_str() {
        "argileux" => TypeSolHydrique::Argileux,
        "sableux" => TypeSolHydrique::Sableux,
        _ => TypeSolHydrique::Limoneux,
    };
    let mut service = HydriqueService::new(type_sol, input.surface_ha, &input.culture);
    for j in &input.jours {
        service.enregistrer_jour(&j.date, j.pluie_mm, j.irrigation_mm, j.temperature);
    }
    Ok(service.resume())
}

// ═══════════════════════════════════════════════
// M6 — MÉMOIRE
// ═══════════════════════════════════════════════

#[derive(Deserialize)]
pub struct MemoireInput {
    pub recherche: Option<String>,
}

#[command]
pub fn consulter_memoire(input: MemoireInput) -> Result<String, String> {
    let service = MemoireService::new();
    if let Some(terme) = &input.recherche {
        let savoirs = service.chercher_savoir(terme);
        let mut result = format!("{} savoir(s) trouvé(s) pour '{}' :\n\n", savoirs.len(), terme);
        for s in &savoirs {
            result.push_str(&format!("  - {}\n", s));
        }
        Ok(result)
    } else {
        Ok(service.resume())
    }
}

// ═══════════════════════════════════════════════
// M7 — COMPTABILITÉ
// ═══════════════════════════════════════════════

#[derive(Deserialize)]
pub struct ComptaInput {
    pub devise: String,
    pub nom_parcelle: String,
    pub culture: String,
    pub surface_ha: f64,
    pub saison: String,
    pub couts: Vec<CoutItem>,
    pub revenus: Vec<RevenuItem>,
}

#[derive(Deserialize)]
pub struct CoutItem {
    pub categorie: String,
    pub description: String,
    pub montant: f64,
}

#[derive(Deserialize)]
pub struct RevenuItem {
    pub quantite_kg: f64,
    pub prix_kg: f64,
    pub canal: String,
}

#[command]
pub fn resume_comptabilite(input: ComptaInput) -> Result<String, String> {
    let devise = match input.devise.as_str() {
        "EUR" => Devise::EUR,
        _ => Devise::CHF,
    };
    let mut service = ComptaService::new(devise);
    let idx = service.creer_parcelle(&input.nom_parcelle, &input.culture, input.surface_ha, &input.saison);
    if let Some(p) = service.parcelle(idx) {
        for c in &input.couts {
            let cat = match c.categorie.as_str() {
                "semences" => CategorieCout::Semences,
                "engrais" => CategorieCout::Engrais,
                "irrigation" => CategorieCout::Irrigation,
                "main_oeuvre" => CategorieCout::MainOeuvre,
                "materiel" => CategorieCout::Materiel,
                _ => CategorieCout::Autre,
            };
            p.ajouter_cout(cat, &c.description, c.montant, devise);
        }
        for r in &input.revenus {
            let canal = match r.canal.as_str() {
                "marche" => CanalVente::Marche,
                "cooperative" => CanalVente::Cooperative,
                "panier" => CanalVente::Panier,
                _ => CanalVente::VenteDirecte,
            };
            p.ajouter_revenu(r.quantite_kg, r.prix_kg, devise, canal);
        }
    }
    Ok(service.resume())
}

// ═══════════════════════════════════════════════
// M8 — BIODIVERSITÉ
// ═══════════════════════════════════════════════

#[derive(Deserialize)]
pub struct BiodiversiteInput {
    pub parcelle: String,
    pub observations: Vec<BioObs>,
}

#[derive(Deserialize)]
pub struct BioObs {
    pub espece: String,
    pub categorie: String,
    pub nombre: usize,
    pub role: String,
}

#[command]
pub fn resume_biodiversite(input: BiodiversiteInput) -> Result<String, String> {
    let mut service = BiodiversiteService::new(&input.parcelle);
    for obs in &input.observations {
        let cat = match obs.categorie.as_str() {
            "predateur" => CategorieBio::PredateurNaturel,
            "pollinisateur" => CategorieBio::Pollinisateur,
            "oiseau" => CategorieBio::Oiseau,
            "ver_de_terre" => CategorieBio::VerDeTerre,
            "plante" => CategorieBio::PlanteIndicatrice,
            _ => CategorieBio::Insecte,
        };
        let role = match obs.role.as_str() {
            "predation" => RoleEcologique::PredationRavageurs,
            "pollinisation" => RoleEcologique::Pollinisation,
            "aeration" => RoleEcologique::AerationSol,
            "indicateur" => RoleEcologique::IndicateurSante,
            _ => RoleEcologique::IndicateurSante,
        };
        service.observer(&obs.espece, cat, obs.nombre as u32, role);
    }
    Ok(service.resume())
}

// ═══════════════════════════════════════════════
// SYSTÈME
// ═══════════════════════════════════════════════

#[derive(Serialize)]
pub struct VersionInfo {
    pub version: String,
    pub modules: Vec<String>,
    pub piliers: Vec<String>,
}

#[command]
pub fn version_info() -> VersionInfo {
    VersionInfo {
        version: "0.2.0".into(),
        modules: vec![
            "M1 Sol".into(),
            "M2 Cultures".into(),
            "M3 Météo".into(),
            "M4 Santé végétale".into(),
            "M5 Eau".into(),
            "M6 Mémoire ancestrale".into(),
            "M7 Comptabilité".into(),
            "M8 Biodiversité".into(),
        ],
        piliers: vec![
            "Local-first absolu".into(),
            "Zéro extraction".into(),
            "Authentification terrain".into(),
            "Explicabilité totale".into(),
            "Déconnectabilité garantie".into(),
            "Open source protégé".into(),
            "Interopérabilité souveraine".into(),
        ],
    }
}
