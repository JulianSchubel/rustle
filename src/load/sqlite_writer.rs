use rusqlite::{Connection};
use std::thread;
use flume;

use crate::rustle::{transform};

pub fn spawn_writer(
    db_path: String,
    receiver: flume::Receiver<transform::TransformedRecord>,
) -> thread::JoinHandle<(usize, usize)> {
    thread::spawn( move || {
        /* Open a connection */
        let mut conn = Connection::open(db_path).unwrap();
        /* Create a transaction */
        let txn = conn.transaction().unwrap();

        /* Statement to be executed */
        let statement = "INSERT OR REPLACE INTO metrics \
                    (id, timestamp, value, tag, positive) \
                VALUES (?1, ?2, ?3, ?4, ?5)";

        let mut ok = 0usize;
        let mut fail = 0usize;

        for row in receiver {
            /* Prepare and execute the statement */
            match txn.execute(&statement, (
                &row.id,
                &row.timestamp,
                &row.value,
                &row.tag,
                &row.positive,
            )) {
                Ok(_) => ok += 1,
                Err(error) => {
                    eprintln!("Failed to load record!");
                    eprintln!(" Message: {error}");
                    eprintln!(" Row: {:?}", row);
                    fail += 1;
                },
            }
        }

        /* Commit the transaction */
        txn.commit().unwrap();
        (ok, fail)
    })
}
