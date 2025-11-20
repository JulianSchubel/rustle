use anyhow::{Result};
use flume;
use rusqlite::{params_from_iter, Connection, Transaction};
use crate::rustle::{transform};
use std::thread;

pub fn spawn_batch_writer(
    db_path: String, 
    receiver: flume::Receiver<transform::TransformedRecord>, 
    batch_size: usize
) -> thread::JoinHandle<Result<(usize, usize)>> {
    std::thread::spawn(move || {
        /* Open a connection */
        let mut conn = Connection::open(db_path).unwrap();
        configure_connection(&conn).unwrap();

        /* Buffer to hold batches */
        let mut buffer: Vec<transform::TransformedRecord> = Vec::with_capacity(batch_size);

        let mut ok = 0usize;
        let mut fail = 0usize;

        /* Start a transaction */
        let txn = conn.transaction()?;
        /* push records to the buffer */
        while let Ok(record) = receiver.recv() {
            buffer.push(record);

            if buffer.len() >= batch_size {
                match insert_batch(&txn, &buffer) {
                    Ok(_) => ok += buffer.len(),
                    Err(error) => {
                        eprintln!("Batch insert failed: {error}");
                        fail += buffer.len();
                    }
                }
                buffer.clear();
            }
        }

        /* Flush last partial batch */
        if !buffer.is_empty() {
            match insert_batch(&txn, &buffer) {
                Ok(_) => ok += buffer.len(),
                Err(error) => {
                    eprintln!("Final batch insert failed: {error}");
                    fail += buffer.len();
                }
            }
        }

        /* Close the transaction */
        txn.commit()?;

        Ok((ok, fail))
    })
}

/* Constructs and inserts a full or partial batch */
fn insert_batch(txn: &Transaction, batch: &[transform::TransformedRecord]) -> Result<()> {
    /* skip empty batches */
    if batch.is_empty() {
        return Ok(());
    }

    /* Build SQL statement */
    let mut sql = String::from(
        "INSERT INTO metrics (id, timestamp, value, tag, positive) VALUES "
    );

    for i in 0..batch.len() {
        sql.push_str("(?, ?, ?, ?, ?)");
        if i < batch.len() - 1 {
            sql.push_str(", ");
        }
    }
    sql.push(';');

    let mut statement = txn.prepare_cached(&sql)?;

    /* Flatten parameters as SQLite receives a one dimensional parameter array */
    let params = params_from_iter(
        batch.iter().flat_map(|record| {
            [
                &record.id as &dyn rusqlite::ToSql,
                &record.timestamp,
                &record.value,
                &record.tag,
                &record.positive,
            ]
        })
    );

    statement.execute(params)?;
    Ok(())
}

fn configure_connection(conn: &Connection) -> rusqlite::Result<()> {
    /* Set Write-Ahead Logging mode 
    * ∙ Changes written to WAL file.
    * ∙ Commit performance increased as pages are appended to instead of re-written
    * */
    conn.pragma_update(None, "journal_mode", &"WAL")?;
    /* Set how aggressively SQLite forces data to disk during writes
    * ∙ Do not want to comprise durability, but want improved performance
    * ∙ NORMAL mode is the middle ground
    * */
    conn.pragma_update(None, "synchronous", &"NORMAL")?;
    conn.pragma_update(None, "temp_store", &"MEMORY")?;
    conn.pragma_update(None, "mmap_size", & (256 * 1024 * 1024))?;
    Ok(())
}
