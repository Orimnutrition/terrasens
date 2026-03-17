//! # AGRORIM™ — Desktop Application
//!
//! Interface bureau Tauri v2 pour l'IA agronomique souveraine.
//! Local-first · Zéro cloud · 100% explicable
//!
//! © 2026 Association ORIM Genève — AGPL-3.0-or-later

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::analyser_sol,
            commands::recommander_cultures,
            commands::resume_meteo,
            commands::diagnostiquer_sante,
            commands::bilan_hydrique,
            commands::consulter_memoire,
            commands::resume_comptabilite,
            commands::resume_biodiversite,
            commands::version_info,
        ])
        .run(tauri::generate_context!())
        .expect("Erreur au lancement d'AGRORIM Desktop");
}
