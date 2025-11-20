use uuid::{Uuid};
use std::io::Write;
use anyhow::Result;
use rand::{Rng, seq::SliceRandom};
use chrono::{Local};
use serde::Serialize;

#[derive(Serialize)]
struct Record {
    id: String,
    timestamp: String,
    value: f64,
    tag: String,
}

pub fn generate(output: &str, records: usize, format: &str) -> Result<()> {
    println!("Generating {} records -> {}", records, output);

    let file = std::fs::File::create(output)?;
    let mut rng = rand::thread_rng();
    
    /* Add headers for the CSV format */
    if format == "csv" {
        writeln!(&file, "id, timestamp, value, tag")?;
    }

    /* Generate n random records */
    for _ in 0..records {
        let id = Uuid::new_v4().to_string();
        let timestamp = Local::now()
            .to_rfc3339();
        let value =  rng.gen_range(-100.0..100.0);
        let tag = ["alpha", "beta", "gamma"]
                .choose(&mut rng).unwrap()
                .to_string();
        let rec = Record {
            id,
            timestamp,
            value,
            tag,
        };
        if format == "ndjson" { 
            serde_json::to_writer(&file, &rec)?;
            writeln!(&file)?;
        } else {
            writeln!(
                &file, 
                "{},{},{},{}", 
                rec.id,
                rec.timestamp,
                rec.value,
                rec.tag
            )?;
        };

    }

    println!("Done.");
    Ok(())
}
