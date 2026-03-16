//! # TERRASENS — IA Agronomique Souveraine
//!
//! Point d'entrée principal.
//! Local-first · Zéro cloud · Zéro extraction · 100% explicable
//!
//! © 2026 Association ORIM Genève — AGPL-3.0-or-later

use terrasens_core::{MgParKg, ParcelleId, Ph, Pourcentage};
use terrasens_soil::{AnalyseSol, SolService};

fn main() {
    println!("╔═══════════════════════════════════════════════════╗");
    println!("║          TERRASENS v0.1.0                        ║");
    println!("║  IA Agronomique Souveraine                       ║");
    println!("║  Local-first · Zéro cloud · 100% explicable      ║");
    println!("║  © 2026 Association ORIM Genève                  ║");
    println!("╚═══════════════════════════════════════════════════╝");
    println!();

    // === DÉMONSTRATION : Analyse de sol ===
    println!("=== DÉMONSTRATION : Module M1 — Analyse de Sol ===\n");

    // Créer un service sol
    let mut service = SolService::new();

    // Simuler une analyse avec des valeurs problématiques
    println!("Analyse d'un sol acide et pauvre en matière organique...\n");

    let parcelle = ParcelleId::new();
    let mut sol = AnalyseSol::nouvelle(parcelle);
    sol.ph = Ph::new(5.0);
    sol.matiere_organique = Pourcentage::new(1.8);
    sol.phosphore_olsen = MgParKg::new(12.0);
    sol.potassium_echangeable = MgParKg::new(75.0);
    sol.cec = Some(9.0);
    sol.argile = Pourcentage::new(20.0);
    sol.limon = Pourcentage::new(30.0);
    sol.sable = Pourcentage::new(50.0);

    let rapport = service.analyse_complete(sol);
    println!("{}", rapport.resume_agriculteur());

    // Analyser un sol sain
    println!("\nAnalyse d'un sol en bonne santé...\n");

    let parcelle2 = ParcelleId::new();
    let mut sol2 = AnalyseSol::nouvelle(parcelle2);
    sol2.ph = Ph::new(6.5);
    sol2.matiere_organique = Pourcentage::new(4.2);
    sol2.phosphore_olsen = MgParKg::new(28.0);
    sol2.potassium_echangeable = MgParKg::new(220.0);
    sol2.cec = Some(18.0);

    let rapport2 = service.analyse_complete(sol2);
    println!("{}", rapport2.resume_agriculteur());

    println!("Total analyses effectuées : {}", service.count_analyses());
    println!("\nTERRASENS opérationnel — Module M1 Sol fonctionnel");
}
