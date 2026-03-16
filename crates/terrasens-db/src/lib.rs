//! # TERRASENS DB
//!
//! Couche base de données SQLite locale.
//! Zéro cloud. Les données restent sur l'appareil de l'agriculteur.

use rusqlite::Connection;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DbError {
    #[error("Erreur SQLite : {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("Migration échouée : {0}")]
    Migration(String),
}

pub type DbResult<T> = Result<T, DbError>;

/// Base de données locale TERRASENS
pub struct TerraDb {
    conn: Connection,
}

impl TerraDb {
    /// Ouvre ou crée la base de données locale
    pub fn ouvrir(path: &Path) -> DbResult<Self> {
        let conn = Connection::open(path)?;

        // Pragmas de sécurité et performance
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA synchronous = NORMAL;
             PRAGMA foreign_keys = ON;
             PRAGMA cache_size = -64000;
             PRAGMA busy_timeout = 5000;",
        )?;

        let db = Self { conn };
        db.migrer()?;
        Ok(db)
    }

    /// Base en mémoire (pour les tests)
    pub fn en_memoire() -> DbResult<Self> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch("PRAGMA foreign_keys = ON;")?;
        let db = Self { conn };
        db.migrer()?;
        Ok(db)
    }

    /// Exécute les migrations
    fn migrer(&self) -> DbResult<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS migrations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                version TEXT NOT NULL UNIQUE,
                applied_at TEXT NOT NULL DEFAULT (datetime('now'))
            );

            CREATE TABLE IF NOT EXISTS parcelles (
                id TEXT PRIMARY KEY,
                nom TEXT NOT NULL,
                superficie_hectares REAL NOT NULL,
                latitude REAL NOT NULL,
                longitude REAL NOT NULL,
                altitude_metres REAL DEFAULT 0,
                proprietaire_id TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now'))
            );

            CREATE TABLE IF NOT EXISTS analyses_sol (
                id TEXT PRIMARY KEY,
                parcelle_id TEXT NOT NULL REFERENCES parcelles(id),
                date_prelevement TEXT NOT NULL,
                date_analyse TEXT NOT NULL,
                profondeur_cm INTEGER DEFAULT 30,
                ph REAL,
                matiere_organique REAL,
                azote_total REAL,
                phosphore_olsen REAL,
                potassium_echangeable REAL,
                calcium REAL,
                magnesium REAL,
                cec REAL,
                rapport_cn REAL,
                argile REAL,
                limon REAL,
                sable REAL,
                source TEXT NOT NULL DEFAULT 'manuel',
                laboratoire TEXT,
                notes TEXT,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            );

            CREATE INDEX IF NOT EXISTS idx_analyses_parcelle
                ON analyses_sol(parcelle_id);
            CREATE INDEX IF NOT EXISTS idx_analyses_date
                ON analyses_sol(date_prelevement);

            CREATE TABLE IF NOT EXISTS agriculteurs (
                id TEXT PRIMARY KEY,
                nom TEXT NOT NULL,
                prenom TEXT NOT NULL,
                exploitation TEXT,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            );",
        )?;

        Ok(())
    }

    /// Accès direct à la connexion (pour les repositories)
    pub fn conn(&self) -> &Connection {
        &self.conn
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation_db_memoire() {
        let db = TerraDb::en_memoire().unwrap();
        // Vérifier que les tables existent
        let count: i32 = db
            .conn()
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='parcelles'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_tables_analyses_sol() {
        let db = TerraDb::en_memoire().unwrap();
        let count: i32 = db
            .conn()
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='analyses_sol'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 1);
    }
}
