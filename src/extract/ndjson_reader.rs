/// Read NDJSON lines, deserialize, and send RawRecord into channel
use flume;
use std::{fs::File, io::{BufRead, BufReader}, thread};

pub fn spawn_ndjson_reader(path: String, sender: flume::Sender<super::RawRecord>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let file = File::open(&path).expect("failed to open NDJSON file");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(text) = line {
                if text.trim().is_empty() {
                    continue;
                }

                match serde_json::from_str::<super::RawRecord>(&text) {
                    Ok(rec) => {
                        sender.send(rec).unwrap();
                    }
                    Err(err) => {
                        eprintln!("Invalid NDJSON line: {err}");
                    }
                }
            }
        }

        drop(sender);
    })
}
