# TERRASENS — IA Agronomique Souveraine

**Local-first · Zero cloud · Zero extraction · 100% explicable**

TERRASENS est une intelligence artificielle agronomique souveraine, conçue pour protéger les agriculteurs de la technologie, pas pour les asservir.

## 7 Piliers Inviolables

1. **Local-first absolu** — Toutes les données restent sur l'appareil de l'agriculteur
2. **Zero extraction** — Aucune donnée ne quitte jamais la ferme
3. **Authentification terrain** — L'agriculteur prouve qu'il connait SA terre
4. **Explicabilite totale** — Chaque recommandation est expliquee en langage simple avec sa source scientifique
5. **Deconnectabilite garantie** — Fonctionne 100% hors ligne
6. **Open source protege** — AGPL-3.0 : personne ne peut fermer le code
7. **Interoperabilite souveraine** — Export en formats ouverts (JSON, CSV)

## Modules

| Module | Description | Statut |
|--------|-------------|--------|
| **M1 — Sol** | Analyse de sol, recommandations explicables, 7 regles agronomiques | Fonctionnel |
| **M2 — Cultures** | Planification, rotation, associations | En cours |
| **M3 — Meteo** | Previsions hyper-locales par capteurs | En cours |
| **M4 — Sante vegetale** | Detection maladies par vision (YOLOv8 nano) | En cours |
| **M5 — Eau** | Bilan hydrique parcellaire | En cours |
| **M6 — Memoire ancestrale** | Savoirs traditionnels, journal de bord | En cours |
| **M7 — Comptabilite** | Couts reels par parcelle, marge brute | En cours |
| **M8 — Biodiversite** | Inventaire especes, corridors ecologiques | En cours |

## Stack Technique

- **Langage** : Rust (securite memoire, performance, embarque)
- **Base de donnees** : SQLite + SpatiaLite (local, zero cloud)
- **ML** : ONNX Runtime (inference locale)
- **Vision** : YOLOv8 nano (detection plantes/maladies)
- **Voix** : Whisper.cpp (commande vocale offline)
- **UI** : Tauri (desktop/mobile natif)
- **Capteurs** : MQTT / LoRa
- **Cible** : Raspberry Pi 4+, laptop, smartphone

## Quickstart

```bash
# Prerequis : Rust 1.75+
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Cloner et compiler
git clone https://github.com/Orimnutrition/terrasens.git
cd terrasens
cargo build

# Lancer la demo (analyse de sol)
cargo run

# Lancer les tests (39 tests)
cargo test --workspace
```

## Demo : Analyse de Sol

```
=== RAPPORT DE SOL ===
Score : 64/100 — Bon

5 recommandation(s) :

1. pH — Haute
   pH 5.0 (Acide) -> 6.0 — 7.0
   Action : Chaulage : 2-4 t/ha de CaCO3
   Pourquoi : Votre sol est trop acide. Les plantes peinent a absorber
   les nutriments. Un chaulage corrigera cela en 6 a 12 mois.
   Source : Agroscope — Guide de fertilisation (2017)
```

## Architecture

```
terrasens/
├── src/main.rs                    # Point d'entree
├── crates/
│   ├── terrasens-core/            # Types, unites, geo, erreurs, traits
│   ├── terrasens-soil/            # M1 — Analyse de sol
│   ├── terrasens-crops/           # M2 — Cultures
│   ├── terrasens-weather/         # M3 — Meteo
│   ├── terrasens-health/          # M4 — Sante vegetale
│   ├── terrasens-water/           # M5 — Eau
│   ├── terrasens-memory/          # M6 — Memoire ancestrale
│   ├── terrasens-accounting/      # M7 — Comptabilite
│   ├── terrasens-biodiversity/    # M8 — Biodiversite
│   ├── terrasens-auth/            # Authentification terrain 4 couches
│   ├── terrasens-db/              # SQLite local
│   ├── terrasens-sensors/         # Capteurs IoT (MQTT/LoRa)
│   └── terrasens-rules/           # Moteur de regles explicables
```

## Contribuer

TERRASENS est un projet de l'Association ORIM Geneve. Contributions bienvenues :

1. Fork le repo
2. Cree une branche (`git checkout -b feature/mon-module`)
3. Commite tes changements
4. Pousse et cree une Pull Request

**Priorites de contribution** : Modules M2-M8, interface Tauri, modeles ML edge.

## Licence

**AGPL-3.0-or-later** — Le code reste libre. Toute modification doit etre partagee.

## Contact

Association ORIM · Geneve, Suisse · info@orimnutrition.com
