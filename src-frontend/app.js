// ═══════════════════════════════════════════════════
// AGRORIM™ — Frontend Application
// Pont JavaScript ↔ Rust via Tauri IPC
// Local-first · Zéro cloud · 100% souverain
// ═══════════════════════════════════════════════════

const { invoke } = window.__TAURI__.core;

// ─── Navigation par onglets ────────────────────────

document.querySelectorAll('.tab').forEach(tab => {
  tab.addEventListener('click', () => {
    document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
    document.querySelectorAll('.module').forEach(m => m.classList.remove('active'));
    tab.classList.add('active');
    document.getElementById('mod-' + tab.dataset.module).classList.add('active');
  });
});

// ─── Helpers ───────────────────────────────────────

function showResult(id, text) {
  const box = document.getElementById(id);
  box.className = 'result-box';
  box.textContent = text;
}

function showError(id, msg) {
  const box = document.getElementById(id);
  box.className = 'result-box error';
  box.textContent = 'Erreur : ' + msg;
}

function showLoading(id) {
  const box = document.getElementById(id);
  box.className = 'result-box loading';
  box.textContent = 'Analyse en cours...';
}

function num(id) {
  const v = document.getElementById(id).value;
  return v ? parseFloat(v) : null;
}

function str(id) {
  return document.getElementById(id).value.trim();
}

function parseLines(id) {
  return document.getElementById(id).value
    .split('\n')
    .map(l => l.trim())
    .filter(l => l.length > 0);
}

// ─── M1 SOL ────────────────────────────────────────

document.getElementById('form-sol').addEventListener('submit', async (e) => {
  e.preventDefault();
  showLoading('result-sol');
  try {
    const result = await invoke('analyser_sol', {
      input: {
        ph: num('sol-ph'),
        matiere_organique: num('sol-mo'),
        phosphore: num('sol-p'),
        potassium: num('sol-k'),
        cec: num('sol-cec'),
        argile: num('sol-argile'),
        limon: num('sol-limon'),
        sable: num('sol-sable'),
      }
    });
    showResult('result-sol', result.resume);
  } catch (err) {
    showError('result-sol', err);
  }
});

// ─── M2 CULTURES ───────────────────────────────────

document.getElementById('form-cultures').addEventListener('submit', async (e) => {
  e.preventDefault();
  showLoading('result-cultures');
  try {
    const moisVal = str('cult-mois');
    const result = await invoke('recommander_cultures', {
      input: {
        zone: str('cult-zone'),
        ph: num('cult-ph'),
        matiere_organique: num('cult-mo'),
        mois: moisVal ? parseInt(moisVal) : null,
      }
    });
    showResult('result-cultures', result);
  } catch (err) {
    showError('result-cultures', err);
  }
});

// ─── M3 MÉTÉO ──────────────────────────────────────

document.getElementById('form-meteo').addEventListener('submit', async (e) => {
  e.preventDefault();
  showLoading('result-meteo');
  try {
    const lines = parseLines('meteo-obs');
    const observations = lines.map(line => {
      const parts = line.split(',').map(s => parseFloat(s.trim()));
      return {
        temperature: parts[0] || 0,
        humidite: parts[1] || 0,
        precipitation: parts[2] || 0,
        vent: parts[3] || 0,
      };
    });
    const result = await invoke('resume_meteo', {
      input: {
        latitude: num('meteo-lat') || 46.2044,
        longitude: num('meteo-lon') || 6.1432,
        observations: observations,
        mois: parseInt(str('meteo-mois')) || 5,
      }
    });
    showResult('result-meteo', result);
  } catch (err) {
    showError('result-meteo', err);
  }
});

// ─── M4 SANTÉ ──────────────────────────────────────

document.getElementById('form-sante').addEventListener('submit', async (e) => {
  e.preventDefault();
  showLoading('result-sante');
  try {
    const symptomes = parseLines('sante-symptomes');
    const result = await invoke('diagnostiquer_sante', {
      input: {
        culture: str('sante-culture'),
        symptomes: symptomes,
      }
    });
    showResult('result-sante', result);
  } catch (err) {
    showError('result-sante', err);
  }
});

