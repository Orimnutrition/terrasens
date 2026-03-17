//! # TERRASENS™ — IA Agronomique Souveraine
//!
//! Point d'entrée principal — Démonstration de tous les modules M1-M8.
//! Local-first · Zéro cloud · Zéro extraction · 100% explicable
//!
//! © 2026 Association ORIM Genève — AGPL-3.0-or-later

use terrasens_core::{GeoPoint, MgParKg, ParcelleId, Ph, Pourcentage};
use terrasens_soil::{AnalyseSol, SolService};
use terrasens_crops::{CulturesService, ZoneClimatique};
use terrasens_weather::MeteoService;
use terrasens_health::SanteService;
use terrasens_water::{HydriqueService, TypeSolHydrique};
use terrasens_memory::MemoireService;
use terrasens_accounting::{ComptaService, CategorieCout, Devise, CanalVente};
use terrasens_biodiversity::{BiodiversiteService, CategorieBio, RoleEcologique};

fn main() {
    println!("╔═══════════════════════════════════════════════════╗");
    println!("║          TERRASENS v0.2.0                        ║");
    println!("║  IA Agronomique Souveraine                       ║");
    println!("║  8 modules · Local-first · Zéro cloud            ║");
    println!("║  © 2026 Association ORIM Genève                  ║");
    println!("╚═══════════════════════════════════════════════════╝");
    println!();

    // ═══════════════════════════════════════════════
    // M1 — ANALYSE DE SOL
    // ═══════════════════════════════════════════════
    println!("━━━ M1 SOL ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    let mut service_sol = SolService::new();

    let parcelle = ParcelleId::new();
    let mut sol = AnalyseSol::nouvelle(parcelle);
    sol.ph = Ph::new(6.5);
    sol.matiere_organique = Pourcentage::new(3.8);
    sol.phosphore_olsen = MgParKg::new(25.0);
    sol.potassium_echangeable = MgParKg::new(180.0);
    sol.cec = Some(16.0);
    sol.argile = Pourcentage::new(25.0);
    sol.limon = Pourcentage::new(40.0);
    sol.sable = Pourcentage::new(35.0);

    let rapport_sol = service_sol.analyse_complete(sol);
    println!("{}", rapport_sol.resume_agriculteur());

    // ═══════════════════════════════════════════════
    // M2 — CULTURES (INTRICATION ENT-001 : Sol → Cultures)
    // ═══════════════════════════════════════════════
    println!("━━━ M2 CULTURES ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    let service_cultures = CulturesService::new(ZoneClimatique::Continental);
    let ph = Ph::new(6.5).unwrap();
    let rapport_cultures = service_cultures.recommander_pour_sol(Some(&ph), Some(3.8), Some(5));
    println!("{}", rapport_cultures.resume());

    // ═══════════════════════════════════════════════
    // M3 — MÉTÉO
    // ═══════════════════════════════════════════════
    println!("━━━ M3 MÉTÉO ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    let mut service_meteo = MeteoService::new(GeoPoint::new(46.2044, 6.1432)); // Genève
    service_meteo.enregistrer(14.0, 65.0, 0.0, 12.0);
    service_meteo.enregistrer(16.0, 58.0, 0.0, 8.0);
    service_meteo.enregistrer(12.0, 72.0, 4.5, 15.0);
    service_meteo.enregistrer(18.0, 55.0, 0.0, 6.0);
    service_meteo.enregistrer(20.0, 50.0, 0.0, 10.0);
    println!("{}", service_meteo.resume(5)); // mai

    // ═══════════════════════════════════════════════
    // M4 — SANTÉ VÉGÉTALE
    // ═══════════════════════════════════════════════
    println!("━━━ M4 SANTÉ VÉGÉTALE ━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    let mut service_sante = SanteService::new();
    let rapport_sante = service_sante.diagnostiquer(
        "Tomate",
        &["taches brunes", "duvet blanc sous les feuilles"],
    );
    println!("{}", rapport_sante.resume());

    // ═══════════════════════════════════════════════
    // M5 — GESTION HYDRIQUE (INTRICATION ENT-002 : Météo → Eau)
    // ═══════════════════════════════════════════════
    println!("━━━ M5 GESTION HYDRIQUE ━━━━━━━━━━━━━━━━━━━━━━━━━━");
    let mut service_eau = HydriqueService::new(TypeSolHydrique::Limoneux, 0.5, "Tomate");
    service_eau.enregistrer_jour("2026-07-10", 0.0, 0.0, 28.0);
    service_eau.enregistrer_jour("2026-07-11", 0.0, 0.0, 30.0);
    service_eau.enregistrer_jour("2026-07-12", 0.0, 0.0, 32.0);
    service_eau.enregistrer_jour("2026-07-13", 15.0, 0.0, 25.0);
    service_eau.enregistrer_jour("2026-07-14", 0.0, 0.0, 27.0);
    println!("{}", service_eau.resume());

    // ═══════════════════════════════════════════════
    // M6 — MÉMOIRE ANCESTRALE
    // ═══════════════════════════════════════════════
    println!("━━━ M6 MÉMOIRE ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    let mut service_memoire = MemoireService::new();
    service_memoire.journal().ajouter(
        terrasens_memory::TypeEntree::Observation,
        "Sol meuble après la pluie, vers de terre nombreux",
        Some("P1"), Some("Tomate"), &["sol", "printemps"],
    );
    service_memoire.journal().ajouter(
        terrasens_memory::TypeEntree::Recolte,
        "Première récolte de tomates cerises — 3.5 kg",
        Some("P1"), Some("Tomate"), &["récolte"],
    );
    println!("{}", service_memoire.resume());

    // Recherche de savoir
    let savoirs = service_memoire.chercher_savoir("ortie");
    println!("Savoirs trouvés pour 'ortie' : {}", savoirs.len());
    for s in &savoirs {
        println!("  → {}", s);
    }
    println!();

    // ═══════════════════════════════════════════════
    // M7 — COMPTABILITÉ (INTRICATION ENT-004 : Mémoire → Compta)
    // ═══════════════════════════════════════════════
    println!("━━━ M7 COMPTABILITÉ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    let mut service_compta = ComptaService::new(Devise::CHF);
    let idx = service_compta.creer_parcelle("P1", "Tomate coeur de boeuf", 0.08, "2026");
    if let Some(p) = service_compta.parcelle(idx) {
        p.ajouter_cout(CategorieCout::Semences, "Graines bio", 35.0, Devise::CHF);
        p.ajouter_cout(CategorieCout::Engrais, "Compost maison", 0.0, Devise::CHF);
        p.ajouter_cout(CategorieCout::Irrigation, "Goutte-à-goutte", 60.0, Devise::CHF);
        p.ajouter_cout(CategorieCout::MainOeuvre, "25h × 30 CHF", 750.0, Devise::CHF);
        p.ajouter_revenu(180.0, 6.00, Devise::CHF, CanalVente::VenteDirecte);
        p.ajouter_revenu(120.0, 4.50, Devise::CHF, CanalVente::Marche);
    }
    println!("{}", service_compta.resume());

    // ═══════════════════════════════════════════════
    // M8 — BIODIVERSITÉ (INTRICATION ENT-003 : Santé → Bio)
    // ═══════════════════════════════════════════════
    println!("━━━ M8 BIODIVERSITÉ ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    let mut service_bio = BiodiversiteService::new("Parcelle P1");
    service_bio.observer("Coccinelle à 7 points", CategorieBio::PredateurNaturel, 12, RoleEcologique::PredationRavageurs);
    service_bio.observer("Abeille domestique", CategorieBio::Pollinisateur, 25, RoleEcologique::Pollinisation);
    service_bio.observer("Mésange bleue", CategorieBio::Oiseau, 4, RoleEcologique::PredationRavageurs);
    service_bio.observer("Ver de terre", CategorieBio::VerDeTerre, 40, RoleEcologique::AerationSol);
    service_bio.observer("Syrphe", CategorieBio::Pollinisateur, 8, RoleEcologique::Pollinisation);
    service_bio.observer("Pissenlit", CategorieBio::PlanteIndicatrice, 15, RoleEcologique::IndicateurSante);
    println!("{}", service_bio.resume());

    // ═══════════════════════════════════════════════
    // RÉSUMÉ FINAL
    // ═══════════════════════════════════════════════
    println!("╔═══════════════════════════════════════════════════╗");
    println!("║  TERRASENS — 8 modules opérationnels             ║");
    println!("║  135+ tests · 0 cloud · 100% local               ║");
    println!("║  Prêt pour le terrain.                           ║");
    println!("╚═══════════════════════════════════════════════════╝");
}
