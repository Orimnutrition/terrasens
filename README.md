# AGRORIM™ — IA Agronomique Souveraine

**Local-first · Zéro cloud · Zéro extraction · 100% explicable**

AGRORIM™ est une intelligence artificielle agronomique souveraine, conçue pour protéger les agriculteurs de la technologie, pas pour les asservir.

> *AGRORIM™ est une marque d'Olivier Chevalley. Le logiciel est développé par l'Association ORIM Genève sous licence AGPL-3.0-or-later.*

## 7 Piliers Inviolables

1. **Local-first absolu** — Toutes les données restent sur l'appareil de l'agriculteur
2. **Zéro extraction** — Aucune donnée ne quitte jamais la ferme
3. **Authentification terrain** — L'agriculteur prouve qu'il connaît SA terre
4. **Explicabilité totale** — Chaque recommandation est expliquée en langage simple avec sa source scientifique
5. **Déconnectabilité garantie** — Fonctionne 100% hors ligne
6. **Open source protégé** — AGPL-3.0 : personne ne peut fermer le code
7. **Interopérabilité souveraine** — Export en formats ouverts (JSON, CSV)

## 8 Modules Opérationnels

| Module | Description | Tests | Statut |
|--------|-------------|-------|--------|
| **M1 — Sol** | Analyse 12 paramètres, score 0-100, 7 règles agronomiques | 10 | Fonctionnel |
| **M2 — Cultures** | 13 cultures, rotation, associations, calendrier, intrication ENT-001 | 24 | Fonctionnel |
| **M3 — Météo** | Alertes gel/canicule/sécheresse, prévisions locales, tendances | 17 | Fonctionnel |
| **M4 — Santé végétale** | 15 pathologies, diagnostic par symptômes, traitements bio | 11 | Fonctionnel |
| **M5 — Eau** | Bilan hydrique, ETP Hargreaves, Kc FAO, stress hydrique | 11 | Fonctionnel |
| **M6 — Mémoire ancestrale** | 10 savoirs validés scientifiquement, journal de bord | 15 | Fonctionnel |
| **M7 — Comptabilité** | Coûts/revenus parcelle, marge brute CHF/EUR | 11 | Fonctionnel |
| **M8 — Biodiversité** | Indice Shannon-Wiener, équitabilité Piélou, auxiliaires | 13 | Fonctionnel |

**Total : 134 tests · 0 failures · ~4500 lignes Rust**

## Matrice d'intrication

| Paire | Module A | Module B | Description |
|-------|----------|----------|-------------|
| ENT-001 | Sol | Cultures | pH et MO filtrent les recommandations de cultures |
| ENT-002 | Météo | Eau | Observations météo alimentent le bilan hydrique |
| ENT-003 | Santé | Biodiversité | Diagnostics impactent l'indice de biodiversité |
| ENT-004 | Mémoire | Comptabilité | Journal enrichit le suivi des coûts |

## Stack Technique

- **Langage** : Rust 1.94 (sécurité mémoire, performance, embarqué)
- **UI Desktop** : Tauri v2 (vanilla HTML/CSS/JS, 9 commandes IPC, 3.2 MB)
- **Base de données** : SQLite + WAL mode (local, zéro cloud)
- **ML** : ONNX Runtime (inférence locale, YOLOv8 nano)
- **Capteurs** : MQTT / LoRa / I2C
- **Cible** : macOS (DMG), Linux, Windows, Raspberry Pi 4+

## Quickstart

```bash
# Prérequis : Rust 1.75+
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Cloner et compiler
git clone https://github.com/Orimnutrition/agrorim.git
cd agrorim
cargo build

# Lancer la démo CLI (8 modules)
cargo run

# Lancer les tests (134 tests)
cargo test --workspace

# Lancer l'interface desktop Tauri
cargo tauri dev

# Build macOS release (.app + .dmg)
cargo tauri build
```

## Architecture

```
agrorim/
├── src/main.rs                    # Démo CLI M1-M8
├── src-tauri/                     # Backend desktop Tauri v2
│   ├── src/main.rs                # Point d'entrée + 9 commandes IPC
│   └── src/commands.rs            # Pont Rust ↔ JavaScript (M1-M8)
├── src-frontend/                  # Interface web (vanilla)
│   ├── index.html                 # 8 onglets modules
│   ├── styles.css                 # Charte ORIM Teal/Gold/Dark
│   └── app.js                     # IPC invoke() layer
├── CLA.md                         # Contributor License Agreement
├── CONTRIBUTORS.md                # Contributeurs
├── entanglement.toml              # Matrice d'intrication
├── crates/
│   ├── agrorim-core/            # Types, unités, géo, erreurs, traits
│   ├── agrorim-soil/            # M1 — Analyse de sol
│   ├── agrorim-crops/           # M2 — Cultures et rotation
│   ├── agrorim-weather/         # M3 — Météo hyper-locale
│   ├── agrorim-health/          # M4 — Santé végétale
│   ├── agrorim-water/           # M5 — Gestion hydrique
│   ├── agrorim-memory/          # M6 — Mémoire ancestrale
│   ├── agrorim-accounting/      # M7 — Comptabilité agronomique
│   ├── agrorim-biodiversity/    # M8 — Biodiversité
│   ├── agrorim-auth/            # Authentification terrain 4 couches
│   ├── agrorim-db/              # SQLite local (WAL mode)
│   ├── agrorim-sensors/         # Capteurs IoT (MQTT/LoRa)
│   └── agrorim-rules/           # Moteur de règles explicables
```

## Contribuer

AGRORIM™ est un projet de l'Association ORIM Genève. Contributions bienvenues !

**Important : tout contributeur doit accepter le [CLA](CLA.md) avant sa première contribution.**

1. Fork le repo
2. Accepte le CLA (commentaire "I accept the CLA" sur ta PR)
3. Crée une branche (`git checkout -b feature/mon-module`)
4. Commite tes changements
5. Pousse et crée une Pull Request

**Priorités** : Interface Tauri, modèles ML edge, données régionales (autres zones climatiques).

## Propriété Intellectuelle

- **AGRORIM™** est une marque d'Olivier Chevalley (Agro + ORIM)
- **Code source** : © 2026 Association ORIM Genève — AGPL-3.0-or-later
- **Brevets** : 15 innovations identifiées, 25 revendications, dossier IPI en cours
- **Defensive publication** : 5 innovations publiées (antériorité mars 2026)
- **CLA** : Obligatoire pour tout contributeur

## Licence

**AGPL-3.0-or-later** — Le code reste libre. Toute modification déployée côté serveur doit être partagée.

Voir [LICENSE](LICENSE) pour le texte complet.

## Contact

Association ORIM · Genève, Suisse · info@orimnutrition.com

---

*AGRORIM™ — L'agriculture a un sens.*
