use anyhow::{Result, Context};
use rusqlite::Connection;
use std::io::{self, Write};

/** -----------------------------
*  Bootstrap
*  -----------------------------
*  Manages database schema initialization.
*  */
pub fn bootstrap(db_path: &str, drop: bool, force: bool) -> Result<()> {
    println!("Bootstrapping SQLite database at: {}", db_path);

    let conn = Connection::open(db_path)
        .with_context(|| format!("Failed to open database {}", db_path))?;

    /* If force flag absent on drop */
    if drop && !force {
        /* Prompt for confirmation */
        println!("WARNING: This will DROP ALL DATA in '{}'", db_path);
        print!("Proceed? [y/N]: ");
        io::stdout().flush()?;

        /* Read confirmation input */
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let confirm = input.trim().to_lowercase();

        /* Check confirmation input - abort if not "y" or "yes" */
        if confirm != "y" && confirm != "yes" {
            println!("Aborted.");
            return Ok(());
        }
    }

    /* Drop branch */
    if drop {
        conn.execute_batch(
            r#"
            DROP TABLE IF EXISTS metrics;
            DROP INDEX IF EXISTS idx_metrics_tag;
            DROP INDEX IF EXISTS idx_metrics_ts;
            "#,
        )?;
    }

    /* Create schema */
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS metrics (
            id TEXT PRIMARY KEY,
            timestamp INTEGER NOT NULL,
            value REAL NOT NULL,
            tag TEXT NOT NULL,
            positive INTEGER NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_metrics_tag ON metrics(tag);
        CREATE INDEX IF NOT EXISTS idx_metrics_ts ON metrics(timestamp);
        "#,
    )?;

    println!("Database ready.");

    Ok(())
}
