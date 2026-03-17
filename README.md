# TERRASENS™ — IA Agronomique Souveraine

**Local-first · Zéro cloud · Zéro extraction · 100% explicable**

TERRASENS™ est une intelligence artificielle agronomique souveraine, conçue pour protéger les agriculteurs de la technologie, pas pour les asservir.

> *TERRASENS™ est une marque d'Olivier Chevalley. Le logiciel est développé par l'Association ORIM Genève sous licence AGPL-3.0-or-later.*

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
- **Base de données** : SQLite + WAL mode (local, zéro cloud)
- **ML** : ONNX Runtime (inférence locale, prévu)
- **Vision** : YOLOv8 nano (détection maladies, prévu)
- **UI** : Tauri (desktop/mobile natif, prévu)
- **Capteurs** : MQTT / LoRa / I2C
- **Cible** : Raspberry Pi 4+, laptop, smartphone

## Quickstart

```bash
# Prérequis : Rust 1.75+
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Cloner et compiler
git clone https://github.com/Orimnutrition/terrasens.git
cd terrasens
cargo build

# Lancer la démo complète (8 modules)
cargo run

# Lancer les tests (134 tests)
cargo test --workspace
```

## Architecture

```
terrasens/
├── src/main.rs                    # Démo M1-M8
├── CLA.md                         # Contributor License Agreement
├── CONTRIBUTORS.md                # Contributeurs
├── entanglement.toml              # Matrice d'intrication
├── crates/
│   ├── terrasens-core/            # Types, unités, géo, erreurs, traits
│   ├── terrasens-soil/            # M1 — Analyse de sol
│   ├── terrasens-crops/           # M2 — Cultures et rotation
│   ├── terrasens-weather/         # M3 — Météo hyper-locale
│   ├── terrasens-health/          # M4 — Santé végétale
│   ├── terrasens-water/           # M5 — Gestion hydrique
│   ├── terrasens-memory/          # M6 — Mémoire ancestrale
│   ├── terrasens-accounting/      # M7 — Comptabilité agronomique
│   ├── terrasens-biodiversity/    # M8 — Biodiversité
│   ├── terrasens-auth/            # Authentification terrain 4 couches
│   ├── terrasens-db/              # SQLite local (WAL mode)
│   ├── terrasens-sensors/         # Capteurs IoT (MQTT/LoRa)
│   └── terrasens-rules/           # Moteur de règles explicables
```

## Contribuer

TERRASENS™ est un projet de l'Association ORIM Genève. Contributions bienvenues !

**Important : tout contributeur doit accepter le [CLA](CLA.md) avant sa première contribution.**

1. Fork le repo
2. Accepte le CLA (commentaire "I accept the CLA" sur ta PR)
3. Crée une branche (`git checkout -b feature/mon-module`)
4. Commite tes changements
5. Pousse et crée une Pull Request

**Priorités** : Interface Tauri, modèles ML edge, données régionales (autres zones climatiques).

## Propriété Intellectuelle

- **TERRASENS™** est une marque d'Olivier Chevalley
- **Code source** : © 2026 Association ORIM Genève — AGPL-3.0-or-later
- **Brevets** : Innovations protégées par defensive publication (mars 2026)
- **CLA** : Obligatoire pour tout contributeur

## Licence

**AGPL-3.0-or-later** — Le code reste libre. Toute modification déployée côté serveur doit être partagée.

Voir [LICENSE](LICENSE) pour le texte complet.

## Contact

Association ORIM · Genève, Suisse · info@orimnutrition.com

---

*TERRASENS™ — Parce que la terre a un sens.*
