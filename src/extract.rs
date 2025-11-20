mod csv_reader;
mod ndjson_reader;

use std::thread;
use serde::Deserialize;
use flume;

pub use csv_reader::spawn_csv_reader;
pub use ndjson_reader::spawn_ndjson_reader;

#[derive(Debug, Deserialize)]
pub struct RawRecord {
    pub id: String,
    pub timestamp: String,
    pub value: f64,
    pub tag: String,
}

/* helper function to spawn the appropriate reader based on file extension */
pub fn spawn_reader(path: &str, sender: flume::Sender<RawRecord>, csv_headers: bool) -> thread::JoinHandle<()> {
    let lower = path.to_lowercase();

    if lower.ends_with(".csv") {
        spawn_csv_reader(path.to_string(), sender, csv_headers)
    } else if lower.ends_with(".ndjson") || lower.ends_with(".jsonl") {
        spawn_ndjson_reader(path.to_string(), sender)
    } else {
        panic!("Unknown input format: expected .csv, .ndjson, or .jsonl");
    }
}
