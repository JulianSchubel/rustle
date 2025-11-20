use std::{thread};
use csv;
use flume;

/* Read CSV, deserialize, send RawRecord into channel */
pub fn spawn_csv_reader(path: String, sender: flume::Sender<super::RawRecord>, csv_headers: bool) -> thread::JoinHandle<()> {
        thread::spawn(move || {
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(csv_headers)
            .flexible(true)
            .trim(csv::Trim::All)
            .from_path(path)
            .unwrap();

        for result in rdr.deserialize::<super::RawRecord>() {
            match result {
                Ok(rec) => {
                    sender.send(rec).unwrap();
                }
                Err(e) => {
                    eprintln!("Failed to deserialize CSV entry!");
                    eprintln!(" Message: {e}");
                    eprintln!(" Raw line: {}", e.position().map(|p| p.line()).unwrap_or(0));
                }
            }
        }

        drop(sender);
    })
}