// ─── M5 EAU ────────────────────────────────────────

document.getElementById('form-eau').addEventListener('submit', async (e) => {
  e.preventDefault();
  showLoading('result-eau');
  try {
    const lines = parseLines('eau-jours');
    const jours = lines.map(line => {
      const parts = line.split(',').map(s => s.trim());
      return {
        date: parts[0] || '',
        pluie_mm: parseFloat(parts[1]) || 0,
        irrigation_mm: parseFloat(parts[2]) || 0,
        temperature: parseFloat(parts[3]) || 0,
      };
    });
    const result = await invoke('bilan_hydrique', {
      input: {
        type_sol: str('eau-sol'),
        surface_ha: num('eau-surface') || 0.5,
        culture: str('eau-culture') || 'Tomate',
        jours: jours,
      }
    });
    showResult('result-eau', result);
  } catch (err) {
    showError('result-eau', err);
  }
});

// ─── M6 MÉMOIRE ───────────────────────────────────

document.getElementById('form-memoire').addEventListener('submit', async (e) => {
  e.preventDefault();
  showLoading('result-memoire');
  try {
    const recherche = str('memoire-recherche') || null;
    const result = await invoke('consulter_memoire', {
      input: { recherche: recherche }
    });
    showResult('result-memoire', result);
  } catch (err) {
    showError('result-memoire', err);
  }
});

// ─── M7 COMPTABILITÉ ──────────────────────────────

document.getElementById('form-compta').addEventListener('submit', async (e) => {
  e.preventDefault();
  showLoading('result-compta');
  try {
    const coutsLines = parseLines('compta-couts');
    const couts = coutsLines.map(line => {
      const parts = line.split(',').map(s => s.trim());
      return {
        categorie: parts[0] || 'autre',
        description: parts[1] || '',
        montant: parseFloat(parts[2]) || 0,
      };
    });

    const revenusLines = parseLines('compta-revenus');
    const revenus = revenusLines.map(line => {
      const parts = line.split(',').map(s => s.trim());
      return {
        quantite_kg: parseFloat(parts[0]) || 0,
        prix_kg: parseFloat(parts[1]) || 0,
        canal: parts[2] || 'directe',
      };
    });

    const result = await invoke('resume_comptabilite', {
      input: {
        devise: str('compta-devise'),
        nom_parcelle: str('compta-parcelle'),
        culture: str('compta-culture'),
        surface_ha: num('compta-surface') || 0.08,
        saison: str('compta-saison'),
        couts: couts,
        revenus: revenus,
      }
    });
    showResult('result-compta', result);
  } catch (err) {
    showError('result-compta', err);
  }
});

// ─── M8 BIODIVERSITÉ ──────────────────────────────

document.getElementById('form-biodiversite').addEventListener('submit', async (e) => {
  e.preventDefault();
  showLoading('result-biodiversite');
  try {
    const lines = parseLines('bio-obs');
    const observations = lines.map(line => {
      const parts = line.split(',').map(s => s.trim());
      return {
        espece: parts[0] || '',
        categorie: parts[1] || 'autre',
        nombre: parseInt(parts[2]) || 1,
        role: parts[3] || 'indicateur',
      };
    });

    const result = await invoke('resume_biodiversite', {
      input: {
        parcelle: str('bio-parcelle'),
        observations: observations,
      }
    });
    showResult('result-biodiversite', result);
  } catch (err) {
    showError('result-biodiversite', err);
  }
});

// ─── Init : charger la version ─────────────────────

window.addEventListener('DOMContentLoaded', async () => {
  try {
    const info = await invoke('version_info');
    document.getElementById('version-badge').textContent = 'v' + info.version;
  } catch (err) {
    console.log('Version info non disponible :', err);
  }
});
