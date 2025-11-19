use std::io::Write;
use anyhow::Result;
use rand::{Rng, seq::SliceRandom};
use chrono::Utc;
use serde::Serialize;

#[derive(Serialize)]
struct Record {
    id: String,
    timestamp: String,
    value: f64,
    tag: String,
}

pub fn generate_dataset(output: &str, records: usize) -> Result<()> {
    println!("Generating {} records -> {}", records, output);

    let file = std::fs::File::create(output)?;
    let mut rng = rand::thread_rng();
    
    /* Generate n random records */
    for i in 0..records {
        let rec = Record {
            id: format!("id-{i}"),
            timestamp: Utc::now()
                .to_string(),
            value: rng.gen_range(-100.0..100.0),
            tag: ["alpha", "beta", "gamma"]
                .choose(&mut rng).unwrap()
                .to_string(),
        };

        serde_json::to_writer(&file, &rec)?;
        writeln!(&file)?;
    }

    println!("Done.");
    Ok(())
}
